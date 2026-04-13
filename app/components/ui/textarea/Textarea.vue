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
  <textarea
    v-model="modelValue"
    :class="
      cn(
        'min-h-[80px] h-10 block w-full disabled:cursor-not-allowed disabled:opacity-50 focus:outline-none border-0 form-input rounded-md placeholder-gray-400 dark:placeholder-gray-500 text-sm px-2.5 py-2 shadow-sm bg-white dark:bg-gray-900 text-gray-900 dark:text-white ring-1 ring-inset ring-input dark:ring-gray-700 focus:ring-2 focus:ring-black dark:focus:ring-primary-400 pe-9',
        ($attrs.class as string) ?? '',
      )
    "
  />
</template>
