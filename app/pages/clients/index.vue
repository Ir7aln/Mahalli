<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectClients } from "@/bindings";
import { ClientCreate } from "#components";
import { queryBoolean, queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();

const searchQuery = ref(queryString(route.query.search));
const creditOnly = ref<boolean>(queryBoolean(route.query.credit_only, false) ?? false);

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  credit_only: queryBoolean(route.query.credit_only, false) ?? false,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchClients() {
  const result = await commands.listClients({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    credit_only: queryParams.value.credit_only ? true : null,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
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
const activeFilters = computed(() =>
  creditOnly.value ? [{ key: "credit_only", label: t("fields.credit"), value: "> 0" }] : [],
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value, page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

watch(creditOnly, () => {
  updateQueryParams({
    credit_only: creditOnly.value ? true : null,
    page: 1,
  });
});

function clearFilter(key: string) {
  if (key === "credit_only") creditOnly.value = false;
}

function clearAllFilters() {
  creditOnly.value = false;
}

const openCreateClientModal = () => modal.open(ClientCreate, {});
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start">
      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #advanced>
          <DropdownMenuCheckboxItem
            :checked="creditOnly"
            @select.prevent
            @update:checked="creditOnly = $event"
          >
            {{ t("filters.credit-only") }}
          </DropdownMenuCheckboxItem>
        </template>
        <template #actions>
          <Button class="gap-2 text-nowrap" @click="openCreateClientModal()">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-client") }}
          </Button>
        </template>
      </ListFilterBar>
      <ClientsTable :clients="clients" />
    </div>
  </main>
</template>
