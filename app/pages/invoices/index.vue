<script setup lang="ts">
import { commands } from "@/bindings";
import { Calendar as CalendarIcon, Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { InvoiceCreate } from "#components";
import { INVOICE_STATUSES } from "@/consts";
import type { InvoiceProductItem, SelectInvoices } from "@/bindings";
import { firstQueryValue, queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const invoiceProducts = ref<InvoiceProductItem[]>([]);

const searchQuery = ref<string>((route.query.search as string) ?? "");
const status = ref<string>(queryString(route.query.status));
const created_at = ref<string>(queryString(route.query.created_at));

const LIMIT = 50;

const queryParams = computed(() => {
  return {
    search: queryString(route.query.search),
    page: queryNumber(route.query.page, 1),
    limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
    status: queryString(route.query.status) || null,
    created_at: queryString(route.query.created_at) || null,
    refresh: queryString(route.query.refresh) || "",
    sort: queryString(route.query.sort) || "",
    direction: queryString(route.query.direction) || "",
  };
});

async function fetchInvoices() {
  const result = await commands.listInvoices({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    status: queryParams.value.status,
    created_at: queryParams.value.created_at,
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

const { data: invoicesData } = await useAsyncData(fetchInvoices, {
  watch: [queryParams],
});

const invoices = computed<SelectInvoices[]>(() => invoicesData.value?.invoices ?? []);
const totalRows = computed<number>(() => invoicesData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "" });
}, 500);

watch(searchQuery, debouncedSearch);

watch([status, created_at], () => {
  updateQueryParams({
    status: status.value || "",
    created_at: created_at.value ? new Date(created_at.value).toISOString() : "",
    page: 1,
  });
});

async function listInvoiceProducts(id?: string) {
  if (!id) {
    invoiceProducts.value = [];
    return;
  }
  const result = await commands.listInvoiceProducts(id);
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR LIST INVOICES PRODUCTS: ${JSON.stringify(result.error)}`);
    return;
  }
  invoiceProducts.value = result.data.data ?? [];
}

const openCreateInvoiceModal = () => modal.open(InvoiceCreate, { sheet: true });
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
                v-for="invoiceStatus in INVOICE_STATUSES"
                :key="invoiceStatus"
                :value="invoiceStatus"
              >
                {{ t(`status.${invoiceStatus.toLowerCase()}`) }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
        <Button class="gap-2 text-nowrap" @click="openCreateInvoiceModal()">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-invoice") }}
        </Button>
      </div>
      <InvoicesTable
        :invoices="invoices"
        :invoice-products="invoiceProducts"
        @list-invoice-products="listInvoiceProducts"
      />
    </div>
  </main>
</template>
