<script setup lang="ts">
import { FilePenLine, GripHorizontal, Trash2 } from "lucide-vue-next";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ClientDelete, ClientUpdate } from "#components";
import type { ClientInvoiceDebtItem, SelectClients } from "@/bindings";
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { queryString } from "@/utils/query";

const props = defineProps<{
  clients: SelectClients[];
}>();
const route = useRoute();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { t, locale, n } = useI18n();
const localePath = useLocalePath();
const modal = useModal();
const clientInvoiceDebts = ref<ClientInvoiceDebtItem[]>([]);
const debtCache = reactive<Record<string, ClientInvoiceDebtItem[]>>({});
const sortKey = computed(() => queryString(route.query.sort));
const sortDirection = computed(() =>
  queryString(route.query.direction) === "desc" ? "desc" : "asc",
);

let previewDebtsTimer: ReturnType<typeof setTimeout> | undefined;

function toggleSort(key: string) {
  const currentKey = sortKey.value;
  const currentDirection = sortDirection.value;

  if (currentKey !== key) {
    updateQueryParams({ sort: key, direction: "asc", page: 1 });
    return;
  }

  if (currentDirection === "asc") {
    updateQueryParams({ direction: "desc", page: 1 });
    return;
  }

  updateQueryParams({ sort: "", direction: "", page: 1 });
}

async function listClientInvoiceDebts(clientId?: string) {
  if (!clientId) {
    clientInvoiceDebts.value = [];
    return;
  }

  if (debtCache[clientId]) {
    clientInvoiceDebts.value = debtCache[clientId];
    return;
  }

  clientInvoiceDebts.value = [];
  const result = await commands.listClientInvoiceDebts(clientId);
  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR LIST CLIENT INVOICE DEBTS: ${JSON.stringify(result.error)}`);
    return;
  }

  const debts = result.data.data ?? [];
  debtCache[clientId] = debts;
  clientInvoiceDebts.value = debts;
}

function previewDebts(clientId: string) {
  clearTimeout(previewDebtsTimer);
  previewDebtsTimer = setTimeout(() => {
    void listClientInvoiceDebts(clientId);
  }, 400);
}

const cancelPreviewDebts = () => clearTimeout(previewDebtsTimer);

function toggleThisClient(client: SelectClients, name: "delete" | "update") {
  if (name === "delete") {
    modal.open(ClientDelete, {
      id: client.id,
      fullName: client.full_name,
    });
  } else {
    modal.open(ClientUpdate, {
      id: client.id,
      fullName: client.full_name,
      email: client.email ?? undefined,
      phoneNumber: client.phone_number ?? undefined,
      address: client.address ?? undefined,
    });
  }
}
</script>

<template>
  <div class="w-full pb-16">
    <Table :dir="locale === 'ar' ? 'rtl' : 'ltr'">
      <TableHeader>
        <TableRow>
          <TableHead class="w-14" />
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
              :label="t('fields.email')"
              :active="sortKey === 'email'"
              :direction="sortDirection"
              @click="toggleSort('email')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.phone')"
              :active="sortKey === 'phone_number'"
              :direction="sortDirection"
              @click="toggleSort('phone_number')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.address')"
              :active="sortKey === 'address'"
              :direction="sortDirection"
              @click="toggleSort('address')"
            />
          </TableHead>
          <TableHead>
            <TableSortHeader
              :label="t('fields.credit')"
              :active="sortKey === 'credit'"
              :direction="sortDirection"
              @click="toggleSort('credit')"
            />
          </TableHead>
          <TableHead class="w-20 sticky ltr:right-0 rtl:left-0 bg-gray-100 z-10">
            {{ t("fields.actions") }}
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="(client, index) in props.clients" :key="client.id" v-fade="index">
          <TableCell class="p-2 flex justify-center">
            <Avatar>
              <AvatarImage v-if="client.image" :src="convertFileSrc(client.image)" />
              <AvatarFallback class="text-xs">
                {{ client.full_name.substring(0, 5) }}
              </AvatarFallback>
            </Avatar>
          </TableCell>
          <TableCell class="p-2 whitespace-nowrap font-medium">
            {{ client?.full_name }}
          </TableCell>
          <TableCell class="p-2">
            {{ client.email || "--" }}
          </TableCell>
          <TableCell class="p-2">
            {{ client.phone_number || "--" }}
          </TableCell>
          <TableCell class="p-2">
            {{ client.address || "--" }}
          </TableCell>
          <TableCell class="p-2 whitespace-nowrap">
            <Popover v-if="toNumber(client.credit) > 0">
              <PopoverTrigger as-child>
                <Button
                  variant="link"
                  class="px-0 h-fit underline text-nowrap"
                  @mouseenter.passive="previewDebts(client.id)"
                  @mouseleave.passive="cancelPreviewDebts"
                >
                  {{ n(toNumber(client.credit), "currency") }}
                </Button>
              </PopoverTrigger>
              <PopoverContent class="min-w-[18rem] p-2">
                <ScrollArea :class="clientInvoiceDebts.length > 6 ? 'h-[260px]' : 'h-fit'">
                  <table class="w-full not-default">
                    <thead>
                      <tr>
                        <th v-for="i in 2" :key="i" />
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="invoice in clientInvoiceDebts" :key="invoice.id" class="text-sm">
                        <td class="py-1">
                          <NuxtLink
                            :to="
                              localePath({
                                path: '/invoices/',
                                query: { page: 1, search: invoice.identifier },
                              })
                            "
                            class="font-medium text-slate-900 underline decoration-slate-400 underline-offset-4"
                          >
                            {{ invoice.identifier }}
                          </NuxtLink>
                        </td>
                        <td class="py-1 text-nowrap text-end italic text-red-600">
                          -{{
                            n(toNumber(invoice.total) - toNumber(invoice.paid_amount), "currency")
                          }}
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </ScrollArea>
              </PopoverContent>
            </Popover>
            <template v-else>
              {{ n(toNumber(client.credit), "currency") }}
            </template>
          </TableCell>
          <TableCell class="p-2 sticky ltr:right-0 rtl:left-0 bg-background z-10">
            <div class="flex justify-center">
              <DropdownMenu>
                <DropdownMenuTrigger>
                  <GripHorizontal class="text-slate-800 inline" />
                </DropdownMenuTrigger>
                <DropdownMenuContent class="rtl:ml-6 ltr:mr-6">
                  <DropdownMenuItem @click="toggleThisClient(client, 'update')">
                    <FilePenLine :size="20" class="text-slate-800 inline mr-2" />
                    {{ t("buttons.edit") }}
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="toggleThisClient(client, 'delete')">
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
