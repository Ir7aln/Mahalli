<script setup lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { Search, SlidersHorizontal, X, ChevronLeft, ChevronRight } from "lucide-vue-next";

type ActiveFilter = {
  key: string;
  label: string;
  value: string;
};

const props = withDefaults(
  defineProps<{
    search: string;
    activeFilters?: ActiveFilter[];
    searchPlaceholder?: string;
  }>(),
  {
    activeFilters: () => [],
    searchPlaceholder: "",
  },
);

const emit = defineEmits<{
  (e: "update:search", value: string): void;
  (e: "clear-filter", key: string): void;
  (e: "clear-all"): void;
}>();

const { t, locale } = useI18n();

const dir = computed(() => (locale.value === "ar" ? "rtl" : "ltr"));
</script>

<template>
  <div class="mb-3 flex w-full flex-wrap items-center justify-between gap-2" :dir="dir">
    <div class="flex flex-wrap items-center gap-1">
      <DropdownMenu :dir="dir">
        <div class="relative w-full sm:w-auto sm:max-w-sm">
          <Input
            :model-value="search"
            type="text"
            :dir="dir"
            :class="$slots.advanced ? 'pe-9' : ''"
            :placeholder="searchPlaceholder || t('search')"
            @update:model-value="(value) => emit('update:search', String(value))"
          >
            <template #prefix>
              <Search class="size-4" />
            </template>
          </Input>

          <DropdownMenuTrigger v-if="$slots.advanced" as-child>
            <button
              type="button"
              :class="[
                'absolute end-2.5 top-1/2 z-10 -translate-y-1/2 transition-opacity duration-300',
                activeFilters.length > 0 ? 'opacity-100' : 'opacity-40 hover:opacity-100',
              ]"
            >
              <SlidersHorizontal class="size-4" />
            </button>
          </DropdownMenuTrigger>
        </div>

        <DropdownMenuContent
          v-if="$slots.advanced"
          :dir="dir"
          align="start"
          class="min-w-48 [&_[data-radix-vue-dropdown-menu-sub-trigger]]:flex [&_[data-radix-vue-dropdown-menu-sub-trigger]]:items-center [&_[data-radix-vue-dropdown-menu-sub-trigger]]:gap-2 [&_[data-radix-vue-dropdown-menu-sub-trigger]>svg]:ms-auto [&_[data-radix-vue-dropdown-menu-sub-trigger]>svg]:me-0 rtl:[&_[data-radix-vue-dropdown-menu-sub-trigger]>svg]:rotate-180"
        >
          <slot name="advanced" />
        </DropdownMenuContent>
      </DropdownMenu>

      <slot name="quick" />

      <template v-if="activeFilters.length">
        <button
          v-for="filter in activeFilters"
          :key="filter.key"
          type="button"
          class="group flex h-9 items-center gap-1 rounded-md bg-secondary px-2.5 text-xs font-normal text-muted-foreground hover:bg-secondary/80"
          @click="emit('clear-filter', filter.key)"
        >
          <X class="size-0 shrink-0 transition-all duration-200 group-hover:size-3.5" />
          <span>{{ filter.label }}: {{ filter.value }}</span>
        </button>

        <button
          type="button"
          class="h-9 px-2 text-xs text-muted-foreground hover:text-foreground"
          @click="emit('clear-all')"
        >
          {{ t("filters.clear-all") }}
        </button>
      </template>
    </div>

    <div class="flex items-center gap-2 rtl:flex-row-reverse">
      <slot name="columns" />
      <slot name="actions" />
    </div>
  </div>
</template>
