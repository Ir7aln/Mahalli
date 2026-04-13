<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Calendar as CalendarIcon, Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { QuoteCreate } from "#components";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref(route.query.search as string);
const created_at = ref(route.query.created_at as any);

const quoteProducts = ref<QuoteProductsPreviewT[]>([]);

const LIMIT = 50;

const queryParams = computed<QueryParams>(() => ({
  search: route.query.search,
  page: route.query.page,
  refresh: route.query.refresh,
  limit: route.query.limit,
  created_at: route.query.created_at,
}));

async function fetchQuotes() {
  try {
    const res: Res<any> = await invoke("list_quotes", {
      args: {
        page: Number(queryParams.value.page) ?? 1,
        search: queryParams.value.search ?? "",
        limit: queryParams.value.limit
          ? Number(queryParams.value.limit)
          : LIMIT,
        created_at: queryParams.value.created_at,
      },
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST QUOTES: ${err.error ? err.error : err.message}`);
  }
}

const { data: quotesData } = await useAsyncData(fetchQuotes, {
  watch: [queryParams],
});

const quotes = computed<QuoteT[]>(() => quotesData.value?.quotes ?? []);
const totalRows = computed<number>(() => quotesData.value?.count ?? 0);

provide("count", totalRows);
provide(
  "itemsPerPage",
  queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT
);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

watch(created_at, () => {
  updateQueryParams({
    created_at: created_at.value
      ? new Date(created_at.value).toISOString()
      : undefined,
  });
});

async function listQuoteProducts(id?: string) {
  try {
    const res = await invoke<Res<any>>("list_quote_products", {
      id,
    });
    quoteProducts.value = res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST QUOTE PRODUCTS: ${err.error ? err.error : err.message}`);
  }
}

const openCreateQuoteModal = () => modal.open(QuoteCreate, {});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full grid grid-cols-2 gap-2 lg:max-w-screen-md">
          <Input v-model="searchQuery" type="text" :placeholder="t('search')" />
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
        </div>
        <Button class="gap-2 text-nowrap" @click="openCreateQuoteModal">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-quote") }}
        </Button>
      </div>
      <QuotesTable
        :quotes="quotes"
        :quote-products="quoteProducts"
        @list-quote-products="listQuoteProducts"
      />
    </div>
  </main>
</template>
