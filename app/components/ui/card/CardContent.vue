<script setup lang="ts">
import type { HTMLAttributes } from "vue";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <div
    :class="cn('p-4 flex flex-col gap-2 pt-4 flex-1 space-y-2', props.class)"
  >
    <slot />
  </div>
</template>
