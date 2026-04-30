<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import type {
  CreditNoteResponse,
  SelectDeliveryNotes,
  SelectInventory,
  SelectInvoices,
  SelectOrders,
  SelectQuotes,
} from "@/bindings";

const { t, n, d } = useI18n();
const localePath = useLocalePath();
const { showErrorToast } = useCommandError();

const { data: financials } = await useAsyncData("home-financials", async () => {
  const result = await commands.listFinancialMetrics();
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST FINANCIAL METRICS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: invoicesData } = await useAsyncData("home-invoices", async () => {
  const result = await commands.listInvoices({
    page: 1,
    limit: 5,
    search: "",
    status: null,
    created_from: null,
    created_to: null,
    sort: "created_at",
    direction: "desc",
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST INVOICES HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: ordersData } = await useAsyncData("home-orders", async () => {
  const result = await commands.listOrders({
    page: 1,
    limit: 5,
    search: "",
    status: null,
    created_from: null,
    created_to: null,
    sort: "created_at",
    direction: "desc",
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST ORDERS HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: quotesData } = await useAsyncData("home-quotes", async () => {
  const result = await commands.listQuotes({
    page: 1,
    limit: 5,
    search: "",
    status: null,
    created_from: null,
    created_to: null,
    sort: "created_at",
    direction: "desc",
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST QUOTES HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: deliveryNotesData } = await useAsyncData("home-delivery-notes", async () => {
  const result = await commands.listDeliveryNotes({
    page: 1,
    limit: 5,
    search: "",
    status: null,
    created_from: null,
    created_to: null,
    sort: "created_at",
    direction: "desc",
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST DELIVERY NOTES HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: creditNotesData } = await useAsyncData("home-credit-notes", async () => {
  const result = await commands.listCreditNotes({
    limit: 5,
    offset: 0,
    search: "",
    sort: "created_at",
    direction: "desc",
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST CREDIT NOTES HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const { data: inventoryData } = await useAsyncData("home-inventory", async () => {
  const result = await commands.listInventory({
    page: 1,
    limit: 5,
    search: "",
    transaction_type: null,
    source_type: null,
    created_from: null,
    created_to: null,
    quantity_min: null,
    quantity_max: null,
    price_min: null,
    price_max: null,
    sort: "created_at",
    direction: "desc",
    include_voided: false,
  });
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR LIST INVENTORY HOME: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

const recentInvoices = computed<SelectInvoices[]>(() => invoicesData.value?.invoices ?? []);
const recentOrders = computed<SelectOrders[]>(() => ordersData.value?.orders ?? []);
const recentQuotes = computed<SelectQuotes[]>(() => quotesData.value?.quotes ?? []);
const recentDeliveryNotes = computed<SelectDeliveryNotes[]>(
  () => deliveryNotesData.value?.delivery_notes ?? [],
);
const recentCreditNotes = computed<CreditNoteResponse[]>(() => creditNotesData.value?.notes ?? []);
const recentInventoryMovements = computed<SelectInventory[]>(
  () => inventoryData.value?.inventory ?? [],
);

const summaryCards = computed(() => [
  {
    label: t("dashboard.revenue"),
    value: n(toNumber(financials.value?.current_revenue), "currency"),
    sub: t("dashboard.growth", {
      n: n(financials.value?.revenue_growth_percentage || 0, { style: "percent" }),
    }),
  },
  {
    label: t("dashboard.expenses"),
    value: n(toNumber(financials.value?.current_expenses), "currency"),
    sub: t("dashboard.growth", {
      n: n(financials.value?.expenses_growth_percentage || 0, { style: "percent" }),
    }),
  },
  {
    label: t("routes.invoices"),
    value: String(invoicesData.value?.count ?? 0),
    sub: t("tables.empty.description"),
  },
  {
    label: t("routes.orders"),
    value: String(ordersData.value?.count ?? 0),
    sub: t("tables.empty.description"),
  },
]);

const quickActions = [
  { label: "routes.clients", path: "/clients/", query: { page: 1, limit: 50 } },
  { label: "routes.products", path: "/products/", query: { page: 1 } },
  { label: "routes.invoices", path: "/invoices/", query: { page: 1 } },
  { label: "routes.orders", path: "/orders/", query: { page: 1 } },
  { label: "routes.delivery-notes", path: "/delivery-notes/", query: { page: 1 } },
  { label: "routes.settings", path: "/settings", query: {} },
];
</script>

<template>
  <main class="w-full p-4 md:p-6 space-y-4 bg-slate-50/40 min-h-[calc(100vh-70px)]">
    <section class="grid gap-2 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-6">
      <NuxtLink
        v-for="action in quickActions"
        :key="action.path"
        :to="localePath({ path: action.path, query: action.query })"
        class="rounded-sm border border-slate-200 bg-white px-3 py-2 text-xs font-medium uppercase tracking-wide text-slate-700 hover:bg-slate-100 transition-colors text-center whitespace-normal break-words min-h-10 flex items-center justify-center"
      >
        {{ t(action.label) }}
      </NuxtLink>
    </section>

    <section class="grid gap-2 md:grid-cols-2 xl:grid-cols-4">
      <div
        v-for="card in summaryCards"
        :key="card.label"
        class="rounded-sm border border-slate-200 bg-white px-4 py-3"
      >
        <p class="text-xs text-slate-500 uppercase tracking-wide">{{ card.label }}</p>
        <div class="pt-1">
          <p class="text-3xl font-semibold leading-none text-slate-900">{{ card.value }}</p>
          <p class="text-[11px] text-slate-500 mt-2">{{ card.sub }}</p>
        </div>
      </div>
    </section>

    <section class="rounded-sm border border-slate-200 bg-white">
      <div class="border-b border-slate-200 px-4 py-3">
        <h2 class="text-base font-semibold text-slate-900">{{ t("dashboard.title") }}</h2>
      </div>
      <div class="p-4">
        <div class="grid gap-3 sm:grid-cols-2">
          <div class="border border-slate-200 rounded-sm p-4 bg-white">
            <p class="text-xs uppercase tracking-wide text-slate-500">
              {{ t("dashboard.revenue") }}
            </p>
            <p class="text-2xl mt-2 font-semibold text-emerald-700">
              {{ n(toNumber(financials?.current_revenue), "currency") }}
            </p>
            <p class="text-xs text-slate-500 mt-2">
              {{
                t("dashboard.growth", {
                  n: n(financials?.revenue_growth_percentage || 0, { style: "percent" }),
                })
              }}
            </p>
          </div>
          <div class="border border-slate-200 rounded-sm p-4 bg-white">
            <p class="text-xs uppercase tracking-wide text-slate-500">
              {{ t("dashboard.expenses") }}
            </p>
            <p class="text-2xl mt-2 font-semibold text-rose-700">
              {{ n(toNumber(financials?.current_expenses), "currency") }}
            </p>
            <p class="text-xs text-slate-500 mt-2">
              {{
                t("dashboard.growth", {
                  n: n(financials?.expenses_growth_percentage || 0, { style: "percent" }),
                })
              }}
            </p>
          </div>
        </div>
      </div>
    </section>

    <div class="grid gap-4 xl:grid-cols-3">
      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.invoices") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="invoice in recentInvoices"
                :key="invoice.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <NuxtLink
                    :to="localePath(`/invoices/?page=1&highlight=true&id=${invoice.id}`)"
                    class="font-medium hover:underline"
                  >
                    {{ invoice.identifier }}
                  </NuxtLink>
                  <p class="text-xs text-slate-500">{{ invoice.full_name }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ n(toNumber(invoice.total), "currency") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.orders") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="order in recentOrders"
                :key="order.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <NuxtLink
                    :to="localePath(`/orders/?page=1&highlight=true&id=${order.id}`)"
                    class="font-medium hover:underline"
                  >
                    {{ order.identifier }}
                  </NuxtLink>
                  <p class="text-xs text-slate-500">{{ order.full_name }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ d(new Date(order.created_at!), "short") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.quotes") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="quote in recentQuotes"
                :key="quote.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <NuxtLink
                    :to="localePath(`/quotes/?page=1&highlight=true&id=${quote.id}`)"
                    class="font-medium hover:underline"
                  >
                    {{ quote.identifier }}
                  </NuxtLink>
                  <p class="text-xs text-slate-500">{{ quote.full_name }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ n(toNumber(quote.total), "currency") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.delivery-notes") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="note in recentDeliveryNotes"
                :key="note.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <NuxtLink
                    :to="localePath(`/delivery-notes/?page=1&highlight=true&id=${note.id}`)"
                    class="font-medium hover:underline"
                  >
                    {{ note.identifier }}
                  </NuxtLink>
                  <p class="text-xs text-slate-500">{{ note.full_name }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ n(toNumber(note.total), "currency") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.credit-notes") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="note in recentCreditNotes"
                :key="note.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <NuxtLink
                    :to="localePath(`/credit-notes/?page=1&highlight=true&id=${note.id}`)"
                    class="font-medium hover:underline"
                  >
                    {{ note.identifier || "-" }}
                  </NuxtLink>
                  <p class="text-xs text-slate-500">{{ note.full_name }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ n(toNumber(note.total), "currency") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <section class="overflow-hidden rounded-sm border border-slate-200 bg-white">
        <div class="border-b border-slate-200 px-4 py-3">
          <h3 class="text-sm font-semibold text-slate-900">{{ t("routes.inventory") }}</h3>
        </div>
        <div class="px-4 py-2 overflow-x-auto">
          <table class="w-full text-sm">
            <tbody>
              <tr
                v-for="movement in recentInventoryMovements"
                :key="movement.id"
                class="border-b last:border-b-0 hover:bg-slate-50"
              >
                <td class="py-3">
                  <p class="font-medium truncate max-w-[12rem]">{{ movement.name }}</p>
                  <p class="text-[11px] text-slate-500">{{ movement.source_identifier || "-" }}</p>
                </td>
                <td class="py-3 text-right text-xs text-slate-500">
                  {{ t(`status.${movement.transaction_type.toLowerCase()}`) }} ·
                  {{ n(movement.quantity, "decimal") }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
  </main>
</template>
