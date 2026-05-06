<script setup lang="ts">
import { commands } from "@/bindings";
import { GripHorizontal, Printer } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { NuxtLink } from "#components";
import { queryString } from "@/utils/query";

const props = defineProps<{
  creditNotes: any[];
  visibleColumns?: string[];
}>();

const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d, locale, n } = useI18n();
const localePath = useLocalePath();
const sortKey = computed(() => queryString(route.query.sort));
const sortDirection = computed(() =>
  queryString(route.query.direction) === "desc" ? "desc" : "asc",
);

const visibleCols = computed(
  () =>
    props.visibleColumns ?? [
      "identifier",
      "full_name",
      "invoice_identifier",
      "reason",
      "created_at",
      "total",
    ],
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
          <TableHead v-if="visibleCols.includes('identifier')" class="w-28" />
          <TableHead v-if="visibleCols.includes('full_name')">
            <TableSortHeader
              :label="t('fields.full-name')"
              :active="sortKey === 'full_name'"
              :direction="sortDirection"
              @click="toggleSort('full_name')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('invoice_identifier')">
            <TableSortHeader
              :label="t('fields.invoice')"
              :active="sortKey === 'invoice_identifier'"
              :direction="sortDirection"
              @click="toggleSort('invoice_identifier')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('reason')">
            <TableSortHeader
              :label="t('fields.reason')"
              :active="sortKey === 'reason'"
              :direction="sortDirection"
              @click="toggleSort('reason')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('created_at')" class="w-56">
            <TableSortHeader
              :label="t('fields.date')"
              :active="sortKey === 'created_at'"
              :direction="sortDirection"
              @click="toggleSort('created_at')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('total')">
            <TableSortHeader
              :label="t('fields.total')"
              :active="sortKey === 'total'"
              :direction="sortDirection"
              @click="toggleSort('total')"
            />
          </TableHead>
          <TableHead class="w-20 sticky ltr:right-0 rtl:left-0 bg-gray-100 z-10">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(creditNote, index) in props.creditNotes"
          :key="creditNote.id"
          v-fade="index"
          :class="{
            'animate-highlight-row':
              creditNote.id === $route.query.id && $route.query.highlight === 'true',
          }"
        >
          <TableCell v-if="visibleCols.includes('identifier')" class="p-2 text-nowrap font-medium">
            {{ creditNote.identifier }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('full_name')" class="p-2 font-medium">
            {{ creditNote.full_name }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('invoice_identifier')" class="p-2">
            <NuxtLink
              :to="
                localePath({
                  path: '/invoices/',
                  query: { page: 1, search: creditNote.invoice_identifier },
                })
              "
              class="inline-flex items-center gap-2 underline decoration-slate-300 underline-offset-4"
            >
              {{ creditNote.invoice_identifier }}
            </NuxtLink>
          </TableCell>
          <TableCell v-if="visibleCols.includes('reason')" class="p-2">
            {{ creditNote.reason || "-" }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('created_at')" class="p-2">
            {{ d(new Date(creditNote.created_at), "long") }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('total')" class="p-2">
            {{ n(toNumber(creditNote.total), "currency") }}
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem>
                    <NuxtLink :to="localePath(`/credit-notes/${creditNote.id}`)">
                      <Printer :size="20" class="text-slate-800 inline mr-2" />
                      {{ t("buttons.print") }}
                    </NuxtLink>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </TableCell>
        </TableRow>
        <TableEmpty v-if="!props.creditNotes.length" :colspan="visibleCols.length + 1">
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
