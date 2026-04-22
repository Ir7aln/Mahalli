<script setup lang="ts">
import {
  commands,
  type AddInvoicePayment,
  type SelectInvoicePayment,
} from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { CalendarDays, WalletCards, X } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { useForm } from "vee-validate";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";

const props = defineProps<{
  id: string;
  identifier: string;
}>();

const { t, d, n } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const isPosting = ref(false);
const isLoading = ref(true);
const totalAmount = ref(0);
const paidAmount = ref(0);
const payments = ref<SelectInvoicePayment[]>([]);

const paymentSchema = z.object({
  payment_date: z.string().min(1),
  description: z.string().optional(),
  amount: z.coerce.number().positive(),
});

const { handleSubmit, resetForm, values } = useForm({
  validationSchema: toTypedSchema(paymentSchema),
  initialValues: {
    payment_date: new Date().toISOString().slice(0, 10),
    description: "",
    amount: 0,
  },
});

const unpaidAmount = computed(() => Math.max(totalAmount.value - paidAmount.value, 0));

function formatMoney(value: number | string) {
  return n(toNumber(value), "currency");
}

async function loadInvoice() {
  isLoading.value = true;

  try {
    const result = await commands.getInvoice(props.id);
    if (result.status === "error") throw result.error;

    const invoice = result.data.data;
    totalAmount.value = Number(invoice?.total ?? 0);
    paidAmount.value = Number(invoice?.paid_amount ?? 0);
    payments.value = invoice?.payments ?? [];
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR GET INVOICE FOR PAYMENT: ${err.error ? err.error : err.message}`);
    close();
  } finally {
    isLoading.value = false;
  }
}

await loadInvoice();

const onSubmit = handleSubmit(async (formValues) => {
  if (Number(formValues.amount ?? 0) > unpaidAmount.value) {
    toast.error(t("notifications.invoice.payment-too-high"), {
      description: t("notifications.invoice.payment-too-high-description", {
        amount: formatMoney(unpaidAmount.value),
      }),
      closeButton: true,
    });
    return;
  }

  isPosting.value = true;

  try {
    const payload: AddInvoicePayment = {
      invoice_id: props.id,
      payment_date: formValues.payment_date,
      description: formValues.description?.trim() || null,
      amount: formValues.amount,
    };
    const result = await commands.addInvoicePayment(payload);
    if (result.status === "error") throw result.error;

    toast.success(t("notifications.invoice.payment-added"), {
      closeButton: true,
    });

    await loadInvoice();
    resetForm({
      values: {
        payment_date: new Date().toISOString().slice(0, 10),
        description: "",
        amount: 0,
      },
    });

    updateQueryParams({
      refresh: `refresh-payment-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: err.error ?? err.message ?? t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR ADD INVOICE PAYMENT: ${err.error ? err.error : err.message}`);
  } finally {
    isPosting.value = false;
  }
});
</script>

<template>
  <form class="h-full w-full max-w-[720px]" @submit="onSubmit">
    <div
      class="flex h-full w-full flex-col overflow-hidden border-s border-slate-200 bg-white text-slate-900 shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-slate-200 px-5 py-4">
        <div class="space-y-1">
          <p class="text-xs font-medium uppercase tracking-[0.35em] text-slate-500">
            {{ t("routes.invoices") }}
          </p>
          <h2 class="text-xl font-semibold">
            {{ t("titles.invoices.add-payment") }} {{ props.identifier }}
          </h2>
        </div>
        <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
          <X class="size-5" />
        </Button>
      </div>

      <div class="min-h-0 flex-1 overflow-y-auto px-5 py-6 sm:px-6">
        <section class="space-y-6 border border-slate-200 bg-white px-6 py-6 sm:px-7">
          <div class="grid gap-4 sm:grid-cols-3">
            <div class="border border-slate-200 px-4 py-4">
              <p class="text-xs uppercase tracking-[0.3em] text-slate-500">
                {{ t("fields.total") }}
              </p>
              <p class="mt-2 text-2xl font-semibold">
                {{ formatMoney(totalAmount) }}
              </p>
            </div>
            <div class="border border-slate-200 px-4 py-4">
              <p class="text-xs uppercase tracking-[0.3em] text-slate-500">
                {{ t("fields.paid") }}
              </p>
              <p class="mt-2 text-2xl font-semibold">
                {{ formatMoney(paidAmount) }}
              </p>
            </div>
            <div class="border border-slate-200 px-4 py-4">
              <p class="text-xs uppercase tracking-[0.3em] text-slate-500">
                {{ t("fields.balance") }}
              </p>
              <p class="mt-2 text-2xl font-semibold">
                {{ formatMoney(unpaidAmount) }}
              </p>
            </div>
          </div>

          <div class="grid gap-4 sm:grid-cols-2">
            <div class="space-y-2">
              <div class="inline-flex items-center gap-2 text-sm text-slate-500">
                <CalendarDays class="size-4" />
                {{ t("fields.date") }}
              </div>
              <FormField v-slot="{ componentField }" name="payment_date">
                <FormItem>
                  <FormControl>
                    <Input v-bind="componentField" type="date" />
                  </FormControl>
                </FormItem>
              </FormField>
            </div>

            <div class="space-y-2">
              <div class="inline-flex items-center gap-2 text-sm text-slate-500">
                <WalletCards class="size-4" />
                {{ t("fields.amount") }}
              </div>
              <FormField v-slot="{ componentField }" name="amount">
                <FormItem>
                  <FormControl>
                    <Input
                      v-bind="componentField"
                      type="number"
                      step="0.01"
                      min="0"
                      :max="unpaidAmount"
                    >
                      <template #unite>
                        {{ t("fields.currency") }}
                      </template>
                    </Input>
                  </FormControl>
                </FormItem>
              </FormField>
            </div>
          </div>

          <div class="space-y-2">
            <FormField v-slot="{ componentField }" name="description">
              <FormItem>
                <FormLabel>{{ t("fields.description") }}</FormLabel>
                <FormControl>
                  <Textarea
                    v-bind="componentField"
                    :placeholder="t('placeholders.payment-description')"
                    class="min-h-24"
                  />
                </FormControl>
              </FormItem>
            </FormField>
          </div>

          <div class="border-t border-slate-200 pt-4">
            <div class="flex items-center justify-between gap-4 pb-4">
              <p class="text-xs font-semibold uppercase tracking-[0.3em] text-slate-500">
                {{ t("fields.payments") }}
              </p>
              <span class="text-sm text-slate-500">{{ payments.length }}</span>
            </div>

            <div v-if="payments.length" class="overflow-hidden border border-slate-200">
              <div
                class="grid grid-cols-[160px_minmax(0,1fr)_140px] bg-slate-50 px-4 py-3 text-xs font-semibold uppercase tracking-[0.3em] text-slate-500"
              >
                <span>{{ t("fields.date") }}</span>
                <span>{{ t("fields.description") }}</span>
                <span class="text-right">{{ t("fields.amount") }}</span>
              </div>
              <div class="divide-y divide-slate-200">
                <div
                  v-for="payment in payments"
                  :key="payment.id"
                  class="grid grid-cols-[160px_minmax(0,1fr)_140px] gap-3 px-4 py-3 text-sm"
                >
                  <span>{{ d(new Date(payment.payment_date), "short") }}</span>
                  <span class="truncate text-slate-500">{{ payment.description || "--" }}</span>
                  <span class="text-right font-medium">{{ formatMoney(payment.amount) }}</span>
                </div>
              </div>
            </div>
            <div
              v-else
              class="border border-dashed border-slate-200 px-4 py-6 text-sm text-slate-500"
            >
              {{ t("placeholders.no-payments") }}
            </div>
          </div>
        </section>
      </div>

      <div class="border-t border-slate-200 px-5 py-4">
        <div class="flex items-center justify-end gap-3">
          <Button type="button" variant="outline" @click="close">
            {{ t("buttons.cancel") }}
          </Button>
          <Button
            type="submit"
            :disabled="isPosting || isLoading || unpaidAmount <= 0 || !values.payment_date"
          >
            {{ t("buttons.add-payment") }}
          </Button>
        </div>
      </div>
    </div>
  </form>
</template>
