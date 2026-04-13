import type { InjectionKey } from "vue";

export const FORM_ITEM_INJECTION_KEY = Symbol(
  "form_item_injection_key"
) as InjectionKey<string>;
