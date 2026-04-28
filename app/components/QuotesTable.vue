<script setup lang="ts">
import { commands } from "@/bindings";
import { FilePenLine, GripHorizontal, Printer, Trash2, Truck } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { NuxtLink, QuoteDelete, QuoteUpdate } from "#components";
import type { QuoteProductItem, SelectQuotes } from "@/bindings";
import { queryString } from "@/utils/query";

const props = defineProps<{
  quotes: SelectQuotes[];
  quoteProducts: QuoteProductItem[];
  visibleColumns?: string[];
}>();
const emits = defineEmits<{
  listQuoteProducts: [id: string];
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

const visibleCols = computed(
  () =>
    props.visibleColumns ?? ["identifier", "full_name", "status", "products", "created_at", "total"],
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
    emits("listQuoteProducts", id);
  }, 400);
}
const cancelPreviewProducts = () => clearTimeout(previewProductsTimer);

function toggleThisQuote(quote: SelectQuotes, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(QuoteDelete, {
      id: quote.id,
      identifier: quote.identifier,
    });
  } else {
    modal.open(QuoteUpdate, {
      sheet: true,
      id: quote.id,
      identifier: quote.identifier,
    });
  }
}

async function createOrderFromQuote(id: string) {
  const result = await commands.createOrderFromQuote(id);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`GET QUOTE FOR ORDER: ${JSON.stringify(result.error)}`);
    return;
  }
  Logger.info(`CREATE ORDER FROM QUOTE: ${id}`);
  updateQueryParams({
    refresh: `refresh-update-${Math.random() * 9999}`,
  });
  toast.success(t("notifications.order.created"), {
    closeButton: true,
    description: h(NuxtLink, {
      to: localePath(`/orders/?page=1&highlight=true&id=${result.data.data}`),
      class: "underline",
      innerHTML: t("buttons.go-to-order"),
    }),
  });
}
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
          <TableHead v-if="visibleCols.includes('status')">
            <TableSortHeader
              :label="t('fields.status')"
              :active="sortKey === 'status'"
              :direction="sortDirection"
              @click="toggleSort('status')"
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
        <TableRow v-for="(quote, index) in props.quotes" :key="quote.id" v-fade="index">
          <TableCell v-if="visibleCols.includes('identifier')" class="p-2 text-nowrap font-medium">
            {{ quote.identifier }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('full_name')" class="p-2 font-medium">
            <Popover>
              <PopoverTrigger as-child>
                <Button variant="link" class="underline px-0 h-fit text-nowrap">
                  {{ quote.full_name }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[18rem] p-3">
                <div class="space-y-3">
                  <div>
                    <p class="text-xs text-muted-foreground">{{ t("fields.full-name") }}</p>
                    <p class="text-sm font-medium">{{ quote.full_name }}</p>
                  </div>
                  <div
                    v-if="quote.ice || quote.if_number || quote.rc || quote.patente"
                    class="border-t pt-2"
                  >
                    <p class="text-xs text-muted-foreground mb-2">
                      {{ t("fields.legal-identifiers") }}
                    </p>
                    <div class="space-y-1 text-sm">
                      <div v-if="quote.ice">
                        <span class="text-xs text-slate-500">ICE:</span>
                        <span class="font-mono">{{ quote.ice }}</span>
                      </div>
                      <div v-if="quote.if_number">
                        <span class="text-xs text-slate-500">IF:</span>
                        <span class="font-mono">{{ quote.if_number }}</span>
                      </div>
                      <div v-if="quote.rc">
                        <span class="text-xs text-slate-500">RC:</span>
                        <span class="font-mono">{{ quote.rc }}</span>
                      </div>
                      <div v-if="quote.patente">
                        <span class="text-xs text-slate-500">{{ t("fields.patente") }}:</span>
                        <span class="font-mono">{{ quote.patente }}</span>
                      </div>
                    </div>
                  </div>
                  <div
                    v-if="quote.email || quote.phone_number || quote.address"
                    class="border-t pt-2"
                  >
                    <p class="text-xs text-muted-foreground mb-2">{{ t("fields.contact") }}</p>
                    <div class="space-y-1 text-sm">
                      <div v-if="quote.email">
                        <span class="text-xs text-slate-500">{{ t("fields.email") }}:</span>
                        <span>{{ quote.email }}</span>
                      </div>
                      <div v-if="quote.phone_number">
                        <span class="text-xs text-slate-500">{{ t("fields.phone") }}:</span>
                        <span>{{ quote.phone_number }}</span>
                      </div>
                      <div v-if="quote.address">
                        <span class="text-xs text-slate-500">{{ t("fields.address") }}:</span>
                        <span>{{ quote.address }}</span>
                      </div>
                    </div>
                  </div>
                </div>
              </PopoverContent>
            </Popover>
          </TableCell>
          <TableCell v-if="visibleCols.includes('status')" class="p-2">
            <Badge
              variant="outline"
              :class="
                cn(
                  'whitespace-nowrap',
                  quote.status === 'ACCEPTED'
                    ? 'bg-green-100 border-green-500 text-green-900'
                    : quote.status === 'CANCELLED'
                      ? 'bg-red-100 border-red-500 text-red-900'
                      : 'bg-blue-100 border-blue-500 text-blue-900',
                )
              "
            >
              {{ t(`status.${quote.status.toLowerCase()}`) }}
            </Badge>
          </TableCell>
          <TableCell v-if="visibleCols.includes('products')" class="p-2">
            <Popover v-if="quote.products && quote.products > 0">
              <PopoverTrigger as-child>
                <Button
                  size="sm"
                  variant="link"
                  class="underline px-0 h-fit text-nowrap"
                  @mouseenter.passive="previewProducts(quote.id!)"
                  @mouseleave.passive="cancelPreviewProducts"
                >
                  {{
                    `${quote.products} ${t("plrz.p", {
                      n: Math.ceil(quote.products),
                    })}`
                  }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[13rem] p-2">
                <ScrollArea :class="quoteProducts.length > 16 ? 'h-[380px]' : 'h-fit'">
                  <table class="w-full not-default">
                    <thead>
                      <tr>
                        <th v-for="i in 3" :key="i" />
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(product, i) in quoteProducts" :key="i" class="text-sm">
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
                `${quote.products} ${t("plrz.p", {
                  n: Math.ceil(quote?.products ?? 0),
                })}`
              }}
            </template>
          </TableCell>
          <TableCell v-if="visibleCols.includes('created_at')" class="p-2">
            {{ d(new Date(quote.created_at!), "long") }}
          </TableCell>
          <TableCell v-if="visibleCols.includes('total')" class="p-2">
            {{ n(toNumber(quote.total), "currency") }}
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center items-center gap-3">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem @click="toggleThisQuote(quote, 'update')">
                    <FilePenLine :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.edit") }}
                  </DropdownMenuItem>
                  <DropdownMenuItem>
                    <NuxtLink
                      :to="
                        localePath({
                          path: `/quotes/${quote.id}`,
                        })
                      "
                    >
                      <Printer :size="20" class="text-slate-800 inline mr-2" />{{
                        t("buttons.print")
                      }}
                    </NuxtLink>
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="createOrderFromQuote(quote.id!)">
                    <Truck :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.to-order") }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="toggleThisQuote(quote, 'delete')">
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
        <TableEmpty v-if="!props.quotes.length" :colspan="visibleCols.length + 1">
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
