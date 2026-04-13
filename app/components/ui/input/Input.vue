<script setup lang="ts">
import { useVModel } from "@vueuse/core";

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
}>();
const emits = defineEmits<{
  (e: "update:modelValue", payload: string | number): void;
}>();
const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>

<template>
  <div class="relative">
    <input
      v-bind="$attrs"
      v-model="modelValue"
      step="0.01"
      class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none relative h-10 block w-full disabled:cursor-not-allowed disabled:opacity-50 focus:outline-none border-0 form-input rounded-md placeholder-gray-400 dark:placeholder-gray-500 text-sm px-2.5 py-1.5 shadow-sm bg-white dark:bg-gray-900 text-gray-900 dark:text-white ring-1 ring-inset ring-input dark:ring-gray-700 focus:ring-2 focus:ring-black dark:focus:ring-primary-400 pe-9"
    >
    <span
      v-if="$slots.unite"
      class="absolute inset-y-0 end-0 flex items-center pointer-events-none px-2.5"
    >
      <span
        :class="
          cn(
            'text-slate-500 dark:text-gray-400 text-sm',
            $attrs.disabled ? 'opacity-50' : '',
          )
        "
      >
        <slot name="unite" />
      </span>
    </span>
  </div>
</template>
