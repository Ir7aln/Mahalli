<script setup lang="ts">
import { type HTMLAttributes, computed } from "vue";
import {
  DropdownMenuSubTrigger,
  type DropdownMenuSubTriggerProps,
  useForwardProps,
} from "radix-vue";
import { ChevronRight } from "lucide-vue-next";
import { useI18n } from "vue-i18n";

const { locale } = useI18n();

const props = defineProps<
  DropdownMenuSubTriggerProps & { class?: HTMLAttributes["class"]; dir?: "ltr" | "rtl" }
>();

const delegatedProps = computed(() => {
  const { class: _, dir: _dir, ...delegated } = props;
  return delegated;
});

const forwardedProps = useForwardProps(delegatedProps);
const direction = computed(() => props.dir || (locale.value === "ar" ? "rtl" : "ltr"));
</script>

<template>
  <DropdownMenuSubTrigger
    v-bind="forwardedProps"
    :class="
      cn(
        'flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none focus:bg-accent data-[state=open]:bg-accent',
        props.class,
      )
    "
  >
    <slot />
    <ChevronRight
      :class="[
        'h-4 w-4 ms-auto shrink-0 transition-transform',
        direction === 'rtl' && 'rotate-180',
      ]"
    />
  </DropdownMenuSubTrigger>
</template>
