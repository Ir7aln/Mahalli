<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Calendar as CalendarIcon, Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { OrderCreate } from "#components";
import { ORDER_STATUSES } from "@/consts";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const orderProducts = ref<OrderProductsPreviewT[]>([]);

const searchQuery = ref(route.query.search as any);
const status = ref(route.query.status as any);
const created_at = ref(route.query.created_at as any);

const LIMIT = 50;

const queryParams = computed<QueryParams>(() => ({
  search: route.query.search,
  page: route.query.page,
  refresh: route.query.refresh,
  limit: route.query.limit,
  status: route.query.status,
  created_at: route.query.created_at,
}));

async function fetchOrders() {
  try {
    const res: Res<any> = await invoke("list_orders", {
      args: {
        page: Number(queryParams.value.page) ?? 1,
        search: queryParams.value.search ?? "",
        limit: queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT,
        status: queryParams.value.status,
        created_at: queryParams.value.created_at,
      },
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST ORDERS: ${err.error ? err.error : err.message}`);
  }
}

const { data: ordersData } = await useAsyncData(fetchOrders, {
  watch: [queryParams],
});

const orders = computed<OrderT[]>(() => ordersData.value?.orders ?? []);
const totalRows = computed<number>(() => ordersData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

watch(queryParams, fetchOrders, { deep: true });

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

watch([status, created_at], () => {
  updateQueryParams({
    status: status.value,
    created_at: created_at.value ? new Date(created_at.value).toISOString() : undefined,
    page: 1,
  });
});

async function listOrderProducts(id?: string) {
  try {
    const res = await invoke<Res<any>>("list_order_products", {
      id,
    });
    orderProducts.value = res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR LIST ORDER PRODUCTS: ${err.error ? err.error : err.message}`);
  }
}

const openCreateOrderModal = () => modal.open(OrderCreate, { sheet: true });
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full grid grid-cols-3 gap-2 lg:max-w-screen-lg">
          <Input v-model="searchQuery" name="search" type="text" :placeholder="t('search')" />
          <Popover>
            <PopoverTrigger as-child>
              <Button
                variant="outline"
                :class="
                  cn(
                    'w-full justify-start text-left font-normal',
                    !created_at && 'text-muted-foreground',
                  )
                "
              >
                <CalendarIcon class="mr-2 h-4 w-4" />
                <span class="text-nowrap">{{
                  created_at ? d(new Date(created_at), "short") : t("pick-date")
                }}</span>
              </Button>
            </PopoverTrigger>
            <PopoverContent class="w-auto p-0">
              <Calendar v-model="created_at" />
            </PopoverContent>
          </Popover>
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
        <Button class="gap-2 text-nowrap" @click="openCreateOrderModal">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-order") }}
        </Button>
      </div>
      <OrdersTable
        :orders="orders"
        :order-products="orderProducts"
        @list-order-products="listOrderProducts"
      />
    </div>
  </main>
</template>
