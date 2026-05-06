<script setup lang="ts">
import { TabsTrigger as TabsTriggerPrimitive } from "radix-vue";

interface Props {
  value: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  disabled: false,
});
</script>

<template>
  <TabsTriggerPrimitive
    :value="value"
    :disabled="disabled"
    class="inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-white transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-white data-[state=active]:text-slate-950 data-[state=active]:shadow-sm"
  >
    <slot />
  </TabsTriggerPrimitive>
</template>
