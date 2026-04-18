<script setup lang="ts">
import { commands } from "@/bindings";
import { Calendar as CalendarIcon, Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { QuoteCreate } from "#components";
import type { QuoteProductItem, SelectQuotes } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t, d } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref<string>((route.query.search as string) ?? "");
const created_at = ref<string>(queryString(route.query.created_at));

const quoteProducts = ref<QuoteProductItem[]>([]);

const LIMIT = 50;

const queryParams = computed(() => {
  return {
    search: queryString(route.query.search),
    page: queryNumber(route.query.page, 1),
    limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
    created_at: queryString(route.query.created_at) || null,
    refresh: queryString(route.query.refresh) || "",
    sort: queryString(route.query.sort) || "",
    direction: queryString(route.query.direction) || "",
  };
});

async function fetchQuotes() {
  const result = await commands.listQuotes({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
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

const { data: quotesData } = await useAsyncData(fetchQuotes, {
  watch: [queryParams],
});

const quotes = computed<SelectQuotes[]>(() => quotesData.value?.quotes ?? []);
const totalRows = computed<number>(() => quotesData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "" });
}, 500);

watch(searchQuery, debouncedSearch);

watch(created_at, () => {
  updateQueryParams({
    created_at: created_at.value ? new Date(created_at.value).toISOString() : "",
  });
});

async function listQuoteProducts(id?: string) {
  if (!id) {
    quoteProducts.value = [];
    return;
  }
  const result = await commands.listQuoteProducts(id);
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST QUOTE PRODUCTS: ${JSON.stringify(result.error)}`);
    return;
  }
  quoteProducts.value = result.data.data ?? [];
}

const openCreateQuoteModal = () => modal.open(QuoteCreate, { sheet: true });
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
