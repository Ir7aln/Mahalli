<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { FilePenLine, GripHorizontal, NotepadText, Printer, Trash2 } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { NuxtLink, OrderDelete, OrderUpdate } from "#components";
import { ORDER_STATUSES, STATUS_COLORS } from "@/consts";
import type { OrderProductItem, SelectOrders } from "@/bindings";
import { queryString } from "@/utils/query";

const props = defineProps<{ orders: SelectOrders[]; orderProducts: OrderProductItem[] }>();
const emits = defineEmits<{
  listOrderProducts: [id: string];
}>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const modal = useModal();
const { t, d, locale, n } = useI18n();
const { showErrorToast } = useCommandError();
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

let previewProductsTimer: any;
function previewProducts(id: string) {
  clearTimeout(previewProductsTimer);
  previewProductsTimer = setTimeout(() => {
    emits("listOrderProducts", id);
  }, 400);
}
const cancelPreviewProducts = () => clearTimeout(previewProductsTimer);

function toggleThisOrder(order: SelectOrders, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(OrderDelete, {
      id: order.id,
      identifier: order.identifier,
    });
  } else {
    modal.open(OrderUpdate, {
      sheet: true,
      id: order.id,
      identifier: order.identifier,
    });
  }
}

async function updateOrderStatus(id: string, status: string) {
  const result = await commands.updateOrderStatus({ id, status });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR UPDATE ORDER STATUS: ${JSON.stringify(result.error)}`);
    return;
  }
  Logger.info(`UPDATE ORDER STATUS: ${JSON.stringify({ id, status })}`);
  updateQueryParams({
    refresh: `refresh-update-${Math.random() * 9999}`,
  });
}

async function createInvoiceFromOrder(id: string) {
  const result = await commands.createInvoiceFromOrder(id);
  if (result.status === "error") {
    Logger.error(`GET ORDER FOR INVOICE: ${JSON.stringify(result.error)}`);
    showErrorToast(result.error);
    return;
  }
  Logger.info(`CREATE INVOICE FROM ORDER: ${id}`);
  toast.success(t("notifications.invoice.created"), {
    closeButton: true,
    description: h(NuxtLink, {
      to: localePath(`/invoices/?page=1&highlight=true&id=${result.data.data}`),
      class: "underline",
      innerHTML: "go to invoice",
    }),
  });
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead class="w-24" />
          <TableHead>
            <TableSortHeader
              :label="t('fields.full-name')"
              :active="sortKey === 'full_name'"
              :direction="sortDirection"
              @click="toggleSort('full_name')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.items')"
              :active="sortKey === 'products'"
              :direction="sortDirection"
              @click="toggleSort('products')"
            />
          </TableHead>
          <TableHead class="w-fit">
            <TableSortHeader
              :label="t('fields.status')"
              :active="sortKey === 'status'"
              :direction="sortDirection"
              @click="toggleSort('status')"
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
          <TableHead>
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
          v-for="(order, index) in props.orders"
          :key="order.id"
          v-fade="index"
          :class="{
            'animate-highlight-row':
              order.id === $route.query.id && $route.query.highlight === 'true',
          }"
        >
          <TableCell class="p-2 text-nowrap font-medium">
            {{ order.identifier }}
          </TableCell>
          <TableCell class="p-2 font-medium">
            {{ order.full_name }}
          </TableCell>
          <TableCell class="p-2">
            <Popover v-if="order.products && order.products > 0">
              <PopoverTrigger as-child>
                <Button
                  size="sm"
                  variant="link"
                  class="underline px-0 h-fit text-nowrap"
                  @mouseenter.passive="previewProducts(order.id!)"
                  @mouseleave.passive="cancelPreviewProducts"
                >
                  {{
                    `${order.products} ${t("plrz.p", {
                      n: Math.ceil(order.products),
                    })}`
                  }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[13rem] p-2">
                <ScrollArea :class="orderProducts.length > 16 ? 'h-[380px]' : 'h-fit'">
                  <table class="w-full not-default">
                    <thead>
                      <tr>
                        <th v-for="i in 3" :key="i" />
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(product, i) in orderProducts" :key="i" class="text-sm">
                        <td>
                          {{ product.name }}
                        </td>
                        <td class="text-slate-700 text-end">
                          <i> x{{ product.quantity }} </i>
                        </td>
                        <td class="text-nowrap text-end">
                          {{ n(product.price, "decimal") }}
                          <span class="text-xs text-slate-700"> MAD </span>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </ScrollArea>
              </PopoverContent>
            </Popover>
            <template v-else>
              {{
                `${order.products} ${t("plrz.p", {
                  n: Math.ceil(order.products ?? 0),
                })}`
              }}
            </template>
          </TableCell>
          <TableCell class="p-2">
            <Popover>
              <PopoverTrigger as-child>
                <Badge
                  variant="outline"
                  :class="cn('cursor-pointer whitespace-nowrap', STATUS_COLORS[order.status])"
                >
                  {{ t(`status.${order.status.toLowerCase()}`) }}
                </Badge>
              </PopoverTrigger>
              <PopoverContent class="w-40 p-1 flex flex-col gap-1">
                <Button
                  v-for="status in ORDER_STATUSES"
                  :key="status"
                  type="button"
                  variant="secondary"
                  size="sm"
                  :class="cn('border', STATUS_COLORS[status])"
                  @click="() => updateOrderStatus(order.id as string, status)"
                >
                  {{ t(`status.${status.toLowerCase()}`) }}
                </Button>
              </PopoverContent>
            </Popover>
          </TableCell>
          <TableCell class="p-2">
            {{ d(new Date(order.created_at!), "long") }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(toNumber(order.total), "currency") }}
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center items-center gap-3">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem @click="toggleThisOrder(order, 'update')">
                    <FilePenLine :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.edit") }}
                  </DropdownMenuItem>
                  <DropdownMenuItem>
                    <NuxtLink
                      :to="
                        localePath({
                          path: `/orders/${order.id}`,
                        })
                      "
                    >
                      <Printer :size="20" class="text-slate-800 inline mr-2" />{{
                        t("buttons.print")
                      }}
                    </NuxtLink>
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="createInvoiceFromOrder(order.id!)">
                    <NotepadText :size="20" class="text-slate-800 inline mr-2" />{{
                      t("buttons.to-invoice")
                    }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="toggleThisOrder(order, 'delete')">
                    <Trash2 :size="20" class="text-red-500 inline mr-2" />
                    <span class="text-red-500">
                      {{ t("buttons.delete") }}
                    </span>
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </TableCell>
        </TableRow>
        <TableEmpty v-if="!props.orders.length" :colspan="7">
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


