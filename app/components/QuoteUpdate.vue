<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Trash2 } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { useFieldArray, useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";

const props = defineProps<{
  id: string;
  identifier: string;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();
const clients = ref<{ label: string; value: string }[]>([]);
const products = ref<{ label: string; value: string }[]>([]);

const quoteSchema = z.object({
  id: z.string(),
  client_id: z.string().min(1),
  full_name: z.string(),
  items: z.array(
    z.object({
      id: z.string().optional(),
      product_id: z.string().min(1),
      quantity: z.number().min(1),
      price: z.number().min(1),
      name: z.string().optional(),
    })
  ),
});

const { handleSubmit, resetForm, setFieldValue, values } = useForm({
  validationSchema: toTypedSchema(quoteSchema),
  initialValues: {
    id: "",
    client_id: "",
    full_name: "",
    items: [],
  },
});

type item = z.infer<typeof quoteSchema>["items"][number];

const { fields, remove, push } = useFieldArray<item>("items");

const res = await invoke<Res<QuoteForUpdateT>>("get_quote", {
  id: props.id,
});

if (res.data) {
  resetForm({
    values: res.data,
  });
}

function addQuoteItem() {
  push({
    product_id: "",
    quantity: 1,
    price: 0,
  });
}

async function searchClients(search: string | number) {
  const res = await invoke<Res<{ label: string; value: string }[]>>(
    "search_clients",
    {
      search,
    }
  );
  if (!res.error) {
    clients.value = res.data;
  }
}

async function searchProducts(search: string | number) {
  const res = await invoke<Res<{ label: string; value: string }[]>>(
    "search_products",
    {
      search,
    }
  );
  if (!res.error) {
    products.value = res.data;
  }
}

const onSubmit = handleSubmit(async (values) => {
  try {
    await invoke<Res<string>>("update_quote", {
      quote: values,
    });
    Logger.info(`UPDATE QUOTE: ${JSON.stringify(values)}`);
    //
    toast.success(t("notifications.quote.updated"), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-update-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR UPDATE QUOTE: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
});

async function deleteOneQuoteItem(id: string) {
  try {
    await invoke("delete_quote_item", { id });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR DELETE QUOTE ITEM : ${err.error ? err.error : err.message}`
    );
  }
}

function deleteQuoteItem(index: number) {
  const item = values.items![index];
  if (item?.id) {
    deleteOneQuoteItem(item.id);
  }
  remove(index);
}
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle>
          {{ t("titles.quotes.update") }} N° {{ identifier }}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="h-full w-full grid grid-cols-1 gap-2">
          <div class="flex w-full h-fit gap-1">
            <div class="w-full h-full flex flex-col gap-1">
              <FormField v-slot="{ field }" name="client_id">
                <FormItem>
                  <FormLabel>{{ t("fields.full-name") }}</FormLabel>
                  <FormControl>
                    <SearchList
                      :default-value="values.full_name"
                      :items="clients"
                      @update-items="searchClients"
                      @on-select="field.onChange"
                    />
                  </FormControl>
                </FormItem>
              </FormField>
            </div>
          </div>
          <Separator />
          <div class="w-full h-full flex flex-col gap-1">
            <Button type="button" @click="addQuoteItem">
              {{ t("buttons.add-product") }}
            </Button>
            <ScrollArea :class="{ 'h-60': fields.length > 5 }">
              <div class="flex flex-col space-y-1 my-1">
                <div
                  v-for="(field, index) in fields"
                  :key="field.key"
                  class="grid grid-flow-col gap-1"
                >
                  <FormField
                    v-slot="{ field: productField }"
                    :name="`items[${index}].product_id`"
                  >
                    <SearchList
                      :default-value="field.value.name"
                      :items="products"
                      @update-items="searchProducts"
                      @on-select="
                        (id, price) => {
                          productField.onChange(id);
                          setFieldValue(`items.${index}.price`, price!);
                        }
                      "
                    />
                  </FormField>
                  <FormField
                    v-slot="{ componentField }"
                    :name="`items[${index}].quantity`"
                  >
                    <FormItem>
                      <FormControl>
                        <Input
                          :placeholder="t('fields.quantity')"
                          type="number"
                          v-bind="componentField"
                        >
                          <template #unite>
                            {{ t("fields.item") }}
                          </template>
                        </Input>
                      </FormControl>
                    </FormItem>
                  </FormField>
                  <FormField
                    v-slot="{ componentField }"
                    :name="`items[${index}].price`"
                  >
                    <FormItem>
                      <FormControl>
                        <Input
                          :placeholder="t('fields.price')"
                          type="number"
                          v-bind="componentField"
                        >
                          <template #unite>
                            DH
                          </template>
                        </Input>
                      </FormControl>
                    </FormItem>
                  </FormField>
                  <Trash2
                    class="cursor-pointer size-6 m-auto"
                    @click="deleteQuoteItem(index)"
                  />
                </div>
              </div>
            </ScrollArea>
          </div>
        </div>
      </CardContent>
      <CardFooter>
        <Button type="button" variant="outline" @click="close">
          {{ t("buttons.cancel") }}
        </Button>
        <Button type="submit" class="col-span-2">
          {{ t("buttons.confirme") }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
