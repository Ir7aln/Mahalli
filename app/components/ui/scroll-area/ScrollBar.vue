<script setup lang="ts">
import { type HTMLAttributes, computed } from "vue";
import { ScrollAreaScrollbar, type ScrollAreaScrollbarProps, ScrollAreaThumb } from "radix-vue";

const props = withDefaults(
  defineProps<ScrollAreaScrollbarProps & { class?: HTMLAttributes["class"] }>(),
  {
    orientation: "vertical",
  },
);

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props;

  return delegated;
});
</script>

<template>
  <ScrollAreaScrollbar
    v-bind="delegatedProps"
    :class="
      cn(
        'flex touch-none select-none rounded-full bg-slate-100 transition-colors',
        orientation === 'vertical' && 'h-full w-1.5 p-0',
        orientation === 'horizontal' && 'h-1.5 flex-col p-0',
        props.class,
      )
    "
  >
    <ScrollAreaThumb
      class="relative flex-1 rounded-full bg-slate-300 hover:bg-[rgb(184,197,212)]"
    />
  </ScrollAreaScrollbar>
</template>
