<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {
  FilePenLine,
  GripHorizontal,
  Printer,
  Trash2,
  Truck,
} from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { NuxtLink, QuoteDelete, QuoteUpdate } from "#components";

defineProps<{ quotes: QuoteT[]; quoteProducts: QuoteProductsPreviewT[] }>();
const emits = defineEmits<{
  listQuoteProducts: [id: string];
}>();
const modal = useModal();
const { t, d, locale, n } = useI18n();
const localePath = useLocalePath();

let previewProductsTimer: any;
function previewProducts(id: string) {
  clearTimeout(previewProductsTimer);
  previewProductsTimer = setTimeout(() => {
    emits("listQuoteProducts", id);
  }, 400);
}
const cancelPreviewProducts = () => clearTimeout(previewProductsTimer);

function toggleThisQuote(quote: QuoteT, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(QuoteDelete, {
      id: quote.id!,
      identifier: quote.identifier,
    });
  } else {
    modal.open(QuoteUpdate, {
      id: quote.id!,
      identifier: quote.identifier,
    });
  }
}

async function createOrderFromQuote(id: string) {
  try {
    const res = await invoke<Res<QuoteForUpdateT>>("create_order_from_quote", {
      id,
    });
    Logger.info(`CREATE ORDER FROM QUOTE: ${id}`);
    //
    toast.success(t("notifications.order.created"), {
      closeButton: true,
      description: h(NuxtLink, {
        to: localePath(`/orders/?page=1&highlight=true&id=${res.data}`),
        class: "underline",
        innerHTML: "go to order",
      }),
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`GET QUOTE FOR ORDER: ${err.error ? err.error : err.message}`);
  }
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead class="w-24" />
          <TableHead>{{ t("fields.full-name") }}</TableHead>
          <TableHead>{{ t("fields.items") }}</TableHead>
          <TableHead class="w-56">
            {{ t("fields.date") }}
          </TableHead>
          <TableHead>{{ t("fields.total") }}</TableHead>
          <TableHead class="w-20">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(quote, index) in quotes"
          :key="quote.id"
          v-fade="index"
        >
          <TableCell class="p-2 text-nowrap font-medium">
            {{ quote.identifier }}
          </TableCell>
          <TableCell class="p-2 font-medium">
            {{ quote.full_name }}
          </TableCell>
          <TableCell class="p-2">
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
                <ScrollArea
                  :class="quoteProducts.length > 16 ? 'h-[380px]' : 'h-fit'"
                >
                  <table class="w-full not-default">
                    <thead>
                      <tr>
                        <th v-for="i in 3" :key="i" />
                      </tr>
                    </thead>
                    <tbody>
                      <tr
                        v-for="(product, i) in quoteProducts"
                        :key="i"
                        class="text-sm"
                      >
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
          <TableCell class="p-2">
            {{ d(new Date(quote.created_at!), "long") }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(quote.total!, "currency") }}
          </TableCell>
          <TableCell class="p-2">
            <div class="flex justify-center items-center gap-3">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem @click="toggleThisQuote(quote, 'update')">
                    <FilePenLine
                      :size="20"
                      class="text-slate-800 inline mr-2"
                    />
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
                      <Printer
                        :size="20"
                        class="text-slate-800 inline mr-2"
                      />{{ t("buttons.print") }}
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
      </TableBody>
    </Table>
    <Pagination />
  </div>
</template>
