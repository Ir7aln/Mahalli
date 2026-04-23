<script setup lang="ts">
import { ChevronLeft, ChevronRight } from "lucide-vue-next";
import { commands } from "@/bindings";

const activeDb = ref<string | null>(null);

const { data } = await useAsyncData("active-database", async () => {
  const result = await commands.getActiveDatabase();
  if (result.status === "ok") return result.data.data?.name ?? null;
  return null;
});

activeDb.value = data.value ?? null;
</script>

<template>
  <header class="flex-1 sticky border-b border-slate-100 top-0 z-50">
    <div class="w-full h-full flex items-center justify-between py-3 px-2 bg-white">
      <div class="text-black flex items-center justify-center gap-2 rtl:flex-row-reverse">
        <ChevronLeft :size="20" class="cursor-pointer" @click="$router.back()" />
        <ChevronRight :size="20" class="cursor-pointer" @click="$router.forward()" />
      </div>

      <div
        v-if="activeDb"
        class="flex items-center gap-2 rtl:flex-row-reverse border border-slate-200 bg-slate-50 rounded-full px-3 py-1"
      >
        <span class="relative flex h-2.5 w-2.5 shrink-0">
          <span
            class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"
          />
          <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-green-500" />
        </span>
        <span class="text-sm font-medium text-slate-700">{{ activeDb }}</span>
      </div>
    </div>
  </header>
</template>
