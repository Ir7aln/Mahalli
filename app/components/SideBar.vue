<script setup lang="ts">
import {
  ArrowLeftRight,
  BarChart3Icon,
  HomeIcon,
  Languages,
  Package,
  Quote,
  ReceiptText,
  SquareUser,
  Store,
  Truck,
} from "lucide-vue-next";
import { TranslationModal } from "#components";

const { t, locale } = useI18n();
const localePath = useLocalePath();
const modal = useModal();
const LOCALE_TEXT = {
  en: "English",
  fr: "Francais",
  ar: "العربية",
  de: "Deutsch",
} as const;
const openTranslationModal = () => modal.open(TranslationModal, {});
const menuItems = [
  { icon: HomeIcon, label: "routes.home", path: "/" },
  {
    icon: SquareUser,
    label: "routes.clients",
    path: "/clients/",
    query: { page: 1, limit: 50 },
  },
  {
    icon: Package,
    label: "routes.products",
    path: "/products/",
    query: { page: 1 },
  },
  {
    icon: Quote,
    label: "routes.quotes",
    path: "/quotes/",
    query: { page: 1 },
  },
  {
    icon: Truck,
    label: "routes.orders",
    path: "/orders/",
    query: { page: 1 },
  },
  {
    icon: ReceiptText,
    label: "routes.invoices",
    path: "/invoices/",
    query: { page: 1 },
  },
  {
    icon: ArrowLeftRight,
    label: "routes.inventory",
    path: "/inventory/",
    query: { page: 1 },
  },
  { icon: BarChart3Icon, label: "routes.dashboard", path: "/dashboard" },
];
</script>

<template>
  <aside
    class="w-12 sticky h-screen top-0 border-x border-slate-100 z-50 flex justify-center bg-white"
  >
    <div class="h-screen flex flex-col gap-1">
      <div class="min-h-12 border-b border-slate-100 flex justify-center">
        <Store :size="20" class="text-primary m-auto" />
      </div>
      <div class="px-1 flex flex-col justify-between h-full pb-2">
        <div class="h-full flex flex-col gap-1">
          <NuxtLink
            v-for="item in menuItems"
            :key="item.path"
            class="isolate flex flex-nowrap size-9 rounded-md items-center p-1 group transition-all duration-300 relative hover:bg-gray-50"
            :to="localePath({ path: item.path, query: item.query })"
          >
            <component
              :is="item.icon"
              class="m-auto text-gray-600 transition-colors duration-300 group-hover:text-primary"
              :size="20"
            />
            <div
              class="absolute invisible opacity-0 group-hover:visible group-hover:opacity-100 ltr:left-full rtl:right-full w-fit ltr:ml-2 rtl:mr-2 bg-white border border-gray-200 rounded-md shadow-md px-3 py-2 whitespace-nowrap transition-all duration-300 pointer-events-none ltr:-translate-x-2 rtl:translate-x-2 group-hover:translate-x-0 ltr:origin-left rtl:origin-right"
            >
              <span class="text-sm font-medium text-gray-700">{{
                t(item.label)
              }}</span>
            </div>
          </NuxtLink>
        </div>
        <div
          class="isolate cursor-pointer flex size-9 rounded-md items-center p-1 group transition-all duration-300 relative hover:bg-gray-50"
          @click="openTranslationModal"
        >
          <Languages
            class="m-auto text-gray-700 transition-colors duration-300 group-hover:text-primary"
            :size="20"
          />
          <div
            class="absolute invisible opacity-0 group-hover:visible group-hover:opacity-100 ltr:left-full rtl:right-full w-fit ltr:ml-2 rtl:mr-2 bg-white border border-gray-200 rounded-md shadow-md px-3 py-2 whitespace-nowrap transition-all duration-300 pointer-events-none ltr:-translate-x-2 rtl:translate-x-2 group-hover:translate-x-0 ltr:origin-left rtl:origin-right"
          >
            <span class="text-sm font-medium text-gray-700">{{
              LOCALE_TEXT[locale]
            }}</span>
          </div>
        </div>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.router-link-active {
  @apply bg-slate-50 transition-colors rounded-md transform duration-200;
}

.router-link-active > svg {
  @apply text-black;
}
</style>
