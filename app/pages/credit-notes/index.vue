<script setup lang="ts">
import { commands } from "@/bindings";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { ColumnVisibilityDropdown, Input } from "#components";
import {
  DropdownMenuGroup,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
} from "@/components/ui/dropdown-menu";
import type { CreditNoteResponse } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();

const LIMIT = 50;
const searchQuery = ref(queryString(route.query.search));
const createdFrom = ref(queryString(route.query.created_from));
const createdTo = ref(queryString(route.query.created_to));
const totalMin = ref(queryString(route.query.total_min));
const totalMax = ref(queryString(route.query.total_max));

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
  created_from: createdFrom.value || null,
  created_to: createdTo.value || null,
  total_min: totalMin.value ? queryNumber(totalMin.value, 0) : null,
  total_max: totalMax.value ? queryNumber(totalMax.value, 0) : null,
}));

const activeFilters = computed(
  () =>
    [
      createdFrom.value
        ? {
            key: "created_from",
            label: t("filters.from"),
            value: d(new Date(createdFrom.value), "short"),
          }
        : null,
      createdTo.value
        ? {
            key: "created_to",
            label: t("filters.to"),
            value: d(new Date(createdTo.value), "short"),
          }
        : null,
      totalMin.value
        ? { key: "total_min", label: `${t("fields.total")} ${t("filters.min")}`, value: totalMin.value }
        : null,
      totalMax.value
        ? { key: "total_max", label: `${t("fields.total")} ${t("filters.max")}`, value: totalMax.value }
        : null,
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

async function fetchCreditNotes() {
  const result = await commands.listCreditNotes({
    limit: queryParams.value.limit,
    offset: (queryParams.value.page - 1) * queryParams.value.limit,
    search: queryParams.value.search,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
    created_from: queryParams.value.created_from,
    created_to: queryParams.value.created_to,
    total_min: queryParams.value.total_min,
    total_max: queryParams.value.total_max,
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

watch(createdFrom, () => {
  updateQueryParams({ created_from: createdFrom.value || "", page: 1 });
});

watch(createdTo, () => {
  updateQueryParams({ created_to: createdTo.value || "", page: 1 });
});

watch(totalMin, () => {
  updateQueryParams({ total_min: totalMin.value || "", page: 1 });
});

watch(totalMax, () => {
  updateQueryParams({ total_max: totalMax.value || "", page: 1 });
});

function clearFilter(key: string) {
  if (key === "created_from") createdFrom.value = "";
  if (key === "created_to") createdTo.value = "";
  if (key === "total_min") totalMin.value = "";
  if (key === "total_max") totalMax.value = "";
}

function clearAllFilters() {
  createdFrom.value = "";
  createdTo.value = "";
  totalMin.value = "";
  totalMax.value = "";
}

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
    <div class="flex h-full w-full flex-col items-start justify-start gap-2">
      <div class="flex w-full items-start justify-between">
        <div class="w-full ltr:text-left rtl:text-right">
          <h1 class="text-3xl font-bold text-slate-900">
            {{ t("credit-notes.title") }}
          </h1>
          <p class="mt-1 text-slate-500">
            {{ t("credit-notes.description") }}
          </p>
        </div>
      </div>

      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #advanced>
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("fields.date") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent class="w-52 p-3">
                <div class="space-y-5">
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.from") }}</p>
                    <Input v-model="createdFrom" type="date" />
                  </div>
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.to") }}</p>
                    <Input v-model="createdTo" type="date" />
                  </div>
                </div>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("fields.total") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent class="w-52 p-3">
                <div class="space-y-5">
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.min") }}</p>
                    <Input v-model="totalMin" type="number" :placeholder="t('filters.min')" />
                  </div>
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.max") }}</p>
                    <Input v-model="totalMax" type="number" :placeholder="t('filters.max')" />
                  </div>
                </div>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
        </template>

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
