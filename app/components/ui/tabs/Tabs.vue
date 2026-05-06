<script setup lang="ts">
import { TabsRoot } from "radix-vue";

interface Props {
  defaultValue?: string;
  value?: string;
  orientation?: "horizontal" | "vertical";
}

withDefaults(defineProps<Props>(), {
  orientation: "horizontal",
});

defineEmits<{
  "update:value": [value: string];
}>();
</script>

<template>
  <TabsRoot v-bind="$props" @update:value="$emit('update:value', $event)">
    <slot />
  </TabsRoot>
</template>
