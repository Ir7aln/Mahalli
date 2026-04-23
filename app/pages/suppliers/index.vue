<script setup lang="ts">
import { Plus } from "lucide-vue-next";
import { useDebounceFn } from "@vueuse/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type SelectSuppliers } from "@/bindings";
import { SupplierCreate } from "#components";
import { queryBoolean, queryNumber, queryString } from "@/utils/query";

const route = useRoute();
const { t } = useI18n();
const modal = useModal();
const { updateQueryParams } = useUpdateRouteQueryParams();

const searchQuery = ref(queryString(route.query.search));
const hasEmail = ref<boolean>(queryBoolean(route.query.has_email, false) ?? false);
const hasPhone = ref<boolean>(queryBoolean(route.query.has_phone, false) ?? false);

const LIMIT = 50;

const queryParams = computed(() => ({
  search: queryString(route.query.search),
  page: queryNumber(route.query.page, 1),
  limit: route.query.limit ? queryNumber(route.query.limit, LIMIT) : LIMIT,
  has_email: queryBoolean(route.query.has_email, false) ?? false,
  has_phone: queryBoolean(route.query.has_phone, false) ?? false,
  refresh: queryString(route.query.refresh),
  sort: queryString(route.query.sort) || null,
  direction: queryString(route.query.direction) || null,
}));

async function fetchSuppliers() {
  const result = await commands.listSuppliers({
    search: queryParams.value.search,
    page: queryParams.value.page,
    limit: queryParams.value.limit,
    has_email: queryParams.value.has_email ? true : null,
    has_phone: queryParams.value.has_phone ? true : null,
    sort: queryParams.value.sort,
    direction: queryParams.value.direction,
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
const activeFilters = computed(
  () =>
    [
      hasEmail.value
        ? { key: "has_email", label: t("fields.email"), value: t("filters.required") }
        : null,
      hasPhone.value
        ? { key: "has_phone", label: t("fields.phone"), value: t("filters.required") }
        : null,
    ].filter(Boolean) as Array<{ key: string; label: string; value: string }>,
);

provide("count", totalRows);
provide("itemsPerPage", queryParams.value.limit);

const debouncedSearch = useDebounceFn(() => {
  updateQueryParams({ search: searchQuery.value || "", page: 1 });
}, 350);

watch(searchQuery, debouncedSearch);

watch([hasEmail, hasPhone], () => {
  updateQueryParams({
    has_email: hasEmail.value ? true : null,
    has_phone: hasPhone.value ? true : null,
    page: 1,
  });
});

function clearFilter(key: string) {
  if (key === "has_email") hasEmail.value = false;
  if (key === "has_phone") hasPhone.value = false;
}

function clearAllFilters() {
  hasEmail.value = false;
  hasPhone.value = false;
}

const openCreateSupplierModal = () => modal.open(SupplierCreate, {});
</script>

<template>
  <main class="h-full w-full">
    <div class="flex h-full w-full flex-col items-start justify-start">
      <ListFilterBar
        :search="searchQuery"
        :active-filters="activeFilters"
        :advanced-label="t('filters.more')"
        @update:search="(value) => (searchQuery = value)"
        @clear-filter="clearFilter"
        @clear-all="clearAllFilters"
      >
        <template #advanced>
          <div class="grid gap-3 sm:grid-cols-2">
            <Button
              class="w-full justify-start"
              :variant="hasEmail ? 'default' : 'outline'"
              @click="hasEmail = !hasEmail"
            >
              {{ t("filters.has-email") }}
            </Button>
            <Button
              class="w-full justify-start"
              :variant="hasPhone ? 'default' : 'outline'"
              @click="hasPhone = !hasPhone"
            >
              {{ t("filters.has-phone") }}
            </Button>
          </div>
        </template>
        <template #actions>
          <Button class="gap-2 text-nowrap" @click="openCreateSupplierModal">
            <Plus :size="20" />
            {{ t("buttons.toggle-create-supplier") }}
          </Button>
        </template>
      </ListFilterBar>
      <SuppliersTable :suppliers="suppliers" />
    </div>
  </main>
</template>
