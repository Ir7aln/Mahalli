<script setup lang="ts">
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Filter, Search, SlidersHorizontal, X } from "lucide-vue-next";

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
    advancedLabel?: string;
  }>(),
  {
    activeFilters: () => [],
    searchPlaceholder: "",
    advancedLabel: "Filters",
  },
);

const emit = defineEmits<{
  (e: "update:search", value: string): void;
  (e: "clear-filter", key: string): void;
  (e: "clear-all"): void;
}>();

const { t } = useI18n();
</script>

<template>
  <div class="mb-3 flex w-full flex-col gap-2">
    <div class="flex w-full items-center justify-end gap-2">
      <slot name="actions" />
    </div>

    <div class="flex w-full flex-col gap-2 lg:flex-row lg:items-center">
      <div class="flex w-full flex-col gap-2 sm:flex-row sm:items-center">
        <div class="w-full sm:max-w-sm">
          <Input
            :model-value="search"
            type="text"
            :placeholder="searchPlaceholder || t('search')"
            @update:model-value="(value) => emit('update:search', String(value))"
          >
            <template #prefix>
              <Search class="size-4" />
            </template>
          </Input>
        </div>

        <slot name="quick" />

        <Popover v-if="$slots.advanced">
          <PopoverTrigger as-child>
            <Button variant="outline" class="gap-2">
              <SlidersHorizontal class="size-4" />
              {{ advancedLabel }}
              <Badge
                v-if="activeFilters.length"
                variant="secondary"
                class="rounded-full bg-slate-100 px-2 py-0.5 text-xs text-slate-700"
              >
                {{ activeFilters.length }}
              </Badge>
            </Button>
          </PopoverTrigger>
          <PopoverContent class="w-[min(92vw,36rem)] p-4">
            <div class="space-y-4">
              <div class="flex items-center justify-between gap-3">
                <div class="flex items-center gap-2 text-sm font-medium text-slate-900">
                  <Filter class="size-4" />
                  {{ advancedLabel }}
                </div>
                <Button
                  v-if="activeFilters.length"
                  variant="ghost"
                  size="sm"
                  class="h-8 px-2 text-slate-600"
                  @click="emit('clear-all')"
                >
                  {{ t("filters.clear-all") }}
                </Button>
              </div>
              <slot name="advanced" />
            </div>
          </PopoverContent>
        </Popover>
      </div>
    </div>

    <div v-if="activeFilters.length" class="flex flex-wrap gap-2">
      <Badge
        v-for="filter in activeFilters"
        :key="filter.key"
        variant="secondary"
        class="flex items-center gap-2 rounded-full bg-slate-100 px-3 py-1 text-slate-700"
      >
        <span class="text-xs font-medium">{{ filter.label }}: {{ filter.value }}</span>
        <button
          type="button"
          class="rounded-full text-slate-500 hover:text-slate-900"
          @click="emit('clear-filter', filter.key)"
        >
          <X class="size-3.5" />
        </button>
      </Badge>
    </div>
  </div>
</template>
