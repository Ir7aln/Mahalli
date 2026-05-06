<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectProducts } from "@/bindings";
import { ProductCreate, ColumnVisibilityDropdown } from "#components";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();

const productTableColumns = [
  { key: "image", label: "Image" },
  { key: "name", label: t("fields.name") },
  { key: "inventory", label: t("fields.inventory") },
  { key: "threshold", label: t("fields.threshold") },
  { key: "purchase_price", label: t("fields.purchase-price") },
  { key: "selling_price", label: t("fields.selling-price") },
];

const visibleColumns = ref<string[]>(productTableColumns.map((col) => col.key));

const searchQuery = ref(queryString(route.query.search));
const stockStatus = ref(queryString(route.query.stock_status));
const sellingPriceMin = ref(queryString(route.query.selling_price_min));
const sellingPriceMax = ref(queryString(route.query.selling_price_max));

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  stock_status: queryString(route.query.stock_status) || null,
  selling_price_min: queryString(route.query.selling_price_min)
    ? queryNumber(route.query.selling_price_min, 0)
    : null,
  selling_price_max: queryString(route.query.selling_price_max)
    ? queryNumber(route.query.selling_price_max, 0)
    : null,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchProducts() {
  const result = await commands.listProducts({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    stock_status: queryParams.value.stock_status,
    selling_price_min: queryParams.value.selling_price_min,
    selling_price_max: queryParams.value.selling_price_max,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`LIST PRODUCTS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: productsData } = await useAsyncData(fetchProducts, {
  watch: [queryParams],
});

const products = computed<SelectProducts[]>(() => productsData.value?.products ?? []);
const totalRows = computed<number>(() => productsData.value?.count ?? 0);
const stockStatusLabel = computed(() => {
  if (stockStatus.value === "out") return t("filters.out-of-stock");
  if (stockStatus.value === "low") return t("filters.low-stock");
  if (stockStatus.value === "healthy") return t("filters.healthy-stock");
  return "";
});
const activeFilters = computed(
  () =>
    [
      stockStatus.value
        ? { key: "stock_status", label: t("filters.stock-status"), value: stockStatusLabel.value }
        : null,
      sellingPriceMin.value
        ? { key: "selling_price_min", label: `${t("fields.selling-price")} ${t("filters.min")}`, value: sellingPriceMin.value }
        : null,
      sellingPriceMax.value
        ? { key: "selling_price_max", label: `${t("fields.selling-price")} ${t("filters.max")}`, value: sellingPriceMax.value }
        : null,
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value, page: 1 });
}, 350);

const debouncedFilters = useDebounceFn(() => {
  updateQueryParams({
    stock_status: stockStatus.value || null,
    selling_price_min: sellingPriceMin.value || null,
    selling_price_max: sellingPriceMax.value || null,
    page: 1,
  });
}, 250);

watch(searchQuery, debouncedSearch);
watch([stockStatus, sellingPriceMin, sellingPriceMax], debouncedFilters);

onMounted(async () => {
  const preferences = await commands.getColumnPreferences("products");
  if (preferences.status === "ok" && preferences.data?.data?.visible_columns) {
    visibleColumns.value = preferences.data.data.visible_columns;
  }
});

watch(
  visibleColumns,
  async (newColumns) => {
    await commands.saveColumnPreferences({
      page: "products",
      visible_columns: newColumns,
    });
  },
  { deep: true },
);

function clearFilter(key: string) {
  if (key === "stock_status") stockStatus.value = "";
  if (key === "selling_price_min") sellingPriceMin.value = "";
  if (key === "selling_price_max") sellingPriceMax.value = "";
}

function clearAllFilters() {
  stockStatus.value = "";
  sellingPriceMin.value = "";
  sellingPriceMax.value = "";
}

const openCreateProductModal = () => modal.open(ProductCreate, {});
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
          <Button class="gap-2 text-nowrap" @click="openCreateProductModal">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-product") }}
          </Button>
        </template>
        <template #columns>
          <ColumnVisibilityDropdown
            :columns="productTableColumns"
            :visible-columns="visibleColumns"
            @update:visible-columns="(cols) => (visibleColumns = cols)"
          />
        </template>
        <template #advanced>
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("filters.stock-status") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent>
                <DropdownMenuCheckboxItem
                  :checked="stockStatus === 'out'"
                  @select.prevent
                  @update:checked="stockStatus = stockStatus === 'out' ? '' : 'out'"
                >
                  {{ t("filters.out-of-stock") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="stockStatus === 'low'"
                  @select.prevent
                  @update:checked="stockStatus = stockStatus === 'low' ? '' : 'low'"
                >
                  {{ t("filters.low-stock") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="stockStatus === 'healthy'"
                  @select.prevent
                  @update:checked="stockStatus = stockStatus === 'healthy' ? '' : 'healthy'"
                >
                  {{ t("filters.healthy-stock") }}
                </DropdownMenuCheckboxItem>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("fields.selling-price") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent class="w-52 p-3">
                <div class="space-y-5">
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.min") }}</p>
                    <Input
                      v-model="sellingPriceMin"
                      type="number"
                      :placeholder="t('filters.min')"
                    />
                  </div>
                  <div class="space-y-2">
                    <p class="text-xs text-muted-foreground">{{ t("filters.max") }}</p>
                    <Input
                      v-model="sellingPriceMax"
                      type="number"
                      :placeholder="t('filters.max')"
                    />
                  </div>
                </div>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
        </template>
      </ListFilterBar>
      <ProductsTable :products="products" :visible-columns="visibleColumns" />
    </div>
  </main>
</template>
