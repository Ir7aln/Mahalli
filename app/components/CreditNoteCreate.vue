<script setup lang="ts">
import { commands } from "@/bindings";
import type { InvoiceWithClient } from "@/bindings";
import { Plus, Trash2, X } from "lucide-vue-next";
import { useFieldArray, useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { FormField, FormItem, FormControl, FormLabel } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";

const props = defineProps<{
  invoiceId: string;
  invoiceIdentifier: string;
}>();

const { t, n } = useI18n();
const { showErrorToast } = useCommandError();
const { close } = useModal();
const router = useRouter();
const isPosting = ref(false);
const loading = ref(true);
const invoiceDetails = ref<InvoiceWithClient | null>(null);

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

const { handleSubmit, resetForm, setFieldValue, values } = useForm({
  validationSchema: toTypedSchema(creditNoteSchema),
  initialValues: {
    reason: "",
    items: [],
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

const getResult = await commands.getInvoice(props.invoiceId);
if (getResult.status === "error") {
  Logger.error(`ERROR GET INVOICE: ${JSON.stringify(getResult.error)}`);
} else if (getResult.data.data) {
  const invoice = getResult.data.data as InvoiceWithClient;
  invoiceDetails.value = invoice;

  resetForm({
    values: {
      reason: "",
      items: (invoice.items ?? []).map((item) => ({
        product_id: item.product_id,
        quantity: item.quantity,
        price: item.price,
      })),
    },
  });
}
loading.value = false;

function addItem() {
  push({
    product_id: "",
    quantity: 1,
    price: 0,
  });
}

const onSubmit = handleSubmit(async (formData) => {
  isPosting.value = true;

  try {
    const result = await commands.createCreditNote({
      invoice_id: props.invoiceId,
      reason: formData.reason || null,
      items: formData.items,
    });

    if (result.status === "error") throw result.error;

    Logger.info(`CREATE CREDIT NOTE: ${JSON.stringify(result.data)}`);

    toast.success(t("notifications.credit-note.created"), {
      action: {
        label: t("buttons.view"),
        onClick: () => {
          router.push(`/credit-notes/${result.data?.id}`);
        },
      },
    });
  } catch (err: any) {
    Logger.error(`ERROR CREATE CREDIT NOTE: ${err.error ? err.error : err.message}`);
  } finally {
    isPosting.value = false;
    close();
  }
});
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="card-modal-shell w-4/6 lg:w-1/2">
      <div class="card-modal-header">
        <div class="card-modal-header-inner">
          <div class="space-y-1">
            <p class="card-modal-eyebrow">{{ t("routes.invoices") }}</p>
            <h2 class="card-modal-title">{{ t("buttons.create-credit-note") }}</h2>
            <p class="card-modal-description">{{ t("fields.invoice") }}: {{ invoiceIdentifier }}</p>
          </div>
          <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
            <X class="size-5" />
          </Button>
        </div>
      </div>

      <CardContent class="card-modal-body space-y-4">
        <FormField v-slot="{ componentField }" name="reason">
          <FormItem>
            <FormLabel>{{ t("fields.reason") }}</FormLabel>
            <FormControl>
              <Textarea
                v-bind="componentField"
                :placeholder="t('placeholders.reason')"
              />
            </FormControl>
          </FormItem>
        </FormField>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <h3 class="font-semibold text-sm">{{ t("fields.items") }}</h3>
            <Button
              type="button"
              variant="outline"
              size="sm"
              @click="addItem"
            >
              <Plus :size="16" class="mr-2" />
              {{ t("buttons.add-product") }}
            </Button>
          </div>

          <div class="space-y-3 border-t pt-4">
            <div v-for="(field, idx) in fields" :key="field.key" class="grid grid-cols-12 gap-3">
              <div class="col-span-6">
                <FormField v-slot="{ componentField }" :name="`items[${idx}].product_id`">
                  <FormItem>
                    <FormLabel class="text-xs">{{ t("fields.product") }}</FormLabel>
                    <FormControl>
                      <Input
                        v-bind="componentField"
                        disabled
                        :value="invoiceDetails?.items?.[idx]?.product_name || ''"
                        class="bg-slate-50"
                      />
                    </FormControl>
                  </FormItem>
                </FormField>
              </div>

              <div class="col-span-2">
                <FormField v-slot="{ componentField }" :name="`items[${idx}].quantity`">
                  <FormItem>
                    <FormLabel class="text-xs">{{ t("fields.quantity") }}</FormLabel>
                    <FormControl>
                      <Input v-bind="componentField" type="number" min="0" />
                    </FormControl>
                  </FormItem>
                </FormField>
              </div>

              <div class="col-span-3">
                <FormField v-slot="{ componentField }" :name="`items[${idx}].price`">
                  <FormItem>
                    <FormLabel class="text-xs">{{ t("fields.price") }}</FormLabel>
                    <FormControl>
                      <Input v-bind="componentField" type="number" min="0" step="0.01" />
                    </FormControl>
                  </FormItem>
                </FormField>
              </div>

              <div class="col-span-1 flex items-end justify-center">
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  @click="remove(idx)"
                  class="text-red-600 hover:text-red-700"
                >
                  <Trash2 :size="16" />
                </Button>
              </div>
            </div>
          </div>
        </div>

        <div class="border-t pt-4 space-y-2">
          <div class="flex justify-between font-semibold">
            <span>{{ t("fields.subtotal") }}</span>
            <span>{{ formatMoney(subtotal) }}</span>
          </div>
        </div>
      </CardContent>

      <div class="card-modal-footer">
        <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-end">
          <Button type="button" variant="outline" @click="close">
            {{ t("buttons.cancel") }}
          </Button>
          <Button type="submit" :disabled="isPosting">
            {{ isPosting ? t("buttons.creating") : t("buttons.create") }}
          </Button>
        </div>
      </div>
    </Card>
  </form>
</template>
