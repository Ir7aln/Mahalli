<script setup lang="ts">
const { t } = useI18n();
const localePath = useLocalePath();
const pages = ref([
  {
    image: "/clients.jpg",
    text: "clients",
    path: "/clients/",
  },
  {
    image: "/products.jpg",
    text: "products",
    path: "/products",
  },
  // {
  //   image: "/suppliers.jpeg",
  //   text: "Suppliers",
  //   path: "/suppliers/",
  // },
  {
    image: "/devis.webp",
    text: "quotes",
    path: "/quotes/",
  },
  {
    image: "/orders.jpg",
    text: "orders",
    path: "/orders/",
  },
  {
    image: "/invoices.jpg",
    text: "invoices",
    path: "/invoices/",
  },
  {
    image: "/inventory.jpg",
    text: "inventory",
    path: "/inventory",
  },
  {
    image: "/dashboard.jpg",
    text: "dashboard",
    path: "/dashboard",
  },
]);
</script>

<template>
  <div class="w-full h-[calc(100vh-80px)]">
    <div class="w-full h-full flex justify-center items-center flex-col">
      <div
        class="md:h-[700px] m-auto w-full max-w-3xl h-full p-3 gap-3 grid-cols-3 grid-rows-3 grid"
      >
        <ClientOnly>
          <div
            v-for="(page, index) in pages"
            :key="index"
            v-fade="index + 1"
            class="w-full h-full"
          >
            <NuxtLink
              :to="
                localePath({ path: page.path, query: { page: 1, limit: 50 } })
              "
            >
              <div
                class="w-full h-full overflow-hidden cursor-pointer relative hover:-translate-y-1 group transition-all duration-250 flex justify-center rounded-md items-center bg-gray-400"
              >
                <img
                  class="absolute top-0 left-0 w-full object-fill opacity-20 group-hover:opacity-60 h-full transition-all duration-250"
                  :src="page.image"
                >
                <span
                  class="text-center text-xl text-gray-900 font-bold bg-white rounded-md px-4 z-20 py-2 flex justify-center items-center transition-all duration-250"
                >
                  {{ t(`routes.${page.text}`) }}
                </span>
              </div>
            </NuxtLink>
          </div>
        </ClientOnly>
      </div>
    </div>
  </div>
</template>
