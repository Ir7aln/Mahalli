<script setup lang="ts">
import { commands, type QuoteWithClient } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";

const props = defineProps<{
  id: string;
  identifier: string;
}>();

const { close } = useModal();
const { t, d, n } = useI18n();
const quote = ref<QuoteWithClient | null>(null);
const isLoading = ref(true);

onMounted(async () => {
  const getResult = await commands.getQuote(props.id);
  if (getResult.status === "ok" && getResult.data.data) {
    quote.value = getResult.data.data as unknown as QuoteWithClient;
  } else {
    Logger.error(`ERROR GET QUOTE: ${JSON.stringify(getResult.error)}`);
  }
  isLoading.value = false;
});

const subtotal = computed(() => {
  if (!quote.value?.items) return 0;
  return quote.value.items.reduce((sum, item) => {
    return sum + Number(item.quantity ?? 0) * Number(item.price ?? 0);
  }, 0);
});

const clientDetails = computed(() => {
  if (!quote.value) return [];
  return [
    quote.value.email || t("placeholders.no-email"),
    quote.value.address || t("placeholders.no-address"),
    quote.value.phone_number || t("placeholders.no-phone"),
  ];
});

function formatMoney(value: number | string) {
  return n(toNumber(value), "currency");
}
</script>

<template>
  <div class="h-full w-full max-w-[860px]">
    <div
      class="flex h-full w-full flex-col overflow-hidden border-s border-slate-200 bg-white text-slate-900 shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-slate-200 px-5 py-4">
        <div class="space-y-1">
          <p class="text-xs font-medium text-slate-500">
            {{ t("routes.quotes") }}
          </p>
          <h2 class="text-xl font-semibold">
            {{ t("titles.quotes.view") }} {{ props.identifier }}
          </h2>
        </div>
        <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
          <X class="size-5" />
        </Button>
      </div>

      <div class="min-h-0 flex-1 overflow-y-auto">
        <div v-if="isLoading" class="flex h-full w-full items-center justify-center">
          <p class="text-slate-500">{{ t("loading") }}</p>
        </div>
        <div v-else-if="quote" class="w-full h-full px-5 py-6 sm:px-6">
          <section class="border border-slate-200 bg-white px-6 py-6 sm:px-7">
            <div
              class="flex flex-col gap-6 border-b border-slate-200 pb-6 sm:flex-row sm:items-start sm:justify-between"
            >
              <div>
                <p class="text-3xl font-semibold tracking-tight">
                  {{ t("fields.quote") }}
                </p>
                <p class="mt-2 text-sm text-slate-500">
                  {{ props.identifier }}
                </p>
              </div>
              <div class="text-left sm:text-right">
                <p class="text-xs text-slate-500">
                  {{ t("fields.total") }}
                </p>
                <p class="mt-2 text-3xl font-semibold tracking-tight">
                  {{ formatMoney(subtotal) }}
                </p>
              </div>
            </div>

            <div class="space-y-6 border-b border-slate-200 py-6">
              <div class="space-y-2 text-sm">
                <span class="text-slate-500">{{ t("fields.date") }}</span>
                <div class="flex items-center h-9 font-medium text-slate-900">
                  {{ quote.created_at ? d(new Date(quote.created_at), "long") : "--" }}
                </div>
              </div>

              <div class="space-y-4">
                <div class="space-y-2">
                  <p class="text-xs font-semibold text-slate-500">
                    {{ t("fields.bill-to") }}
                  </p>
                </div>

                <div class="space-y-1 text-sm leading-6 text-slate-500">
                  <p class="text-base font-semibold text-slate-900">
                    {{ quote.full_name }}
                  </p>
                  <p v-for="detail in clientDetails" :key="detail">
                    {{ detail }}
                  </p>
                </div>
              </div>
            </div>

            <div class="py-6">
              <p class="text-xs font-semibold text-slate-500 pb-4">
                {{ t("fields.items") }}
              </p>

              <div class="border-y border-slate-200">
                <div
                  class="hidden bg-slate-50 px-4 py-3 text-xs font-semibold text-slate-500 md:grid md:grid-cols-[minmax(0,2fr)_100px_140px_120px] md:gap-3"
                >
                  <span>{{ t("fields.name") }}</span>
                  <span>{{ t("fields.quantity") }}</span>
                  <span>{{ t("fields.price") }}</span>
                  <span>{{ t("fields.total") }}</span>
                </div>

                <div class="divide-y divide-slate-200">
                  <div
                    v-for="item in quote.items"
                    :key="item.id"
                    class="grid gap-3 px-4 py-4 md:grid-cols-[minmax(0,2fr)_100px_140px_120px] md:items-end"
                  >
                    <div class="space-y-1">
                      <p class="text-xs font-semibold text-slate-500 md:hidden">
                        {{ t("fields.name") }}
                      </p>
                      <p class="text-sm font-medium text-slate-900">
                        {{ item.name }}
                      </p>
                    </div>

                    <div class="space-y-1">
                      <p class="text-xs font-semibold text-slate-500 md:hidden">
                        {{ t("fields.quantity") }}
                      </p>
                      <p class="text-sm font-medium text-slate-900">
                        {{ item.quantity }}
                      </p>
                    </div>

                    <div class="space-y-1">
                      <p class="text-xs font-semibold text-slate-500 md:hidden">
                        {{ t("fields.price") }}
                      </p>
                      <p class="text-sm font-medium text-slate-900">
                        {{ formatMoney(item.price) }}
                      </p>
                    </div>

                    <div class="space-y-1">
                      <p class="text-xs font-semibold text-slate-500 md:hidden">
                        {{ t("fields.total") }}
                      </p>
                      <div class="text-sm font-medium text-slate-900">
                        {{ formatMoney(Number(item.quantity ?? 0) * Number(item.price ?? 0)) }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <div class="ml-auto mt-6 w-full max-w-sm space-y-3 border-t border-slate-200 pt-4">
                <div class="flex items-center justify-between text-base font-semibold">
                  <span>{{ t("fields.total") }}</span>
                  <span>{{ formatMoney(subtotal) }}</span>
                </div>
              </div>
            </div>
          </section>
        </div>
      </div>

      <div class="border-t border-slate-200 px-5 py-4">
        <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
          <div class="text-sm text-slate-500">
            {{ quote?.full_name || t("placeholders.no-client-selected") }}
          </div>
          <Button type="button" variant="outline" @click="close">
            {{ t("buttons.close") }}
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
