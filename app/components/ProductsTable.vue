<script setup lang="ts">
import {
  CalendarDays,
  FilePenLine,
  GripHorizontal,
  Info,
  PackagePlus,
  Trash2,
} from "lucide-vue-next";
import { convertFileSrc } from "@tauri-apps/api/core";
import { InventoryUpdate, ProductDelete, ProductUpdate } from "#components";
import type { SelectProducts } from "@/bindings";
import { queryString } from "@/utils/query";

const props = defineProps<{ products: SelectProducts[] }>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, d, locale, n } = useI18n();
const modal = useModal();
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

function toggleThisProduct(product: SelectProducts, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(ProductDelete, {
      id: product.id,
      name: product.name,
    });
  } else {
    modal.open(ProductUpdate, {
      id: product.id,
      name: product.name,
      purchasePrice: product.purchase_price ?? 0,
      sellingPrice: product.selling_price ?? 0,
      description: product.description ?? undefined,
      minQuantity: product.min_quantity ?? 0,
    });
  }
}

function updateProductInventory(id: string, name: string) {
  modal.open(InventoryUpdate, {
    id,
    name,
  });
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead class="w-14" />
          <TableHead class="w-20">
            <TableSortHeader
              :label="t('fields.name')"
              :active="sortKey === 'name'"
              :direction="sortDirection"
              @click="toggleSort('name')"
            />
          </TableHead>
          <TableHead class="w-fit">
            <TableSortHeader
              :label="t('fields.inventory')"
              :active="sortKey === 'inventory'"
              :direction="sortDirection"
              @click="toggleSort('inventory')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.threshold')"
              :active="sortKey === 'min_quantity'"
              :direction="sortDirection"
              @click="toggleSort('min_quantity')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.purchase-price')"
              :active="sortKey === 'purchase_price'"
              :direction="sortDirection"
              @click="toggleSort('purchase_price')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.selling-price')"
              :active="sortKey === 'selling_price'"
              :direction="sortDirection"
              @click="toggleSort('selling_price')"
            />
          </TableHead>
          <TableHead class="w-20 sticky ltr:right-0 rtl:left-0 bg-gray-100 z-10">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(product, index) in props.products" :key="product.id" v-fade="index">
          <TableCell class="p-2 flex justify-center">
            <Avatar>
              <AvatarImage v-if="product.image" :src="convertFileSrc(product.image)" />
              <AvatarFallback class="text-xs">
                {{ product.name.substring(0, 5) }}
              </AvatarFallback>
            </Avatar>
          </TableCell>
          <TableCell class="p-2">
            <span class="whitespace-nowrap flex justify-between gap-3">
              {{ product.name }}
              <HoverCard>
                <HoverCardTrigger as-child>
                  <Info class="cursor-pointer text-gray-800" :size="20" />
                </HoverCardTrigger>
                <HoverCardContent class="w-80">
                  <div class="flex justify-between space-x-4">
                    <div class="space-y-1">
                      <h4 class="text-sm font-semibold">
                        {{ product.name }}
                      </h4>
                      <p class="text-sm">
                        {{ product.description || "--" }}
                      </p>
                      <div class="flex items-center pt-2">
                        <CalendarDays class="mr-2 h-4 w-4 opacity-70" />
                        <span class="text-xs text-muted-foreground">
                          Created at {{ d(product.created_at!, "short") }}
                        </span>
                      </div>
                    </div>
                  </div>
                </HoverCardContent>
              </HoverCard>
            </span>
          </TableCell>
          <TableCell class="p-2">
            <Badge
              variant="outline"
              :class="
                cn(
                  'whitespace-nowrap',
                  product.inventory !== undefined
                    ? product?.inventory <= 0
                      ? 'bg-red-100 border-red-500 text-red-900'
                      : product?.inventory < (product.min_quantity ?? 0)
                        ? 'bg-yellow-100 border-yellow-500 text-yellow-900'
                        : product?.inventory >= (product.min_quantity ?? 0)
                          ? 'bg-green-100 border-green-500 text-green-900'
                          : ''
                    : '',
                )
              "
            >
              {{
                `${product?.inventory} ${t("plrz.i", {
                  n: Math.ceil(product?.inventory ?? 0),
                })}`
              }}
            </Badge>
          </TableCell>
          <TableCell class="p-2">
            {{
              `${product.min_quantity ?? 0} ${t("plrz.i", {
                n: Math.ceil(product.min_quantity ?? 0),
              })}`
            }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(toNumber(product.purchase_price), "currency") }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(toNumber(product.selling_price), "currency") }}
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center gap-3">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem @click="toggleThisProduct(product, 'update')">
                    <FilePenLine class="text-slate-800 inline mr-2" :size="20" />
                    {{ t("buttons.edit") }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="updateProductInventory(product.id, product.name)">
                    <PackagePlus :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.inventory-update") }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="toggleThisProduct(product, 'delete')">
                    <Trash2 class="text-red-500 inline mr-2" :size="20" />
                    <span>
                      <span class="text-red-500">
                        {{ t("buttons.delete") }}
                      </span>
                    </span>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </TableCell>
        </TableRow>
        <TableEmpty v-if="!props.products.length" :colspan="7">
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
