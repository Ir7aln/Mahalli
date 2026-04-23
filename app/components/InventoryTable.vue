<script setup lang="ts">
import type { SelectInventory } from "@/bindings";
import { queryString } from "@/utils/query";

const props = defineProps<{
  inventory: SelectInventory[];
}>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d, locale, n } = useI18n();
const localePath = useLocalePath();
const sortKey = computed(() => queryString(route.query.sort));
const sortDirection = computed(() =>
  queryString(route.query.direction) === "desc" ? "desc" : "asc",
);

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
            <div class="flex items-center justify-between gap-3">
              <span class="min-w-0 truncate">{{ tx?.name }}</span>
              <div
                v-if="tx.order_identifier || tx.invoice_identifier || tx.quote_identifier"
                class="flex shrink-0 flex-wrap justify-end gap-2 text-xs font-normal"
              >
                <NuxtLink
                  v-if="tx.invoice_identifier"
                  :to="
                    localePath({
                      path: '/invoices/',
                      query: { page: 1, search: tx.invoice_identifier },
                    })
                  "
                  class="text-slate-700 underline decoration-slate-300 underline-offset-4 hover:text-slate-950"
                >
                  {{ tx.invoice_identifier }}
                </NuxtLink>
                <NuxtLink
                  v-if="tx.order_identifier"
                  :to="
                    localePath({
                      path: '/orders/',
                      query: { page: 1, search: tx.order_identifier },
                    })
                  "
                  class="text-slate-700 underline decoration-slate-300 underline-offset-4 hover:text-slate-950"
                >
                  {{ tx.order_identifier }}
                </NuxtLink>
                <NuxtLink
                  v-if="tx.quote_identifier"
                  :to="
                    localePath({
                      path: '/quotes/',
                      query: { page: 1, search: tx.quote_identifier },
                    })
                  "
                  class="text-slate-700 underline decoration-slate-300 underline-offset-4 hover:text-slate-950"
                >
                  {{ tx.quote_identifier }}
                </NuxtLink>
              </div>
            </div>
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
        <TableEmpty v-if="!props.inventory.length" :colspan="5">
          <div class="space-y-1 text-center">
            <p class="font-medium text-slate-900">{{ t("tables.empty.title") }}</p>
            <p class="text-sm text-slate-500">{{ t("tables.empty.description") }}</p>
          </div>
        </TableEmpty>
      </TableBody>
    </Table>
    <Pagination />
  </div>
</template>
