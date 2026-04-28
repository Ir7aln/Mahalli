<script setup lang="ts">
import { commands } from "@/bindings";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { ColumnVisibilityDropdown } from "#components";
import type { CreditNoteResponse } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();

const LIMIT = 50;
const searchQuery = ref(queryString(route.query.search));
const creditNoteTableColumns = [
  { key: "identifier", label: t("fields.identifier") },
  { key: "full_name", label: t("fields.full-name") },
  { key: "invoice_identifier", label: t("fields.invoice") },
  { key: "reason", label: t("fields.reason") },
  { key: "created_at", label: t("fields.date") },
  { key: "total", label: t("fields.total") },
];
const visibleColumns = ref<string[]>(creditNoteTableColumns.map((col) => col.key));

const queryParams = computed(() => ({
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  search: queryString(route.query.search),
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchCreditNotes() {
  const result = await commands.listCreditNotes({
    limit: queryParams.value.limit,
    offset: (queryParams.value.page - 1) * queryParams.value.limit,
    search: queryParams.value.search,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR FETCH CREDIT NOTES: ${JSON.stringify(result.error)}`);
    return null;
  }

  return result.data.data;
}

const { data: creditNotesData } = await useAsyncData(fetchCreditNotes, {
  watch: [queryParams],
});

const creditNotes = computed<CreditNoteResponse[]>(() => creditNotesData.value?.notes ?? []);
const totalRows = computed<number>(() => creditNotesData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "", page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

onMounted(async () => {
  const preferences = await commands.getColumnPreferences("credit-notes");
  if (preferences.status === "ok" && preferences.data?.data?.visible_columns) {
    visibleColumns.value = preferences.data.data.visible_columns;
  }
});

watch(
  visibleColumns,
  async (newColumns) => {
    await commands.saveColumnPreferences({
      page: "credit-notes",
      visible_columns: newColumns,
    });
  },
  { deep: true },
);
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start">
      <ListFilterBar
        :search="searchQuery"
        :active-filters="[]"
        @update:search="(value) => (searchQuery = value)"
      >
        <template #columns>
          <ColumnVisibilityDropdown
            :columns="creditNoteTableColumns"
            :visible-columns="visibleColumns"
            @update:visible-columns="(cols) => (visibleColumns = cols)"
          />
        </template>
      </ListFilterBar>

      <CreditNotesTable :credit-notes="creditNotes" :visible-columns="visibleColumns" />
    </div>
  </main>
</template>
