<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { ProductCreate } from "#components";

const route = useRoute();
const { t } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref(route.query.search as string);

const LIMIT = 50;

const queryParams = computed<QueryParams>(() => ({
  search: route.query.search,
  page: route.query.page,
  refresh: route.query.refresh,
  limit: route.query.limit,
}));

async function fetchProducts() {
  try {
    const res: any = await invoke("list_products", {
      args: {
        search: queryParams.value.search ?? "",
        page: Number(queryParams.value.page) ?? 1,
        limit: queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT,
      },
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST PRODUCTS: ${err.error ? err.error : err.message}`);
  }
}

const { data: productsData } = await useAsyncData(fetchProducts, {
  watch: [queryParams],
});

const products = computed<ProductT[]>(() => productsData.value?.products ?? []);
const totalRows = computed<number>(() => productsData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

const openCreateProductModal = () => modal.open(ProductCreate, {});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full max-w-md">
          <Input v-model="searchQuery" type="text" :placeholder="t('search')" />
        </div>
        <Button class="gap-2 text-nowrap" @click="openCreateProductModal">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-product") }}
        </Button>
      </div>
      <ProductsTable :products="products" />
    </div>
  </main>
</template>
