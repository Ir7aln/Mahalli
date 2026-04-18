<script setup lang="ts">
import type { SelectInventory } from "@/bindings";
import { queryString } from "@/utils/query";

const props = defineProps<{
  inventory: SelectInventory[];
}>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d, locale, n } = useI18n();
const sortKey = computed(() => queryString(route.query.sort));
const sortDirection = computed(() => (queryString(route.query.direction) === "desc" ? "desc" : "asc"));

function toggleSort(key: string) {
  if (sortKey.value !== key) {
    updateQueryParams({ sort: key, direction: "asc", page: 1 });
    return;
  }
  if (sortDirection.value === "asc") {
    updateQueryParams({ direction: "desc", page: 1 });
    return;
  }
  updateQueryParams({ sort: "", direction: "", page: 1 });
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead>
            <TableSortHeader
              :label="t('fields.name')"
              :active="sortKey === 'name'"
              :direction="sortDirection"
              @click="toggleSort('name')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.price')"
              :active="sortKey === 'price'"
              :direction="sortDirection"
              @click="toggleSort('price')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.quantity')"
              :active="sortKey === 'quantity'"
              :direction="sortDirection"
              @click="toggleSort('quantity')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.status')"
              :active="sortKey === 'transaction_type'"
              :direction="sortDirection"
              @click="toggleSort('transaction_type')"
            />
          </TableHead>
          <TableHead class="w-56">
            <TableSortHeader
              :label="t('fields.date')"
              :active="sortKey === 'created_at'"
              :direction="sortDirection"
              @click="toggleSort('created_at')"
            />
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(tx, index) in props.inventory" :key="tx.id" v-fade="index">
          <TableCell class="p-2 font-medium">
            {{ tx?.name }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(toNumber(tx?.price), "currency") }}
          </TableCell>
          <TableCell class="p-2">
            {{ `${tx.quantity} ${t("plrz.i", { n: Math.ceil(tx.quantity) })}` }}
          </TableCell>
          <TableCell class="p-2">
            <Badge
              variant="outline"
              :class="
                cn(
                  'cursor-pointer whitespace-nowrap',
                  tx?.transaction_type === 'OUT'
                    ? 'bg-green-100 border-green-500 text-green-900'
                    : 'bg-sky-100 border-sky-500 text-sky-900',
                )
              "
            >
              {{ t(`status.${tx?.transaction_type.toLowerCase()}`) }}
            </Badge>
          </TableCell>
          <TableCell class="p-2">
            {{ d(new Date(tx.created_at), "long") }}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
    <Pagination />
  </div>
</template>
