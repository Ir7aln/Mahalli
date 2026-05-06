<script setup lang="ts">
import { TabsContent as TabsContentPrimitive } from "radix-vue";

interface Props {
  value: string;
}

defineProps<Props>();
</script>

<template>
  <TabsContentPrimitive
    :value="value"
    class="mt-4 space-y-6 ring-offset-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2"
  >
    <slot />
  </TabsContentPrimitive>
</template>
