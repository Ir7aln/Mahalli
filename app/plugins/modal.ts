import { shallowRef } from "vue";
import { modalInjectionKey } from "@/composables/useModal";
import type { ModalState } from "@/composables/useModal";

export default defineNuxtPlugin((nuxtApp) => {
  const modalState = shallowRef<ModalState>({
    component: "div",
    props: {},
  });

  nuxtApp.vueApp.provide(modalInjectionKey, modalState);
});
