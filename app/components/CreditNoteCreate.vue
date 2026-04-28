<script setup lang="ts">
import { commands, type InvoiceProductItem, type SelectInvoices } from "@/bindings";
import { Plus, Trash2, X } from "lucide-vue-next";
import { useFieldArray, useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const props = defineProps<{
  invoice: SelectInvoices;
  invoiceItems: InvoiceProductItem[];
}>();

const { t, n } = useI18n();
const { showErrorToast } = useCommandError();
const { close } = useModal();
const router = useRouter();
const isPosting = ref(false);

const creditNoteSchema = z.object({
  reason: z.string().optional(),
  items: z.array(
    z.object({
      product_id: z.string().min(1),
      quantity: z.number().min(1),
      price: z.number().min(0),
    })
  ).min(1),
});

type Item = z.infer<typeof creditNoteSchema>["items"][number];

const { handleSubmit, setFieldValue, values } = useForm({
  validationSchema: toTypedSchema(creditNoteSchema),
  initialValues: {
    reason: "",
    items: props.invoiceItems.map((item) => ({
      product_id: item.product_id,
      quantity: item.quantity,
      price: item.price,
    })),
  },
});

const { fields, remove, push } = useFieldArray<Item>("items");

const subtotal = computed(() =>
  (values.items ?? []).reduce((sum, item) => {
    return sum + Number(item.quantity ?? 0) * Number(item.price ?? 0);
  }, 0),
);

function formatMoney(value: number | string) {
  return n(toNumber(value), "currency");
}

const onSubmit = handleSubmit(async (formData) => {
  isPosting.value = true;
  const result = await commands.createCreditNote({
    invoice_id: props.invoice.id as string,
    reason: formData.reason || null,
    items: formData.items,
  });

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR CREATE CREDIT NOTE: ${JSON.stringify(result.error)}`);
    isPosting.value = false;
    return;
  }

  Logger.info(`CREATE CREDIT NOTE: ${JSON.stringify(result.data)}`);
  close();
  toast.success(t("messages.success.credit-note-created"), {
    action: {
      label: t("buttons.view"),
      onClick: () => {
        router.push(`/credit-notes/${result.data?.id}`);
      },
    },
  });
});

function addItem() {
  push({
    product_id: "",
    quantity: 1,
    price: 0,
  });
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4">
    <div class="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
      <div class="sticky top-0 bg-white border-b flex items-center justify-between p-4">
        <h2 class="text-xl font-semibold">{{ t("buttons.create-credit-note") }}</h2>
        <button @click="close" class="p-1 hover:bg-gray-100 rounded">
          <X :size="20" />
        </button>
      </div>

      <form @submit="onSubmit" class="p-6 space-y-6">
        <div class="bg-gray-50 p-4 rounded-lg space-y-2">
          <p class="text-sm text-gray-600">
            {{ t("fields.invoice") }}: <span class="font-semibold">{{ invoice.identifier }}</span>
          </p>
          <p class="text-sm text-gray-600">
            {{ t("fields.total") }}: <span class="font-semibold">{{ formatMoney(invoice.total) }}</span>
          </p>
        </div>

        <div>
          <label class="block text-sm font-medium mb-2">{{ t("fields.reason") }}</label>
          <Field v-slot="{ field }" name="reason">
            <textarea
              v-bind="field"
              class="w-full px-3 py-2 border rounded-lg"
              rows="2"
              :placeholder="t('placeholders.reason')"
            />
          </Field>
        </div>

        <div>
          <div class="flex items-center justify-between mb-4">
            <h3 class="font-semibold">{{ t("fields.items") }}</h3>
            <button
              type="button"
              @click="addItem"
              class="flex items-center gap-2 px-3 py-1 text-sm bg-blue-50 text-blue-700 rounded hover:bg-blue-100"
            >
              <Plus :size="16" />
              {{ t("buttons.add-product") }}
            </button>
          </div>

          <div class="space-y-3">
            <div v-for="(field, idx) in fields" :key="field.key" class="grid grid-cols-12 gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="col-span-5">
                <label class="text-xs font-medium text-gray-600">{{ t("fields.product") }}</label>
                <Field v-slot="{ field: f }" :name="`items.${idx}.product_id`">
                  <input
                    v-bind="f"
                    disabled
                    class="w-full px-2 py-1 border rounded text-sm bg-gray-100"
                    :value="invoiceItems[idx]?.product_name || ''"
                  />
                </Field>
              </div>

              <div class="col-span-2">
                <label class="text-xs font-medium text-gray-600">{{ t("fields.quantity") }}</label>
                <Field v-slot="{ field: f }" :name="`items.${idx}.quantity`" type="number">
                  <input
                    v-bind="f"
                    type="number"
                    min="0"
                    class="w-full px-2 py-1 border rounded text-sm"
                  />
                </Field>
              </div>

              <div class="col-span-3">
                <label class="text-xs font-medium text-gray-600">{{ t("fields.price") }}</label>
                <Field v-slot="{ field: f }" :name="`items.${idx}.price`" type="number">
                  <input
                    v-bind="f"
                    type="number"
                    min="0"
                    step="0.01"
                    class="w-full px-2 py-1 border rounded text-sm"
                  />
                </Field>
              </div>

              <div class="col-span-2 flex items-end">
                <button
                  type="button"
                  @click="remove(idx)"
                  class="w-full px-2 py-1 text-red-600 hover:bg-red-50 rounded transition"
                >
                  <Trash2 :size="16" class="mx-auto" />
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="border-t pt-4 space-y-3">
          <div class="flex justify-between text-lg font-semibold">
            <span>{{ t("fields.subtotal") }}</span>
            <span>{{ formatMoney(subtotal) }}</span>
          </div>
        </div>

        <div class="flex gap-3 justify-end">
          <button
            type="button"
            @click="close"
            class="px-4 py-2 border rounded-lg hover:bg-gray-50"
          >
            {{ t("buttons.cancel") }}
          </button>
          <button
            type="submit"
            :disabled="isPosting"
            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            {{ isPosting ? t("buttons.creating") : t("buttons.create") }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
