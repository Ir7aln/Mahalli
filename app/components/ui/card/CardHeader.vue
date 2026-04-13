<script setup lang="ts">
import type { HTMLAttributes } from "vue";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <div
    :class="
      cn('flex flex-col border-b-2 border-primary gap-y-1.5 p-4', props.class)
    "
  >
    <slot />
  </div>
</template>
