<script setup lang="ts">
import type { HTMLAttributes } from "vue";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();
</script>

<template>
  <h3
    :class="
      cn(
        'font-semibold text-lg text-gray-900 uppercase text-center leading-none tracking-tight',
        props.class,
      )
    "
  >
    <slot />
  </h3>
</template>
