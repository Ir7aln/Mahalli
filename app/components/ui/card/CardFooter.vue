<script setup lang="ts">
import type { HTMLAttributes } from "vue";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <div :class="cn('grid grid-cols-3 gap-3 items-center p-4 pt-0', props.class)">
    <slot />
  </div>
</template>
