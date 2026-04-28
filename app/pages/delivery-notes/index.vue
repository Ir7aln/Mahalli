<script setup lang="ts">
import { commands } from "@/bindings";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { ColumnVisibilityDropdown } from "#components";
import type { DeliveryNoteProductItem, SelectDeliveryNotes } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();
const deliveryNoteProducts = ref<DeliveryNoteProductItem[]>([]);

const searchQuery = ref(queryString(route.query.search));
const createdFrom = ref(queryString(route.query.created_from));
const createdTo = ref(queryString(route.query.created_to));

const deliveryNoteTableColumns = [
  { key: "identifier", label: t("fields.identifier") },
  { key: "full_name", label: t("fields.full-name") },
  { key: "order_identifier", label: t("fields.order") },
  { key: "products", label: t("fields.items") },
  { key: "created_at", label: t("fields.date") },
  { key: "total", label: t("fields.total") },
];

const visibleColumns = ref<string[]>(deliveryNoteTableColumns.map((col) => col.key));
const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  created_from: queryString(route.query.created_from) || null,
  created_to: queryString(route.query.created_to) || null,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchDeliveryNotes() {
  const result = await commands.listDeliveryNotes({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    created_from: queryParams.value.created_from,
    created_to: queryParams.value.created_to,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST DELIVERY NOTES: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: deliveryNotesData } = await useAsyncData(fetchDeliveryNotes, {
  watch: [queryParams],
});

const deliveryNotes = computed<SelectDeliveryNotes[]>(
  () => deliveryNotesData.value?.delivery_notes ?? [],
);
const totalRows = computed<number>(() => deliveryNotesData.value?.count ?? 0);
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
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "", page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

watch([createdFrom, createdTo], () => {
  updateQueryParams({
    created_from: createdFrom.value || null,
    created_to: createdTo.value || null,
    page: 1,
  });
});

onMounted(async () => {
  const preferences = await commands.getColumnPreferences("delivery-notes");
  if (preferences.status === "ok" && preferences.data?.data?.visible_columns) {
    visibleColumns.value = preferences.data.data.visible_columns;
  }
});

watch(visibleColumns, async (newColumns) => {
  await commands.saveColumnPreferences({
    page: "delivery-notes",
    visible_columns: newColumns,
  });
}, { deep: true });

async function listDeliveryNoteProducts(id?: string) {
  if (!id) {
    deliveryNoteProducts.value = [];
    return;
  }
  const result = await commands.listDeliveryNoteProducts(id);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST DELIVERY NOTE PRODUCTS: ${JSON.stringify(result.error)}`);
    return;
  }
  deliveryNoteProducts.value = result.data.data ?? [];
}

function clearFilter(key: string) {
  if (key === "created_from") createdFrom.value = "";
  if (key === "created_to") createdTo.value = "";
}

function clearAllFilters() {
  createdFrom.value = "";
  createdTo.value = "";
}
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start">
      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #columns>
          <ColumnVisibilityDropdown
            :columns="deliveryNoteTableColumns"
            :visible-columns="visibleColumns"
            @update:visible-columns="(cols) => (visibleColumns = cols)"
          />
        </template>
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
        </template>
      </ListFilterBar>

      <DeliveryNotesTable
        :delivery-notes="deliveryNotes"
        :delivery-note-products="deliveryNoteProducts"
        :visible-columns="visibleColumns"
        @list-delivery-note-products="listDeliveryNoteProducts"
      />
    </div>
  </main>
</template>
