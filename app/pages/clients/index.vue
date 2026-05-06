<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectClients } from "@/bindings";
import { ClientCreate, ColumnVisibilityDropdown } from "#components";
import { queryBoolean, queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();

const searchQuery = ref(queryString(route.query.search));
const searchField = ref<string | null>(queryString(route.query.search_field) || null);
const creditOnly = ref<boolean>(queryBoolean(route.query.credit_only, false) ?? false);

const clientTableColumns = [
  { key: "avatar", label: "Avatar" },
  { key: "full_name", label: t("fields.full-name") },
  { key: "email", label: t("fields.email") },
  { key: "phone_number", label: t("fields.phone") },
  { key: "address", label: t("fields.address") },
  { key: "ice", label: t("fields.ice") },
  { key: "if_number", label: t("fields.if-number") },
  { key: "rc", label: t("fields.rc") },
  { key: "patente", label: t("fields.patente") },
  { key: "credit", label: t("fields.credit") },
];

const visibleColumns = ref<string[]>(clientTableColumns.map((col) => col.key));

onMounted(async () => {
  const preferences = await commands.getColumnPreferences("clients");
  if (preferences.status === "ok" && preferences.data?.data?.visible_columns) {
    visibleColumns.value = preferences.data.data.visible_columns;
  }
});

watch(
  visibleColumns,
  async (newColumns) => {
    await commands.saveColumnPreferences({
      page: "clients",
      visible_columns: newColumns,
    });
  },
  { deep: true },
);

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  search_field: queryString(route.query.search_field) || null,
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
    search_field: queryParams.value.search_field,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    credit_only: queryParams.value.credit_only ? true : null,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
  });
  if (result.status === "error") {
    showErrorToast(result.error);
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
const activeFilters = computed(() => {
  const filters = [];
  if (creditOnly.value) {
    filters.push({ key: "credit_only", label: t("fields.credit"), value: "> 0" });
  }
  if (searchField.value) {
    const fieldLabels: Record<string, string> = {
      email: t("fields.email"),
      phone_number: t("fields.phone"),
      address: t("fields.address"),
      ice: t("fields.ice"),
      if_number: t("fields.if-number"),
      rc: t("fields.rc"),
      patente: t("fields.patente"),
    };
    filters.push({
      key: "search_field",
      label: `${t("search")} - ${fieldLabels[searchField.value] || searchField.value}`,
      value: searchQuery.value,
    });
  }
  return filters;
});

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
  if (key === "search_field") {
    searchField.value = null;
    updateQueryParams({ search_field: null, page: 1 });
  }
}

function clearAllFilters() {
  creditOnly.value = false;
  searchField.value = null;
  updateQueryParams({ search_field: null, page: 1 });
}

const openCreateClientModal = () => modal.open(ClientCreate, {});
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start gap-2">
      <div class="flex w-full items-start justify-between mb-4">
        <div class="w-full ltr:text-left rtl:text-right">
          <h1 class="text-3xl font-bold text-slate-900">
            {{ t("clients.title") }}
          </h1>
          <p class="mt-1 text-slate-500">
            {{ t("clients.description") }}
          </p>
        </div>
      </div>

      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #actions>
          <Button size="sm" class="gap-2 text-nowrap" @click="openCreateClientModal()">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-client") }}
          </Button>
        </template>
        <template #columns>
          <ColumnVisibilityDropdown
            :columns="clientTableColumns"
            :visible-columns="visibleColumns"
            @update:visible-columns="(cols) => (visibleColumns = cols)"
          />
        </template>
        <template #advanced>
          <DropdownMenuGroup>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{{ t("filters.search-by-field") }}</DropdownMenuSubTrigger>
              <DropdownMenuSubContent>
                <DropdownMenuCheckboxItem
                  :checked="!searchField"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = null;
                      updateQueryParams({ search_field: null, page: 1 });
                    }
                  "
                >
                  {{ t("filters.clear-all") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuSeparator />
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'email'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'email' ? null : 'email';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.email") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'phone_number'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'phone_number' ? null : 'phone_number';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.phone") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'address'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'address' ? null : 'address';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.address") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'ice'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'ice' ? null : 'ice';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.ice") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'if_number'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'if_number' ? null : 'if_number';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.if-number") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'rc'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'rc' ? null : 'rc';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.rc") }}
                </DropdownMenuCheckboxItem>
                <DropdownMenuCheckboxItem
                  :checked="searchField === 'patente'"
                  @select.prevent
                  @update:checked="
                    () => {
                      searchField = searchField === 'patente' ? null : 'patente';
                      updateQueryParams({ search_field: searchField, page: 1 });
                    }
                  "
                >
                  {{ t("fields.patente") }}
                </DropdownMenuCheckboxItem>
              </DropdownMenuSubContent>
            </DropdownMenuSub>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuCheckboxItem
            :checked="creditOnly"
            @select.prevent
            @update:checked="creditOnly = $event"
          >
            {{ t("filters.credit-only") }}
          </DropdownMenuCheckboxItem>
        </template>
      </ListFilterBar>
      <ClientsTable :clients="clients" :visible-columns="visibleColumns" />
    </div>
  </main>
</template>
