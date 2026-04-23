<script setup lang="ts">
import { commands } from "@/bindings";
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { OrderCreate } from "#components";
import { ORDER_STATUSES } from "@/consts";
import type { OrderProductItem, SelectOrders } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const orderProducts = ref<OrderProductItem[]>([]);

const searchQuery = ref(queryString(route.query.search));
const status = ref(queryString(route.query.status));
const createdFrom = ref(queryString(route.query.created_from));
const createdTo = ref(queryString(route.query.created_to));

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  status: queryString(route.query.status) || null,
  created_from: queryString(route.query.created_from) || null,
  created_to: queryString(route.query.created_to) || null,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchOrders() {
  const result = await commands.listOrders({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    status: queryParams.value.status,
    created_from: queryParams.value.created_from,
    created_to: queryParams.value.created_to,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: ordersData } = await useAsyncData(fetchOrders, {
  watch: [queryParams],
});

const orders = computed<SelectOrders[]>(() => ordersData.value?.orders ?? []);
const totalRows = computed<number>(() => ordersData.value?.count ?? 0);
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
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "", page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

watch([status, createdFrom, createdTo], () => {
  updateQueryParams({
    status: status.value || null,
    created_from: createdFrom.value || null,
    created_to: createdTo.value || null,
    page: 1,
  });
});

async function listOrderProducts(id?: string) {
  if (!id) {
    orderProducts.value = [];
    return;
  }
  const result = await commands.listOrderProducts(id);
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR LIST ORDER PRODUCTS: ${JSON.stringify(result.error)}`);
    return;
  }
  orderProducts.value = result.data.data ?? [];
}

function clearFilter(key: string) {
  if (key === "status") status.value = "";
  if (key === "created_from") createdFrom.value = "";
  if (key === "created_to") createdTo.value = "";
}

function clearAllFilters() {
  status.value = "";
  createdFrom.value = "";
  createdTo.value = "";
}

const openCreateOrderModal = () => modal.open(OrderCreate, { sheet: true });
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
          <div class="grid gap-4 sm:grid-cols-2">
            <div class="space-y-2">
              <p class="text-sm font-medium text-slate-600">
                {{ t("fields.status") }}
              </p>
              <Select v-model="status" name="status">
                <SelectTrigger>
                  <SelectValue class="text-muted-foreground" :placeholder="t('select-status')" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem
                    v-for="orderStatus in ORDER_STATUSES"
                    :key="orderStatus"
                    :value="orderStatus"
                  >
                    {{ t(`status.${orderStatus.toLowerCase()}`) }}
                  </SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="grid gap-3 sm:grid-cols-2">
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
          </div>
        </template>
        <template #actions>
          <Button class="gap-2 text-nowrap" @click="openCreateOrderModal">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-order") }}
          </Button>
        </template>
      </ListFilterBar>
      <OrdersTable
        :orders="orders"
        :order-products="orderProducts"
        @list-order-products="listOrderProducts"
      />
    </div>
  </main>
</template>
