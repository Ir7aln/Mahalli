<script setup lang="ts">
import { inject } from "vue";
import { modalInjectionKey, useModal } from "@/composables/useModal";

const modalState = inject(modalInjectionKey);
const { isOpen } = useModal();
const { locale } = useI18n();

const isSheet = computed(() => Boolean(modalState?.value?.props?.sheet));
const isRtl = computed(() => locale.value === "ar");

const sheetSlideIn = computed(() =>
  isRtl.value ? "opacity-0 -translate-x-10" : "opacity-0 translate-x-10",
);
const sheetSlideOut = computed(() =>
  isRtl.value ? "opacity-0 -translate-x-10" : "opacity-0 translate-x-10",
);
const sheetOverlayAlign = "justify-end";
</script>

<template>
  <Transition
    enter-active-class="ease-out duration-300"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="ease-in duration-200"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="isOpen"
      :class="
        cn(
          'fixed inset-0 z-50 flex h-full w-full transition-opacity',
          isSheet
            ? `items-stretch ${sheetOverlayAlign} bg-slate-950/45 backdrop-blur-[2px]`
            : 'items-center justify-center bg-gray-200/75',
        )
      "
    >
      <Transition
        :appear="true"
        :enter-active-class="isSheet ? 'ease-out duration-300' : 'delay-100 ease-out duration-300'"
        :enter-from-class="
          isSheet ? sheetSlideIn : 'delay-100 opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95'
        "
        :enter-to-class="
          isSheet ? 'opacity-100 translate-x-0' : 'delay-100 opacity-100 translate-y-0 sm:scale-100'
        "
        :leave-active-class="isSheet ? 'ease-in duration-200' : 'delay-100 ease-in duration-200'"
        :leave-from-class="
          isSheet ? 'opacity-100 translate-x-0' : 'delay-100 opacity-100 translate-y-0 sm:scale-100'
        "
        :leave-to-class="
          isSheet ? sheetSlideOut : 'delay-100 opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95'
        "
      >
        <Modals />
      </Transition>
    </div>
  </Transition>
</template>
