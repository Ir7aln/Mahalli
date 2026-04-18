<script setup lang="ts">
import { ArrowDown, ArrowUp, ArrowUpDown } from "lucide-vue-next";
import type { SortDirection } from "@/utils/sort";

const props = defineProps<{
  active?: boolean;
  direction?: SortDirection;
  label: string;
}>();

const emit = defineEmits<{
  (e: "click"): void;
}>();
</script>

<template>
  <Button
    type="button"
    variant="ghost"
    size="sm"
    class="h-auto -ml-2 px-2 py-1 text-left font-semibold text-slate-700 hover:text-slate-950"
    :class="props.active ? 'bg-slate-100 text-slate-950' : ''"
    @click="emit('click')"
  >
    <span class="inline-flex items-center gap-2">
      <span>{{ props.label }}</span>
      <ArrowUpDown v-if="!props.active" :size="14" class="opacity-60" />
      <ArrowUp v-else-if="props.direction === 'asc'" :size="14" />
      <ArrowDown v-else :size="14" />
    </span>
  </Button>
</template>
