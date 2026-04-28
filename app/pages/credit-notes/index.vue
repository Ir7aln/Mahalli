<script setup lang="ts">
import { commands } from "@/bindings";
import type { CreditNoteResponse } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

definePageMeta({
  layout: "default",
});

const { t } = useI18n();
const { showErrorToast } = useCommandError();

async function fetchCreditNotes() {
  const result = await commands.listCreditNotes({
    limit: 100,
    offset: 0,
  });

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR FETCH CREDIT NOTES: ${JSON.stringify(result.error)}`);
    return null;
  }

  return result.data.data;
}

const { data: creditNotesData } = await useAsyncData(fetchCreditNotes);

const creditNotes = computed<CreditNoteResponse[]>(
  () => creditNotesData.value?.notes ?? []
);
const totalNotes = computed<number>(() => creditNotesData.value?.count ?? 0);
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-3xl font-bold">{{ t("sidebar.credit-notes") }}</h1>
    </div>

    <div v-if="!creditNotesData" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-slate-900"></div>
    </div>

    <div v-else-if="creditNotes.length === 0" class="text-center py-12">
      <p class="text-slate-600">{{ t("tables.empty.description") }}</p>
    </div>

    <div v-else class="grid gap-4">
      <div v-for="note in creditNotes" :key="note.id" class="border border-slate-200 rounded-lg p-4 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div class="flex-1">
            <h3 class="font-semibold text-slate-900">{{ note.identifier }}</h3>
            <p class="text-sm text-slate-600 mt-1">{{ note.reason || "-" }}</p>
          </div>
          <NuxtLink
            :to="`/credit-notes/${note.id}`"
            class="ml-4 px-3 py-1 text-sm font-medium text-slate-700 bg-slate-100 rounded hover:bg-slate-200 transition"
          >
            {{ t("buttons.view") }}
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
