<script setup lang="ts">
import { Settings2 } from "lucide-vue-next";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuCheckboxItem,
  DropdownMenuSeparator,
  DropdownMenuLabel,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";

interface Column {
  key: string;
  label: string;
}

const props = defineProps<{
  columns: Column[];
  visibleColumns: string[];
}>();

const emit = defineEmits<{
  (e: "update:visible-columns", value: string[]): void;
}>();

const { locale, t } = useI18n();
const dir = computed(() => (locale.value === "ar" ? "rtl" : "ltr"));

function toggleColumn(key: string) {
  if (props.visibleColumns.includes(key)) {
    emit(
      "update:visible-columns",
      props.visibleColumns.filter((col) => col !== key),
    );
  } else {
    emit("update:visible-columns", [...props.visibleColumns, key]);
  }
}

function toggleAll() {
  if (props.visibleColumns.length === props.columns.length) {
    emit("update:visible-columns", []);
  } else {
    emit(
      "update:visible-columns",
      props.columns.map((col) => col.key),
    );
  }
}
</script>

<template>
  <DropdownMenu :dir="dir">
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="gap-2">
        <Settings2 class="size-4" />
        <span class="hidden sm:inline">{{ t("fields.columns") }}</span>
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent :dir="dir" align="end" class="w-48">
      <DropdownMenuLabel>{{ t("columns.show-hide") }}</DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuCheckboxItem
        :checked="visibleColumns.length === columns.length"
        @select.prevent
        @update:checked="toggleAll"
      >
        <span class="font-medium">{{ t("columns.all") }}</span>
      </DropdownMenuCheckboxItem>
      <DropdownMenuSeparator />
      <DropdownMenuCheckboxItem
        v-for="column in columns"
        :key="column.key"
        :checked="visibleColumns.includes(column.key)"
        @select.prevent
        @update:checked="toggleColumn(column.key)"
      >
        {{ column.label }}
      </DropdownMenuCheckboxItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
