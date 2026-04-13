import { useMotion } from "@vueuse/motion";
import type { DirectiveBinding } from "vue";

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive("fade", {
    mounted: (el: HTMLElement, bin: DirectiveBinding) => {
      useMotion(el, {
        initial: {
          opacity: 0,
        },
        enter: {
          opacity: 1,
          transition: {
            delay: bin.value * 20,
          },
        },
      });
    },
  });
});
