<script setup lang="ts">
import { TabsList as TabsListPrimitive } from "radix-vue";
</script>

<template>
  <TabsListPrimitive
    class="inline-flex h-9 items-center justify-center rounded-md bg-slate-100 p-1 text-slate-500"
  >
    <slot />
  </TabsListPrimitive>
</template>
