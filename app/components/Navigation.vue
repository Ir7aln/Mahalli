<script setup lang="ts">
import { ChevronLeft, ChevronRight } from "lucide-vue-next";
</script>

<template>
  <header class="flex-1 sticky border-b border-slate-100 top-0 z-50">
    <div class="w-full h-full flex items-center py-3 px-2 bg-white">
      <div
        class="text-black flex items-center justify-center gap-2 rtl:flex-row-reverse"
      >
        <ChevronLeft
          :size="20"
          class="cursor-pointer"
          @click="$router.back()"
        />
        <ChevronRight
          :size="20"
          class="cursor-pointer"
          @click="$router.forward()"
        />
      </div>
    </div>
  </header>
</template>
