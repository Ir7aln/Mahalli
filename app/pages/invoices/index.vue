<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Calendar as CalendarIcon, Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { InvoiceCreate } from "#components";
import { INVOICE_STATUSES } from "@/consts";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const invoiceProducts = ref<InvoiceProductsPreviewT[]>([]);

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

async function fetchInvoices() {
  try {
    const res: Res<any> = await invoke("list_invoices", {
      args: {
        page: Number(queryParams.value.page) ?? 1,
        search: queryParams.value.search ?? "",
        limit: queryParams.value.limit
          ? Number(queryParams.value.limit)
          : LIMIT,
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
    Logger.error(`LIST INVOICESS: ${err.error ? err.error : err.message}`);
  }
}

const { data: invoicesData } = await useAsyncData(fetchInvoices, {
  watch: [queryParams],
});

const invoices = computed<InvoiceT[]>(() => invoicesData.value?.invoices ?? []);
const totalRows = computed<number>(() => invoicesData.value?.count ?? 0);

provide("count", totalRows);
provide(
  "itemsPerPage",
  queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT
);

watch(queryParams, fetchInvoices, { deep: true });

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

watch([status, created_at], () => {
  updateQueryParams({
    status: status.value,
    created_at: created_at.value
      ? new Date(created_at.value).toISOString()
      : undefined,
    page: 1,
  });
});

async function listInvoiceProducts(id?: string) {
  try {
    const res = await invoke<Res<any>>("list_invoice_products", {
      id,
    });
    invoiceProducts.value = res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR LIST INVOICES PRODUCTS: ${err.error ? err.error : err.message}`
    );
  }
}

const openCreateInvoiceModal = () => modal.open(InvoiceCreate, { sheet: true });
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full grid grid-cols-3 gap-2 lg:max-w-screen-lg">
          <Input
            v-model="searchQuery"
            name="search"
            type="text"
            :placeholder="t('search')"
          />
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
              <SelectValue
                class="text-muted-foreground"
                :placeholder="t('select-status')"
              />
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
