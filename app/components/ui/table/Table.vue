<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "@/utils/shadcn";

const props = defineProps<{
  class?: HTMLAttributes["class"];
}>();

const wrapperRef = ref<HTMLElement | null>(null);
const isScrollable = ref(false);

function checkScrollable() {
  if (wrapperRef.value) {
    isScrollable.value = wrapperRef.value.scrollWidth > wrapperRef.value.clientWidth;
  }
}

let ro: ResizeObserver | null = null;

onMounted(() => {
  ro = new ResizeObserver(checkScrollable);
  if (wrapperRef.value) ro.observe(wrapperRef.value);
  checkScrollable();
});

onUnmounted(() => ro?.disconnect());
</script>

<template>
  <div
    ref="wrapperRef"
    class="relative w-full overflow-auto"
    :class="{ 'is-x-scrollable': isScrollable }"
  >
    <table :class="cn('min-w-full caption-bottom text-sm', props.class)">
      <slot />
    </table>
  </div>
</template>
