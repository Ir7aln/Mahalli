<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectClients } from "@/bindings";
import { ClientCreate } from "#components";
import type { QueryParams } from "@/types/query";

const route = useRoute();
const { t } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();
const searchQuery = ref(route.query.search as string);

const LIMIT = 50;

const queryParams = computed(() => ({
  search: route.query.search,
  page: route.query.page,
  refresh: route.query.refresh,
  limit: route.query.limit,
}));

async function fetchClients() {
  const result = await commands.listClients({
    search: String(queryParams.value.search ?? ""),
    page: Number(queryParams.value.page ?? 1),
    limit: queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT,
    status: null,
    created_at: null,
  });
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`LIST CLIENTS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
}

const { data: clientsData } = await useAsyncData(fetchClients, {
  watch: [queryParams],
});

const clients = computed<SelectClients[]>(() => clientsData.value?.clients ?? []);
const totalRows = computed<number>(() => clientsData.value?.count ?? 0);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit ? Number(queryParams.value.limit) : LIMIT);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value });
}, 500);

watch(searchQuery, debouncedSearch);

const openCreateClientModal = () => modal.open(ClientCreate, {});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col items-start justify-start">
      <div class="flex justify-between w-full gap-9 mb-2">
        <div class="w-full max-w-md">
          <Input v-model="searchQuery" type="text" :placeholder="t('search')" />
        </div>
        <Button class="gap-2 text-nowrap" @click="openCreateClientModal()">
          <Plus :size="20" />
          {{ t("buttons.toggle-create-client") }}
        </Button>
      </div>
      <ClientsTable :clients="clients" />
    </div>
  </main>
</template>
