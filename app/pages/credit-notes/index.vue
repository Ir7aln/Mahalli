<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

definePageMeta({
  layout: "default",
});

const route = useRoute();
const { t } = useI18n();
const { showErrorToast } = useCommandError();

const creditNotes = ref([]);
const loading = ref(true);

async function fetchCreditNotes() {
  try {
    loading.value = true;
    const result = await commands.listCreditNotes({
      limit: 100,
      offset: 0,
    });

    if (result.status === "error") {
      showErrorToast(result.error);
      return;
    }

    creditNotes.value = result.data || [];
  } catch (error) {
    Logger.error(`Error fetching credit notes: ${error}`);
    showErrorToast({ message: "Failed to fetch credit notes" });
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  fetchCreditNotes();
});
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-3xl font-bold">{{ t("sidebar.credit-notes") }}</h1>
      <p class="text-gray-600 mt-1">{{ t("pages.credit-notes.description") }}</p>
    </div>

    <div v-if="loading" class="flex items-center justify-center py-12">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
    </div>

    <div v-else-if="creditNotes.length === 0" class="text-center py-12">
      <p class="text-gray-600">{{ t("tables.empty.description") }}</p>
    </div>

    <div v-else class="grid gap-4">
      <div v-for="note in creditNotes" :key="note.id" class="border rounded-lg p-4 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <h3 class="font-semibold">{{ note.identifier }}</h3>
            <p class="text-sm text-gray-600">{{ note.reason }}</p>
          </div>
          <NuxtLink
            :to="`/credit-notes/${note.id}`"
            class="px-3 py-1 bg-blue-50 text-blue-700 rounded text-sm hover:bg-blue-100"
          >
            {{ t("buttons.view") }}
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
