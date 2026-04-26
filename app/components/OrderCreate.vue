<script setup lang="ts">
import {
  commands,
  type ClientDetails,
  type ClientSearch,
  type NewOrder,
  type ProductSearch,
} from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { CalendarDays, Plus, Trash2, X } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { useFieldArray, useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";

interface ClientOption {
  label: string;
  value: string;
}

interface ProductOption {
  label: string;
  value: string;
  price?: number;
}

const { t, d, n } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const clients = ref<ClientOption[]>([]);
const products = ref<ProductOption[]>([]);
const selectedClient = ref<Partial<ClientDetails> | null>(null);
const isPosting = ref(false);

const orderSchema = z.object({
  client_id: z.string().min(1),
  items: z.array(
    z.object({
      product_id: z.string().min(1),
      quantity: z.number().min(1),
      price: z.number().min(1),
    }),
  ),
});

const { handleSubmit, setFieldValue, values } = useForm({
  validationSchema: toTypedSchema(orderSchema),
  initialValues: {
    client_id: "",
    items: [
      {
        product_id: "",
        quantity: 1,
        price: 0,
      },
    ],
  },
});

type Item = z.infer<typeof orderSchema>["items"][number];

const { fields, remove, push } = useFieldArray<Item>("items");

const subtotal = computed(() =>
  (values.items ?? []).reduce((sum, item) => {
    return sum + Number(item.quantity ?? 0) * Number(item.price ?? 0);
  }, 0),
);

const clientDetails = computed(() => [
  selectedClient.value?.email || t("placeholders.no-email"),
  selectedClient.value?.address || t("placeholders.no-address"),
  selectedClient.value?.phone_number || t("placeholders.no-phone"),
]);

function formatMoney(value: number | string) {
  return n(toNumber(value), "currency");
}

function addOrderItem() {
  push({
    product_id: "",
    quantity: 1,
    price: 0,
  });
}

function deleteOrderItem(index: number) {
  remove(index);
}

async function searchClients(search: string | number) {
  const result = await commands.searchClients(String(search));
  if (result.status === "ok") clients.value = (result.data.data ?? []) as ClientSearch[];
}

async function searchProducts(search: string | number) {
  const result = await commands.searchProducts(String(search));
  if (result.status === "ok") {
    products.value = ((result.data.data ?? []) as ProductSearch[]).map((p) => ({
      ...p,
      price: p.price ?? undefined,
    }));
  }
}

async function fillClientDetails(id: string) {
  try {
    const result = await commands.getClient(id);
    if (result.status === "ok" && result.data.data) {
      selectedClient.value = result.data.data;
    }
  } catch (err: any) {
    Logger.error(`ERROR GET CLIENT: ${err.error ? err.error : err.message}`);
  }
}

async function handleClientSelect(id: string) {
  setFieldValue("client_id", id);
  await fillClientDetails(id);
}

const onSubmit = handleSubmit(async (formValues) => {
  isPosting.value = true;
  try {
    const payload: NewOrder = {
      client_id: formValues.client_id,
      status: "PENDING",
      items: formValues.items,
    };
    const result = await commands.createOrder(payload);
    if (result.status === "error") throw result.error;

    Logger.info(`CREATE ORDER: ${JSON.stringify(formValues)}`);

    toast.success(t("notifications.order.created"), {
      closeButton: true,
    });

    updateQueryParams({
      refresh: `refresh-create-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR CREATE ORDER: ${err.error ? err.error : err.message}`);
  } finally {
    isPosting.value = false;
    close();
  }
});
</script>

<template>
  <form class="h-full w-full max-w-[860px]" @submit="onSubmit">
    <div
      class="flex h-full w-full flex-col overflow-hidden border-s border-slate-200 bg-white text-slate-900 shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-slate-200 px-5 py-4">
        <div class="space-y-1">
          <p class="text-xs font-medium text-slate-500">
            {{ t("routes.orders") }}
          </p>
          <h2 class="text-xl font-semibold">
            {{ t("titles.orders.create") }}
          </h2>
        </div>
        <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
          <X class="size-5" />
        </Button>
      </div>

      <div class="min-h-0 flex-1 overflow-y-auto">
        <div class="w-full h-full px-5 py-6 sm:px-6">
          <section class="border border-slate-200 bg-white px-6 py-6 sm:px-7">
            <div
              class="flex flex-col gap-6 border-b border-slate-200 pb-6 sm:flex-row sm:items-start sm:justify-between"
            >
              <div>
                <p class="text-3xl font-semibold tracking-tight">
                  {{ t("fields.order") }}
                </p>
                <p class="mt-2 text-sm text-slate-500">
                  {{ t("status.pending") }}
                </p>
              </div>
              <div class="text-left sm:text-right">
                <p class="text-xs text-slate-500">
                  {{ t("fields.total") }}
                </p>
                <p class="mt-2 text-3xl font-semibold tracking-tight">
                  {{ formatMoney(subtotal) }}
                </p>
              </div>
            </div>

            <div class="space-y-6 border-b border-slate-200 py-6">
              <div class="grid gap-4 text-sm sm:grid-cols-2">
                <div
                  class="flex items-center gap-4 border-b border-slate-100 pb-3 sm:border-b-0 sm:pb-0"
                >
                  <span class="text-slate-500">{{ t("fields.status") }}</span>
                  <Badge variant="outline" class="border-gray-300 bg-transparent text-gray-900">
                    {{ t("status.pending") }}
                  </Badge>
                </div>
                <div class="flex items-center gap-4">
                  <span class="inline-flex items-center gap-2 text-slate-500">
                    <CalendarDays class="size-4" />
                    {{ t("fields.date") }}
                  </span>
                  <span class="font-medium text-slate-900">
                    {{ d(new Date(), "long") }}
                  </span>
                </div>
              </div>

              <div class="space-y-4">
                <div class="space-y-2">
                  <p class="text-xs font-semibold text-slate-500">
                    {{ t("fields.bill-to") }}
                  </p>
                  <FormField v-slot="{ field }" name="client_id">
                    <FormItem>
                      <FormControl>
                        <SearchList
                          :items="clients"
                          @update-items="searchClients"
                          @on-select="
                            async (id) => {
                              field.onChange(id);
                              await handleClientSelect(id);
                            }
                          "
                        />
                      </FormControl>
                    </FormItem>
                  </FormField>
                </div>

                <div class="space-y-1 text-sm leading-6 text-slate-500">
                  <p class="text-base font-semibold text-slate-900">
                    {{ selectedClient?.full_name || t("placeholders.select-client") }}
                  </p>
                  <p v-for="detail in clientDetails" :key="detail">
                    {{ detail }}
                  </p>
                </div>
              </div>
            </div>

            <div class="py-6">
              <div class="flex items-center justify-between gap-4 pb-4">
                <p class="text-xs font-semibold text-slate-500">
                  {{ t("fields.items") }}
                </p>
                <Button
                  type="button"
                  variant="ghost"
                  class="gap-2 px-0 text-slate-700"
                  @click="addOrderItem"
                >
                  <Plus class="size-4" />
                  {{ t("buttons.add-product") }}
                </Button>
              </div>

              <div class="border-y border-slate-200">
                <div
                  class="hidden bg-slate-50 px-4 py-3 text-xs font-semibold text-slate-500 md:grid md:grid-cols-[minmax(0,2fr)_100px_140px_120px_40px] md:gap-3"
                >
                  <span>{{ t("fields.name") }}</span>
                  <span>{{ t("fields.quantity") }}</span>
                  <span>{{ t("fields.price") }}</span>
                  <span>{{ t("fields.total") }}</span>
                  <span />
                </div>

                <div class="divide-y divide-slate-200">
                  <div
                    v-for="(entry, index) in fields"
                    :key="entry.key"
                    class="grid gap-3 px-4 py-4 md:grid-cols-[minmax(0,2fr)_100px_140px_120px_40px] md:items-end"
                  >
                    <FormField
                      v-slot="{ field: productField }"
                      :name="`items[${index}].product_id`"
                    >
                      <FormItem>
                        <FormLabel class="md:hidden">
                          {{ t("fields.name") }}
                        </FormLabel>
                        <FormControl>
                          <SearchList
                            :items="products"
                            @update-items="searchProducts"
                            @on-select="
                              (id, price) => {
                                productField.onChange(id);
                                setFieldValue(`items.${index}.price`, Number(price ?? 0));
                              }
                            "
                          />
                        </FormControl>
                      </FormItem>
                    </FormField>

                    <FormField v-slot="{ componentField }" :name="`items[${index}].quantity`">
                      <FormItem>
                        <FormLabel class="md:hidden">
                          {{ t("fields.quantity") }}
                        </FormLabel>
                        <FormControl>
                          <Input v-bind="componentField" type="number" />
                        </FormControl>
                      </FormItem>
                    </FormField>

                    <FormField v-slot="{ componentField }" :name="`items[${index}].price`">
                      <FormItem>
                        <FormLabel class="md:hidden">
                          {{ t("fields.price") }}
                        </FormLabel>
                        <FormControl>
                          <Input v-bind="componentField" type="number">
                            <template #unite>
                              {{ t("fields.currency") }}
                            </template>
                          </Input>
                        </FormControl>
                      </FormItem>
                    </FormField>

                    <div class="space-y-1">
                      <p class="text-xs font-semibold text-slate-500 md:hidden">
                        {{ t("fields.total") }}
                      </p>
                      <div class="flex h-10 items-center text-sm font-medium text-slate-900">
                        {{
                          formatMoney(
                            Number(values.items?.[index]?.quantity ?? 0) *
                              Number(values.items?.[index]?.price ?? 0),
                          )
                        }}
                      </div>
                    </div>

                    <div class="flex items-center justify-end">
                      <Button
                        type="button"
                        variant="ghost"
                        size="icon"
                        class="h-9 w-9 rounded-full text-slate-500 hover:text-red-600"
                        @click="deleteOrderItem(index)"
                      >
                        <Trash2 class="size-4" />
                      </Button>
                    </div>
                  </div>
                </div>
              </div>

              <div class="ml-auto mt-6 w-full max-w-sm space-y-3 border-t border-slate-200 pt-4">
                <div class="flex items-center justify-between text-base font-semibold">
                  <span>{{ t("fields.total") }}</span>
                  <span>{{ formatMoney(subtotal) }}</span>
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>

      <div class="border-t border-slate-200 px-5 py-4">
        <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div class="text-sm text-slate-500">
            {{ selectedClient?.full_name || t("placeholders.no-client-selected") }}
          </div>
          <div class="flex items-center gap-3">
            <Button type="button" variant="outline" @click="close">
              {{ t("buttons.cancel") }}
            </Button>
            <Button type="submit" :disabled="isPosting">
              {{ t("buttons.add") }}
            </Button>
          </div>
        </div>
      </div>
    </div>
  </form>
</template>
