<script setup lang="ts">
import type { DeliveryNoteProductItem, SelectDeliveryNotes } from "@/bindings";
import { FileText, GripHorizontal, Printer } from "lucide-vue-next";
import { queryString } from "@/utils/query";

const props = defineProps<{
  deliveryNotes: SelectDeliveryNotes[];
  deliveryNoteProducts: DeliveryNoteProductItem[];
  visibleColumns?: string[];
}>();
const emits = defineEmits<{
  listDeliveryNoteProducts: [id: string];
}>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d, locale, n } = useI18n();
const localePath = useLocalePath();
const sortKey = computed(() => queryString(route.query.sort));
const sortDirection = computed(() =>
  queryString(route.query.direction) === "desc" ? "desc" : "asc",
);

const visibleCols = computed(() => props.visibleColumns ?? [
  "identifier",
  "full_name",
  "order_identifier",
  "products",
  "created_at",
  "total",
]);

let previewProductsTimer: ReturnType<typeof setTimeout> | undefined;

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

function previewProducts(id: string) {
  clearTimeout(previewProductsTimer);
  previewProductsTimer = setTimeout(() => {
    emits("listDeliveryNoteProducts", id);
  }, 400);
}

const cancelPreviewProducts = () => clearTimeout(previewProductsTimer);
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead v-if="visibleCols.includes('identifier')" class="w-24" />
          <TableHead v-if="visibleCols.includes('full_name')">
            <TableSortHeader
              :label="t('fields.full-name')"
              :active="sortKey === 'full_name'"
              :direction="sortDirection"
              @click="toggleSort('full_name')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('order_identifier')">
            <TableSortHeader
              :label="t('fields.order')"
              :active="sortKey === 'order_identifier'"
              :direction="sortDirection"
              @click="toggleSort('order_identifier')"
            />
          </TableHead>
          <TableHead v-if="visibleCols.includes('products')">
            <TableSortHeader
              :label="t('fields.items')"
              :active="sortKey === 'products'"
              :direction="sortDirection"
              @click="toggleSort('products')"
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
          v-for="(deliveryNote, index) in props.deliveryNotes"
          :key="deliveryNote.id"
          v-fade="index"
          :class="{
            'animate-highlight-row':
              deliveryNote.id === $route.query.id && $route.query.highlight === 'true',
          }"
        >
          <TableCell v-if="visibleCols.includes('identifier')" class="p-2 text-nowrap font-medium">
            {{ deliveryNote.identifier }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('full_name')" class="p-2 font-medium">
            {{ deliveryNote.full_name }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('order_identifier')" class="p-2">
            <NuxtLink
              :to="localePath({ path: '/orders/', query: { page: 1, search: deliveryNote.order_identifier } })"
              class="inline-flex items-center gap-2 underline decoration-slate-300 underline-offset-4"
            >
              <FileText class="size-4 text-slate-500" />
              {{ deliveryNote.order_identifier }}
            </NuxtLink>
          </TableCell>
          <TableCell v-if="visibleCols.includes('products')" class="p-2">
            <Popover v-if="deliveryNote.products && deliveryNote.products > 0">
              <PopoverTrigger as-child>
                <Button
                  size="sm"
                  variant="link"
                  class="underline px-0 h-fit text-nowrap"
                  @mouseenter.passive="previewProducts(deliveryNote.id)"
                  @mouseleave.passive="cancelPreviewProducts"
                >
                  {{ `${deliveryNote.products} ${t("plrz.p", { n: Math.ceil(deliveryNote.products) })}` }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[13rem] p-2">
                <ScrollArea :class="deliveryNoteProducts.length > 16 ? 'h-[380px]' : 'h-fit'">
                  <table class="w-full not-default">
                    <tbody>
                      <tr v-for="(product, i) in deliveryNoteProducts" :key="i" class="text-sm">
                        <td>{{ product.name }}</td>
                        <td class="text-slate-700 text-end">
                          <i>x{{ product.quantity }}</i>
                        </td>
                        <td class="text-nowrap text-end">
                          {{ n(product.price, "decimal") }}
                          <span class="text-xs text-slate-700">MAD</span>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </ScrollArea>
              </PopoverContent>
            </Popover>
            <template v-else>
              {{ `${deliveryNote.products} ${t("plrz.p", { n: Math.ceil(deliveryNote.products ?? 0) })}` }}
            </template>
          </TableCell>
          <TableCell v-if="visibleCols.includes('created_at')" class="p-2">
            {{ d(new Date(deliveryNote.created_at), "long") }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('total')" class="p-2">
            {{ n(toNumber(deliveryNote.total), "currency") }}
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem>
                    <Printer :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.print") }}
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </TableCell>
        </TableRow>
        <TableEmpty v-if="!props.deliveryNotes.length" :colspan="visibleCols.length + 1">
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
