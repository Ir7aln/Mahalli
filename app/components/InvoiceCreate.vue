<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Trash2 } from "lucide-vue-next";
import { useFieldArray, useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const clients = ref<{ label: string; value: string }[]>([]);
const products = ref<{ label: string; value: string }[]>([]);
const isPosting = ref<boolean>(false);

const invoiceSchema = z.object({
  client_id: z.string().min(1),
  paid_amount: z.number().min(0),
  items: z.array(
    z.object({
      product_id: z.string().min(1),
      quantity: z.number().min(1),
      price: z.number().min(1),
    })
  ),
});

const { handleSubmit, setFieldValue } = useForm({
  validationSchema: toTypedSchema(invoiceSchema),
  initialValues: {
    client_id: "",
    paid_amount: 0.0,
    items: [
      {
        product_id: "",
        quantity: 1,
        price: 0,
      },
    ],
  },
});

type item = z.infer<typeof invoiceSchema>["items"][number];

const { fields, remove, push } = useFieldArray<item>("items");

function addInvoiceItem() {
  push({
    product_id: "",
    quantity: 1,
    price: 0,
  });
}

function deleteInvoiceItem(index: number) {
  remove(index);
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
    await invoke<Res<string>>("create_invoice", {
      invoice: {
        client_id: values.client_id,
        status: "DRAFT",
        paid_amount: values.paid_amount,
        items: values.items,
      },
    });
    //
    Logger.info(`CREATE INVOICE: ${JSON.stringify(values)}`);
    //
    toast.success(t("notifications.invoice.created"), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-create-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR CREATE INVOICE: ${err.error ? err.error : err.message}`
    );
  } finally {
    isPosting.value = false;
    close();
  }
});
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle>{{ t("titles.invoices.create") }}</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="h-full w-full grid grid-cols-1 gap-2">
          <div class="flex w-full flex-col h-fit gap-1">
            <div class="w-full h-full flex flex-col gap-1">
              <FormField v-slot="{ field }" name="client_id">
                <FormItem>
                  <FormLabel>{{ t("fields.full-name") }}</FormLabel>
                  <FormControl>
                    <SearchList
                      :items="clients"
                      @update-items="searchClients"
                      @on-select="field.onChange"
                    />
                  </FormControl>
                </FormItem>
              </FormField>
            </div>
            <div class="w-full h-full flex flex-col gap-1">
              <FormField v-slot="{ componentField }" name="paid_amount">
                <FormItem>
                  <FormLabel>
                    {{ t("fields.paid") }}
                  </FormLabel>
                  <FormControl>
                    <Input
                      v-bind="componentField"
                      placeholder=""
                      type="number"
                    />
                  </FormControl>
                </FormItem>
              </FormField>
            </div>
          </div>
          <Separator />
          <div class="w-full h-full flex flex-col gap-1">
            <Button type="button" @click="addInvoiceItem">
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
                    <FormItem>
                      <FormControl>
                        <SearchList
                          :items="products"
                          @update-items="searchProducts"
                          @on-select="
                            (id, price) => {
                              productField.onChange(id);
                              setFieldValue(`items.${index}.price`, price!);
                            }
                          "
                        />
                      </FormControl>
                    </FormItem>
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
                    @click="deleteInvoiceItem(index)"
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
          {{ t("buttons.add") }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
