<script setup lang="ts">
import { commands } from "@/bindings";
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { QuoteCreate, ColumnVisibilityDropdown } from "#components";
import type { QuoteProductItem, SelectQuotes } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const { showErrorToast } = useCommandError();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();

const quoteTableColumns = [
  { key: "identifier", label: t("fields.identifier") },
  { key: "full_name", label: t("fields.full-name") },
  { key: "status", label: t("fields.status") },
  { key: "products", label: t("fields.items") },
  { key: "created_at", label: t("fields.date") },
  { key: "total", label: t("fields.total") },
];

const visibleColumns = ref<string[]>(quoteTableColumns.map((col) => col.key));

const searchQuery = ref(queryString(route.query.search));
const status = ref(queryString(route.query.status));
const createdFrom = ref(queryString(route.query.created_from));
const createdTo = ref(queryString(route.query.created_to));
const totalMin = ref(queryString(route.query.total_min));
const totalMax = ref(queryString(route.query.total_max));
const quoteProducts = ref<QuoteProductItem[]>([]);

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  status: queryString(route.query.status) || null,
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  created_from: queryString(route.query.created_from) || null,
  created_to: queryString(route.query.created_to) || null,
  total_min: queryString(route.query.total_min) ? queryNumber(route.query.total_min, 0) : null,
  total_max: queryString(route.query.total_max) ? queryNumber(route.query.total_max, 0) : null,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchQuotes() {
  const result = await commands.listQuotes({
    search: queryParams.value.search,
    status: queryParams.value.status,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    created_from: queryParams.value.created_from,
    created_to: queryParams.value.created_to,
    total_min: queryParams.value.total_min,
    total_max: queryParams.value.total_max,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: quotesData } = await useAsyncData(fetchQuotes, {
  watch: [queryParams],
});

const quotes = computed<SelectQuotes[]>(() => quotesData.value?.quotes ?? []);
const totalRows = computed<number>(() => quotesData.value?.count ?? 0);
const activeFilters = computed(
  () =>
    [
      status.value
        ? {
            key: "status",
            label: t("fields.status"),
            value: t(`status.${status.value.toLowerCase()}`),
          }
        : null,
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

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "", page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

watch([status, createdFrom, createdTo, totalMin, totalMax], () => {
  updateQueryParams({
    status: status.value || null,
    created_from: createdFrom.value || null,
    created_to: createdTo.value || null,
    total_min: totalMin.value || null,
    total_max: totalMax.value || null,
    page: 1,
  });
});

onMounted(async () => {
  const preferences = await commands.getColumnPreferences("quotes");
  if (preferences.status === "ok" && preferences.data?.data?.visible_columns) {
    visibleColumns.value = preferences.data.data.visible_columns;
  }
});

watch(
  visibleColumns,
  async (newColumns) => {
    await commands.saveColumnPreferences({
      page: "quotes",
      visible_columns: newColumns,
    });
  },
  { deep: true },
);

async function listQuoteProducts(id?: string) {
  if (!id) {
    quoteProducts.value = [];
    return;
  }
  const result = await commands.listQuoteProducts(id);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`LIST QUOTE PRODUCTS: ${JSON.stringify(result.error)}`);
    return;
  }
  quoteProducts.value = result.data.data ?? [];
}

function clearFilter(key: string) {
  if (key === "status") status.value = "";
  if (key === "created_from") createdFrom.value = "";
  if (key === "created_to") createdTo.value = "";
  if (key === "total_min") totalMin.value = "";
  if (key === "total_max") totalMax.value = "";
}

function clearAllFilters() {
  status.value = "";
  createdFrom.value = "";
  createdTo.value = "";
  totalMin.value = "";
  totalMax.value = "";
}

const openCreateQuoteModal = () => modal.open(QuoteCreate, { sheet: true });
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
        <template #actions>
          <Button class="gap-2 text-nowrap" @click="openCreateQuoteModal">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-quote") }}
          </Button>
        </template>
        <template #columns>
          <ColumnVisibilityDropdown
            :columns="quoteTableColumns"
            :visible-columns="visibleColumns"
            @update:visible-columns="(cols) => (visibleColumns = cols)"
          />
        </template>
        <template #advanced>
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("fields.status") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent>
                <DropdownMenuCheckboxItem
                  :checked="status === 'PENDING'"
                  @select.prevent
                  @update:checked="status = status === 'PENDING' ? '' : 'PENDING'"
                >
                  {{ t("status.pending") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="status === 'ACCEPTED'"
                  @select.prevent
                  @update:checked="status = status === 'ACCEPTED' ? '' : 'ACCEPTED'"
                >
                  {{ t("status.accepted") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="status === 'CANCELLED'"
                  @select.prevent
                  @update:checked="status = status === 'CANCELLED' ? '' : 'CANCELLED'"
                >
                  {{ t("status.cancelled") }}
                </DropdownMenuCheckboxItem>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
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
      </ListFilterBar>
      <QuotesTable
        :quotes="quotes"
        :quote-products="quoteProducts"
        :visible-columns="visibleColumns"
        @list-quote-products="listQuoteProducts"
      />
    </div>
  </main>
</template>
