<script setup lang="ts">
import { commands } from "@/bindings";
import type { DeliveryNoteDetailsResponse } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

definePageMeta({
  layout: "default",
});

const route = useRoute();
const localePath = useLocalePath();
const { t, n, d, locale } = useI18n();
const { showErrorToast } = useCommandError();

const deliveryNoteId = computed(() => route.params.id as string);

async function fetchDeliveryNote() {
  const result = await commands.getDeliveryNote(deliveryNoteId.value);

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR FETCH DELIVERY NOTE: ${JSON.stringify(result.error)}`);
    return null;
  }

  return result.data.data;
}

const { data: deliveryNote } = await useAsyncData<DeliveryNoteDetailsResponse | null>(
  () => fetchDeliveryNote(),
  { watch: [deliveryNoteId] },
);

function formatMoney(value: number) {
  return n(value, "currency");
}
</script>

<template>
  <main class="h-full w-full overflow-auto p-6">
    <div v-if="!deliveryNote" class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-b-2 border-slate-900" />
    </div>

    <section
      v-else
      class="mx-auto max-w-5xl space-y-6 rounded-xl border border-slate-200 bg-white p-8 shadow-sm"
    >
      <div
        class="flex flex-col gap-4 border-b border-slate-200 pb-6 sm:flex-row sm:items-start sm:justify-between"
      >
        <div>
          <p class="text-sm font-medium uppercase tracking-wide text-slate-500">
            {{ t("routes.delivery-notes") }}
          </p>
          <h1 class="mt-1 text-3xl font-bold text-slate-950">
            {{ deliveryNote.identifier }}
          </h1>
          <p class="mt-2 text-sm text-slate-500">
            {{ d(new Date(deliveryNote.created_at), "long") }}
          </p>
        </div>
        <div class="text-left sm:text-right rtl:sm:text-left">
          <p class="text-sm text-slate-500">{{ t("fields.order") }}</p>
          <NuxtLink
            :to="localePath(`/orders/?page=1&search=${deliveryNote.order_identifier ?? ''}`)"
            class="font-semibold text-slate-900 underline decoration-slate-300 underline-offset-4"
          >
            {{ deliveryNote.order_identifier }}
          </NuxtLink>
        </div>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <div class="rounded-lg border border-slate-200 bg-slate-50 p-4">
          <p class="text-xs font-semibold uppercase tracking-wide text-slate-500">
            {{ t("fields.bill-to") }}
          </p>
          <h2 class="mt-2 text-lg font-semibold text-slate-950">
            {{ deliveryNote.client.full_name }}
          </h2>
          <p v-if="deliveryNote.client.email" class="text-sm text-slate-600">
            {{ deliveryNote.client.email }}
          </p>
          <p v-if="deliveryNote.client.phone_number" class="text-sm text-slate-600">
            {{ deliveryNote.client.phone_number }}
          </p>
          <p v-if="deliveryNote.client.address" class="text-sm text-slate-600">
            {{ deliveryNote.client.address }}
          </p>
        </div>

        <div class="rounded-lg border border-slate-200 bg-slate-50 p-4">
          <p class="text-xs font-semibold uppercase tracking-wide text-slate-500">
            {{ t("fields.legal-identifiers") }}
          </p>
          <dl class="mt-2 grid grid-cols-2 gap-2 text-sm">
            <dt class="text-slate-500">{{ t("fields.ice") }}</dt>
            <dd class="font-medium text-slate-900">{{ deliveryNote.client.ice || "-" }}</dd>
            <dt class="text-slate-500">{{ t("fields.if-number") }}</dt>
            <dd class="font-medium text-slate-900">{{ deliveryNote.client.if_number || "-" }}</dd>
            <dt class="text-slate-500">{{ t("fields.rc") }}</dt>
            <dd class="font-medium text-slate-900">{{ deliveryNote.client.rc || "-" }}</dd>
            <dt class="text-slate-500">{{ t("fields.patente") }}</dt>
            <dd class="font-medium text-slate-900">{{ deliveryNote.client.patente || "-" }}</dd>
          </dl>
        </div>
      </div>

      <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
        <TableHeader>
          <TableRow>
            <TableHead>{{ t("fields.product") }}</TableHead>
            <TableHead>{{ t("fields.quantity") }}</TableHead>
            <TableHead>{{ t("fields.price") }}</TableHead>
            <TableHead>{{ t("fields.subtotal") }}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="item in deliveryNote.items" :key="item.product_id">
            <TableCell class="font-medium">{{ item.name }}</TableCell>
            <TableCell>{{ item.quantity }}</TableCell>
            <TableCell>{{ formatMoney(item.price) }}</TableCell>
            <TableCell>{{ formatMoney(item.price * item.quantity) }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>

      <div class="flex justify-end border-t border-slate-200 pt-4">
        <div class="w-full max-w-sm space-y-2">
          <div class="flex items-center justify-between text-lg font-bold text-slate-950">
            <span>{{ t("fields.total") }}</span>
            <span>{{ formatMoney(deliveryNote.total) }}</span>
          </div>
        </div>
      </div>
    </section>
  </main>
</template>
