<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands } from "@/bindings";
import { SupplierCreate } from "#components";
import type { SelectSuppliers } from "@/bindings";
import { queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref<string>(queryString(route.query.search));

const LIMIT = 50;

const queryParams = computed(() => {
  return {
    search: queryString(route.query.search),
    page: queryNumber(route.query.page, 1),
    limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
    refresh: queryString(route.query.refresh) || "",
    sort: queryString(route.query.sort) || "",
    direction: queryString(route.query.direction) || "",
  };
});

async function fetchSuppliers() {
  const result = await commands.listSuppliers({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    sort: queryString(route.query.sort) || null,
    direction: queryString(route.query.direction) || null,
  });
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST SUPPLIERS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: suppliersData } = await useAsyncData(fetchSuppliers, {
  watch: [queryParams],
});

const suppliers = computed<SelectSuppliers[]>(() => suppliersData.value?.suppliers ?? []);
const totalRows = computed<number>(() => suppliersData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "" });
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
