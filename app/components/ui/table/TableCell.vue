<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/utils/shadcn";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <td :class="cn('p-4 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0', props.class)">
    <slot />
  </td>
</template>
