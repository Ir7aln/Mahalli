import { useMotion } from "@vueuse/motion";
import type { DirectiveBinding } from "vue";

const animatedElements = new WeakSet<HTMLElement>();

export default defineNuxtPlugin((nuxtApp) => {
  nuxtApp.vueApp.directive("fade", {
    mounted: (el: HTMLElement, bin: DirectiveBinding) => {
      if (animatedElements.has(el)) return;

      animatedElements.add(el);
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
