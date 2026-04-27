<script setup lang="ts">
import { commands } from "@/bindings";
import { GroupedBar } from "@unovis/ts";
import { VisAxis, VisBulletLegend, VisGroupedBar, VisTooltip, VisXYContainer } from "@unovis/vue";
import { DollarSign, NotepadText, Truck } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { INVOICE_STATUSES, ORDER_STATUSES, STATUS_COLORS } from "@/consts";

const { t, d, n } = useI18n();
const { showErrorToast } = useCommandError();
const { data: inventoryTransactions } = useAsyncData(async () => {
  const result = await commands.listInventoryStats();
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    showErrorToast(result.error);
    return {
      result: {},
      transactionLabels: [],
    };
  }
  const res = result.data.data;
  if (!res) {
    return {
      result: {},
      transactionLabels: [],
    };
  }
  const data = res.reduce((acc: any, item: any) => {
    const { created_at: date, transaction_type, quantity, price } = item;
    const created_at = d(new Date(date), "monthOnly");
    if (!acc[created_at]) {
      acc[created_at] = {
        IN: { quantity: 0, price: 0 },
        OUT: { quantity: 0, price: 0 },
      };
    }
    acc[created_at][transaction_type].quantity += quantity;
    acc[created_at][transaction_type].price += price;
    return acc;
  }, {});

  return {
    result: data,
    transactionLabels: [...new Set<string>(Object.keys(data))],
  };
});

const { data: bestClients } = useAsyncData(async () => {
  const result = await commands.listTopClients();
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return [];
  }
  return result.data.data;
});

const { data: bestProducts } = useAsyncData(async () => {
  const result = await commands.listTopProducts();
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return [];
  }
  return result.data.data;
});

const { data: statusCounts } = useAsyncData(async () => {
  const result = await commands.listStatusCount();
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return null;
  }
  const res = result.data.data;
  if (!res) return { orders: {}, invoices: {} };

  const data: {
    orders: Record<string, number>;
    invoices: Record<string, number>;
  } = {
    orders: {},
    invoices: {},
  };

  res.orders.forEach((item: { status: string; status_count: number }) => {
    data.orders[item.status] = item.status_count;
  });

  res.invoices.forEach((item: { status: string; status_count: number }) => {
    data.invoices[item.status] = item.status_count;
  });

  return data;
});

const { data: financials } = useAsyncData(async () => {
  const result = await commands.listFinancialMetrics();
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});
</script>

<template>
  <main class="w-full h-full">
    <div class="w-full h-full flex flex-col lg:grid lg:grid-cols-2 gap-2">
      <div class="grid grid-cols-1 lg:grid-cols-2 col-span-2 gap-2">
        <Card class="h-fit w-full">
          <CardHeader class="flex border-b-0 flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">
              {{ t("dashboard.revenue") }}
            </CardTitle>
            <DollarSign class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent class="pt-0">
            <div class="text-2xl font-bold">
              {{ n(toNumber(financials?.current_revenue), "currency") }}
            </div>
            <p class="text-xs text-muted-foreground">
              {{
                t("dashboard.growth", {
                  n: n(financials?.revenue_growth_percentage || 0, {
                    style: "percent",
                  }),
                })
              }}
            </p>
          </CardContent>
        </Card>
        <Card class="h-fit w-full lg:order-3">
          <CardHeader class="flex border-b-0 flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">
              {{ t("dashboard.expenses") }}
            </CardTitle>
            <DollarSign class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent class="pt-0">
            <div class="text-2xl font-bold">
              {{ n(toNumber(financials?.current_expenses), "currency") }}
            </div>
            <p class="text-xs text-muted-foreground">
              {{
                t("dashboard.growth", {
                  n: n(financials?.expenses_growth_percentage || 0, {
                    style: "percent",
                  }),
                })
              }}
            </p>
          </CardContent>
        </Card>
        <Card class="w-full">
          <CardHeader class="flex flex-row border-b-0 items-center justify-between space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">
              {{ t("routes.orders") }}
            </CardTitle>
            <Truck class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent class="flex flex-row items-end gap-2 py-3">
            <Badge
              v-for="(status, index) in ORDER_STATUSES"
              :key="index"
              variant="secondary"
              :class="cn('rounded-sm h-8 w-full', STATUS_COLORS[status])"
            >
              {{ statusCounts?.orders[status] || 0 }}
              {{ t(`status.${status.toLowerCase()}`) }}
            </Badge>
          </CardContent>
        </Card>
        <Card class="lg:order-4 w-full">
          <CardHeader class="flex flex-row items-center justify-between border-b-0 space-y-0 pb-2">
            <CardTitle class="text-sm font-medium">
              {{ t("routes.invoices") }}
            </CardTitle>
            <NotepadText class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent class="flex flex-row items-end gap-2 py-3">
            <Badge
              v-for="(status, index) in INVOICE_STATUSES"
              :key="index"
              variant="secondary"
              :class="cn('rounded-sm h-8 w-full', STATUS_COLORS[status])"
            >
              {{ statusCounts?.invoices[status] || 0 }}
              {{ t(`status.${status.toLowerCase()}`) }}
            </Badge>
          </CardContent>
        </Card>
      </div>
      <div class="w-full h-fit">
        <LazyChartHolder>
          <template #default>
            <VisXYContainer v-if="bestClients" :data="bestClients" :height="500">
              <VisGroupedBar
                :bar-padding="0.1"
                :x="(d: any, index: number) => index"
                :y="(d: any) => d.price"
              />
              <VisAxis
                type="x"
                :tick-format="(i: number) => (bestClients ? bestClients[i]?.full_name : i)"
              />
              <VisAxis type="y" :label="`${t('fields.price')} (MAD)`" />
              <VisTooltip
                :triggers="{
                  [GroupedBar.selectors.bar]: (d: any) => {
                    return `${n(toNumber(d.price), 'currency')} `;
                  },
                }"
              />
            </VisXYContainer>
          </template>
          <template #title>
            <h1 class="m-2 w-full text-center text-base font-medium">
              <i>{{ t("dashboard.b3c") }}</i>
            </h1>
          </template>
        </LazyChartHolder>
      </div>
      <div class="w-full h-fit">
        <LazyChartHolder>
          <template #default>
            <VisXYContainer v-if="bestProducts" :data="bestProducts" :height="500">
              <VisGroupedBar
                :bar-padding="0.1"
                :x="(d: any, index: number) => index"
                :y="(d: any) => d.quantity"
              />
              <VisAxis
                type="x"
                :tick-format="(i: number) => (bestProducts ? bestProducts[i]?.name : i)"
              />
              <VisAxis type="y" :label="t('fields.quantity')" />
              <VisTooltip
                :triggers="{
                  [GroupedBar.selectors.bar]: (d: any) => {
                    return `${d.name}: ${n(d.quantity, 'decimal')} ${t('plrz.i', {
                      n: Math.ceil(d.quantity),
                    })}`;
                  },
                }"
              />
            </VisXYContainer>
          </template>
          <template #title>
            <h1 class="m-2 w-full text-center text-base font-medium">
              <i>{{ t("dashboard.bp") }}</i>
            </h1>
          </template>
        </LazyChartHolder>
      </div>
      <div class="w-full h-fit">
        <LazyChartHolder>
          <template #default>
            <VisXYContainer
              v-if="inventoryTransactions"
              :data="Object.values(inventoryTransactions.result)"
              :height="500"
            >
              <VisGroupedBar
                :bar-padding="0.1"
                :x="(_: any, index: number) => index"
                :y="[(d: any) => d.IN.quantity, (d: any) => d.OUT.quantity]"
              />
              <VisAxis
                type="x"
                :tick-format="(i: number) => inventoryTransactions?.transactionLabels[i] || ''"
              />
              <VisAxis type="y" :label="t('fields.quantity')" />
              <VisTooltip
                :triggers="{
                  [GroupedBar.selectors.bar]: (d: any, i: number) => {
                    const transaction_type = (i % 2 === 0 ? 'IN' : 'OUT') as 'IN' | 'OUT';
                    const quantity = d[transaction_type].quantity;
                    return `${n(quantity, 'decimal')} ${t('plrz.i', { n: Math.ceil(quantity) })}`;
                  },
                }"
              />
              <VisBulletLegend
                class="my-2 m-auto w-fit"
                :items="
                  ['in', 'out'].map((a) => ({
                    name: t(`status.${a}`),
                  }))
                "
              />
            </VisXYContainer>
          </template>
          <template #title>
            <h1 class="m-2 w-full text-center text-base font-medium">
              <i>{{ t("dashboard.title") }} ({{ t("fields.item") }})</i>
            </h1>
          </template>
        </LazyChartHolder>
      </div>
      <div class="w-full h-fit">
        <LazyChartHolder>
          <template #default>
            <VisXYContainer
              v-if="inventoryTransactions"
              :data="Object.values(inventoryTransactions.result)"
              :height="500"
            >
              <VisGroupedBar
                :bar-padding="0.1"
                :x="(_: any, index: number) => index"
                :y="[(d: any) => d.IN.price, (d: any) => d.OUT.price]"
              />
              <VisAxis
                type="x"
                :tick-format="(i: number) => inventoryTransactions?.transactionLabels[i] || ''"
              />
              <VisAxis type="y" :label="`${t('fields.price')} (MAD)`" />
              <VisTooltip
                :triggers="{
                  [GroupedBar.selectors.bar]: (d: any, i: number) => {
                    const transaction_type = (i % 2 === 0 ? 'IN' : 'OUT') as 'IN' | 'OUT';
                    return `${n(toNumber(d[transaction_type].price), 'currency')} `;
                  },
                }"
              />
              <VisBulletLegend
                class="my-2 m-auto w-fit"
                :items="
                  ['in', 'out'].map((a) => ({
                    name: t(`status.${a}`),
                  }))
                "
              />
            </VisXYContainer>
          </template>
          <template #title>
            <h1 class="m-2 w-full text-center text-base font-medium">
              <i>{{ t("dashboard.title") }} ()</i>
            </h1>
          </template>
        </LazyChartHolder>
      </div>
    </div>
  </main>
</template>
