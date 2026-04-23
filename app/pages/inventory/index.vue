<script setup lang="ts">
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectInventory } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();

const searchQuery = ref(queryString(route.query.search));
const transactionType = ref(queryString(route.query.transaction_type));
const createdFrom = ref(queryString(route.query.created_from));
const createdTo = ref(queryString(route.query.created_to));
const quantityMin = ref(queryString(route.query.quantity_min));
const quantityMax = ref(queryString(route.query.quantity_max));
const priceMin = ref(queryString(route.query.price_min));
const priceMax = ref(queryString(route.query.price_max));

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  transaction_type: queryString(route.query.transaction_type) || null,
  created_from: queryString(route.query.created_from) || null,
  created_to: queryString(route.query.created_to) || null,
  quantity_min: queryString(route.query.quantity_min)
    ? queryNumber(route.query.quantity_min, 0)
    : null,
  quantity_max: queryString(route.query.quantity_max)
    ? queryNumber(route.query.quantity_max, 0)
    : null,
  price_min: queryString(route.query.price_min) ? queryNumber(route.query.price_min, 0) : null,
  price_max: queryString(route.query.price_max) ? queryNumber(route.query.price_max, 0) : null,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchInventory() {
  const result = await commands.listInventory({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    transaction_type: queryParams.value.transaction_type,
    created_from: queryParams.value.created_from,
    created_to: queryParams.value.created_to,
    quantity_min: queryParams.value.quantity_min,
    quantity_max: queryParams.value.quantity_max,
    price_min: queryParams.value.price_min,
    price_max: queryParams.value.price_max,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST INVENTORY: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: inventoryData } = await useAsyncData(fetchInventory, {
  watch: [queryParams],
});

const inventory = computed<SelectInventory[]>(() => inventoryData.value?.inventory ?? []);
const totalRows = computed<number>(() => inventoryData.value?.count ?? 0);
const activeFilters = computed(
  () =>
    [
      transactionType.value
        ? {
            key: "transaction_type",
            label: t("fields.status"),
            value: t(`status.${transactionType.value.toLowerCase()}`),
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
      quantityMin.value
        ? { key: "quantity_min", label: t("filters.min"), value: quantityMin.value }
        : null,
      quantityMax.value
        ? { key: "quantity_max", label: t("filters.max"), value: quantityMax.value }
        : null,
      priceMin.value ? { key: "price_min", label: t("filters.min"), value: priceMin.value } : null,
      priceMax.value ? { key: "price_max", label: t("filters.max"), value: priceMax.value } : null,
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value, page: 1 });
}, 350);

const debouncedFilters = useDebounceFn(() => {
  updateQueryParams({
    transaction_type: transactionType.value || null,
    created_from: createdFrom.value || null,
    created_to: createdTo.value || null,
    quantity_min: quantityMin.value || null,
    quantity_max: quantityMax.value || null,
    price_min: priceMin.value || null,
    price_max: priceMax.value || null,
    page: 1,
  });
}, 250);

watch(searchQuery, debouncedSearch);
watch(
  [transactionType, createdFrom, createdTo, quantityMin, quantityMax, priceMin, priceMax],
  debouncedFilters,
);

function clearFilter(key: string) {
  if (key === "transaction_type") transactionType.value = "";
  if (key === "created_from") createdFrom.value = "";
  if (key === "created_to") createdTo.value = "";
  if (key === "quantity_min") quantityMin.value = "";
  if (key === "quantity_max") quantityMax.value = "";
  if (key === "price_min") priceMin.value = "";
  if (key === "price_max") priceMax.value = "";
}

function clearAllFilters() {
  transactionType.value = "";
  createdFrom.value = "";
  createdTo.value = "";
  quantityMin.value = "";
  quantityMax.value = "";
  priceMin.value = "";
  priceMax.value = "";
}
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start">
      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        :advanced-label="t('filters.more')"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #advanced>
          <div class="space-y-4">
            <div class="grid gap-4 sm:grid-cols-3">
              <div class="space-y-2">
                <p class="text-sm font-medium text-slate-600">
                  {{ t("fields.status") }}
                </p>
                <Select v-model="transactionType">
                  <SelectTrigger>
                    <SelectValue class="text-muted-foreground" :placeholder="t('select-status')" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="OUT">
                      {{ t("status.out") }}
                    </SelectItem>
                    <SelectItem value="IN">
                      {{ t("status.in") }}
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div class="space-y-2">
                <p class="text-sm font-medium text-slate-600">
                  {{ t("filters.from") }}
                </p>
                <Input v-model="createdFrom" type="date" />
              </div>

              <div class="space-y-2">
                <p class="text-sm font-medium text-slate-600">
                  {{ t("filters.to") }}
                </p>
                <Input v-model="createdTo" type="date" />
              </div>
            </div>

            <div class="grid gap-4 sm:grid-cols-2">
              <div class="grid gap-3 sm:grid-cols-2">
                <div class="space-y-2">
                  <p class="text-sm font-medium text-slate-600">
                    {{ t("fields.quantity") }} {{ t("filters.min") }}
                  </p>
                  <Input v-model="quantityMin" type="number" :placeholder="t('filters.min')" />
                </div>
                <div class="space-y-2">
                  <p class="text-sm font-medium text-slate-600">
                    {{ t("fields.quantity") }} {{ t("filters.max") }}
                  </p>
                  <Input v-model="quantityMax" type="number" :placeholder="t('filters.max')" />
                </div>
              </div>

              <div class="grid gap-3 sm:grid-cols-2">
                <div class="space-y-2">
                  <p class="text-sm font-medium text-slate-600">
                    {{ t("fields.price") }} {{ t("filters.min") }}
                  </p>
                  <Input v-model="priceMin" type="number" :placeholder="t('filters.min')" />
                </div>
                <div class="space-y-2">
                  <p class="text-sm font-medium text-slate-600">
                    {{ t("fields.price") }} {{ t("filters.max") }}
                  </p>
                  <Input v-model="priceMax" type="number" :placeholder="t('filters.max')" />
                </div>
              </div>
            </div>
          </div>
        </template>
      </ListFilterBar>
      <InventoryTable :inventory="inventory" />
    </div>
  </main>
</template>
