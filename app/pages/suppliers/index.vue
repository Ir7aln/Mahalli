<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { SupplierCreate } from "#components";

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

async function fetchSuppliers() {
  try {
    const res: Res<any> = await invoke("list_suppliers", {
      args: {
        search: queryParams.value.search ?? "",
        page: Number(queryParams.value.page) ?? 1,
        limit: queryParams.value.limit
          ? Number(queryParams.value.limit)
          : LIMIT,
      },
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST SUPPLIERS: ${err.error ? err.error : err.message}`);
  }
}

const { data } = useAsyncData(fetchSuppliers, { watch: [queryParams] });

const suppliers = computed<SupplierT[]>(() => data.value?.suppliers ?? []);
const totalRows = computed<number>(() => data.value?.count ?? 0);

provide("count", totalRows);
provide(
  "itemsPerPage",
  queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT
);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

const openCreateSupplierModal = () => modal.open(SupplierCreate, {});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full lg:max-w-screen-lg">
          <Input v-model="searchQuery" type="text" :placeholder="t('search')" />
        </div>
        <Button class="gap-2 text-nowrap" @click="openCreateSupplierModal">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-supplier") }}
        </Button>
      </div>
      <SuppliersTable :suppliers="suppliers" />
    </div>
  </main>
</template>
