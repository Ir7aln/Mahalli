<script setup lang="ts">
import { commands } from "@/bindings";
import type { CreditNoteResponse } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

definePageMeta({
  layout: "default",
});

const route = useRoute();
const { t, n, d } = useI18n();
const { showErrorToast } = useCommandError();

const creditNoteId = computed(() => route.params.id as string);

async function fetchCreditNote() {
  const result = await commands.getCreditNote(creditNoteId.value);

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR FETCH CREDIT NOTE: ${JSON.stringify(result.error)}`);
    return null;
  }

  return result.data.data;
}

const { data: creditNote } = await useAsyncData(
  () => fetchCreditNote(),
  { watch: [creditNoteId] }
);

function formatMoney(value: number) {
  return n(value, "currency");
}
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-3xl font-bold">{{ t("sidebar.credit-notes") }}</h1>
    </div>

    <div v-if="!creditNote" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-slate-900"></div>
    </div>

    <div v-else class="space-y-4">
      <div class="bg-white border border-slate-200 rounded-lg p-6">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <p class="text-sm text-slate-600">{{ t("fields.identifier") }}</p>
            <p class="text-lg font-semibold text-slate-900">{{ creditNote.identifier }}</p>
          </div>
          <div>
            <p class="text-sm text-slate-600">{{ t("fields.date") }}</p>
            <p class="text-lg font-semibold text-slate-900">{{ d(new Date(creditNote.created_at), "long") }}</p>
          </div>
        </div>
        <div v-if="creditNote.reason" class="mt-4 pt-4 border-t border-slate-200">
          <p class="text-sm text-slate-600">{{ t("fields.reason") }}</p>
          <p class="text-slate-900 mt-1">{{ creditNote.reason }}</p>
        </div>
      </div>

      <div class="bg-white border border-slate-200 rounded-lg p-6">
        <div class="flex justify-between items-center mb-4">
          <h3 class="font-semibold text-slate-900">{{ t("fields.total") }}</h3>
          <p class="text-2xl font-bold text-slate-900">{{ formatMoney(creditNote.total) }}</p>
        </div>
      </div>
    </div>
  </div>
</template>
