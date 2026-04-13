<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { FilePenLine, GripHorizontal, Printer, Trash2 } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { InvoiceDelete, InvoiceUpdate } from "#components";
import { INVOICE_STATUSES, STATUS_COLORS } from "@/consts";

defineProps<{
  invoices: InvoiceT[];
  invoiceProducts: InvoiceProductsPreviewT[];
}>();
const emits = defineEmits<{
  listInvoiceProducts: [id: string];
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const modal = useModal();
const { t, d, locale, n } = useI18n();
const localePath = useLocalePath();

let previewProductsTimer: any;
function previewProducts(id: string) {
  clearTimeout(previewProductsTimer);
  previewProductsTimer = setTimeout(() => {
    emits("listInvoiceProducts", id);
  }, 400);
}
const cancelPreviewProducts = () => clearTimeout(previewProductsTimer);

function toggleThisInvoice(invoice: InvoiceT, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(InvoiceDelete, {
      id: invoice.id!,
      identifier: invoice.identifier,
    });
  } else {
    modal.open(InvoiceUpdate, {
      id: invoice.id!,
      identifier: invoice.identifier,
    });
  }
}

async function updateInvoiceStatus(id: string, status: string) {
  try {
    await invoke("update_invoice_status", {
      invoice: {
        id,
        status,
      },
    });
    //
    Logger.info(
      `UPDATE INVOICE STATUS: ${JSON.stringify({
        id,
        status,
      })}`
    );
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-update-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR UPDATE INVOICE STATUS: ${err.error ? err.error : err.message}`
    );
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
          <TableHead class="w-fit">
            {{ t("fields.status") }}
          </TableHead>
          <TableHead class="w-56">
            {{ t("fields.date") }}
          </TableHead>
          <TableHead>{{ t("fields.total") }}</TableHead>
          <TableHead>{{ t("fields.paid") }}</TableHead>
          <TableHead class="w-20">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(invoice, index) in invoices"
          :key="invoice.id"
          v-fade="index"
          :class="{
            'animate-highlight-row':
              invoice.id === $route.query.id
              && $route.query.highlight === 'true',
          }"
        >
          <TableCell class="p-2 text-nowrap font-medium">
            {{ invoice.identifier }}
          </TableCell>
          <TableCell class="p-2 font-medium">
            {{ invoice.full_name }}
          </TableCell>
          <TableCell class="p-2">
            <Popover v-if="invoice.products && invoice.products > 0">
              <PopoverTrigger as-child>
                <Button
                  size="sm"
                  variant="link"
                  class="underline px-0 h-fit text-nowrap"
                  @mouseenter.passive="previewProducts(invoice.id!)"
                  @mouseleave.passive="cancelPreviewProducts"
                >
                  {{
                    `${invoice.products} ${t("plrz.p", {
                      n: Math.ceil(invoice.products),
                    })}`
                  }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[13rem] p-2">
                <ScrollArea
                  :class="invoiceProducts.length > 16 ? 'h-[380px]' : 'h-fit'"
                >
                  <table class="w-full not-default">
                    <thead>
                      <tr>
                        <th v-for="i in 3" :key="i" />
                      </tr>
                    </thead>
                    <tbody>
                      <tr
                        v-for="(product, i) in invoiceProducts"
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
                `${invoice.products} ${t("plrz.p", {
                  n: Math.ceil(invoice.products ?? 0),
                })}`
              }}
            </template>
          </TableCell>
          <TableCell class="p-2">
            <Popover>
              <PopoverTrigger as-child>
                <Badge
                  variant="outline"
                  :class="
                    cn(
                      'cursor-pointer whitespace-nowrap',
                      STATUS_COLORS[invoice?.status!],
                    )
                  "
                >
                  {{ t(`status.${invoice.status.toLowerCase()}`) }}
                </Badge>
              </PopoverTrigger>
              <PopoverContent class="w-40 p-1 flex flex-col gap-1">
                <Button
                  v-for="status in INVOICE_STATUSES"
                  :key="status"
                  type="button"
                  variant="secondary"
                  size="sm"
                  :class="cn('border', STATUS_COLORS[status])"
                  @click="() => updateInvoiceStatus(invoice.id as string, status)"
                >
                  {{ t(`status.${status.toLowerCase()}`) }}
                </Button>
              </PopoverContent>
            </Popover>
          </TableCell>
          <TableCell class="p-2">
            {{ d(new Date(invoice.created_at!), "long") }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(invoice.total!, "currency") }}
          </TableCell>
          <TableCell class="p-2">
            {{ n(invoice.paid_amount, "currency") }}
          </TableCell>
          <TableCell class="p-2">
            <div class="flex justify-center gap-3">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem
                    @click="toggleThisInvoice(invoice, 'update')"
                  >
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
                          path: `/invoices/${invoice.id}`,
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
                  <DropdownMenuItem
                    @click="toggleThisInvoice(invoice, 'delete')"
                  >
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
