<script setup lang="ts">
import { onClickOutside } from "@vueuse/core";
import { Check } from "lucide-vue-next";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Input } from "@/components/ui/input";

const props = defineProps<{
  defaultValue?: string;
  items: {
    label: string;
    value: string;
    price?: number;
  }[];
}>();
const emits = defineEmits<{
  updateItems: [s: string | number];
  onSelect: [s: string, a?: number];
}>();
const { t } = useI18n();
const open = ref(false);
const dropdownParent = ref<HTMLElement | null>(null);
const inputValue = ref<string | number>(props.defaultValue ?? "");

let timer: any;
function updateInput(s: string | number) {
  clearTimeout(timer);
  timer = setTimeout(() => {
    emits("updateItems", s);
    open.value = true;
  }, 500);
}

onClickOutside(dropdownParent, () => (open.value = false));

function selectItem(item: { label: string; value: string; price?: number }) {
  inputValue.value = item.label;
  emits("onSelect", item.value, item.price);
  open.value = false;
}
</script>

<template>
  <div ref="dropdownParent">
    <Popover :open="open">
      <PopoverTrigger class="w-full" aria-disabled="true">
        <Input
          v-model:model-value="inputValue"
          :placeholder="t('search')"
          @update:model-value="updateInput"
        />
      </PopoverTrigger>
      <PopoverContent class="p-1" @open-auto-focus="(e) => e.preventDefault()">
        <ScrollArea
          :class="cn('space-y-1', items.length > 7 ? 'h-60' : 'h-fit')"
        >
          <div
            v-for="item in items"
            :key="item.value"
            class="relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
            @click="() => selectItem(item)"
          >
            <span
              v-if="defaultValue === item.label"
              class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center"
            >
              <Check :size="16" />
            </span>
            <span>
              {{ item.label }}
            </span>
          </div>
          <div
            v-if="!items.length"
            class="relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 px-2 text-sm outline-none"
          >
            {{ t("fields.no-match") }}
          </div>
        </ScrollArea>
      </PopoverContent>
    </Popover>
  </div>
</template>
