<script setup lang="ts">
import { ChevronLeft, ChevronRight, Database, ChevronsUpDown, Check } from "lucide-vue-next";
import { commands, type DatabaseRecord } from "@/bindings";
import { toast } from "vue-sonner";
import * as Logger from "@tauri-apps/plugin-log";

const { t } = useI18n();
const { showErrorToast } = useCommandError();

const { data, refresh } = await useAsyncData("nav-databases", async () => {
  const result = await commands.getDatabaseBootstrapStatus();
  if (result.status === "error") return { databases: [], activeDatabase: null };
  return {
    databases: result.data.data?.databases ?? [],
    activeDatabase: result.data.data?.active_database ?? null,
  };
});

const databases = computed<DatabaseRecord[]>(() => data.value?.databases ?? []);
const activeDatabase = computed(() => data.value?.activeDatabase ?? null);

async function switchDatabase(id: string) {
  const result = await commands.switchDatabase(id);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`SWITCH DATABASE: ${JSON.stringify(result.error)}`);
    return;
  }
  toast.success(t("database.notifications.switch-success"), {
    description: result.data.data?.name ?? "",
    closeButton: true,
  });
  await refreshNuxtData();
}
</script>

<template>
  <header class="flex-1 sticky border-b border-slate-100 top-0 z-50">
    <div class="w-full h-full flex items-center justify-between py-3 px-2 bg-white">
      <div class="text-black flex items-center justify-center gap-2 rtl:flex-row-reverse">
        <ChevronLeft :size="20" class="cursor-pointer" @click="$router.back()" />
        <ChevronRight :size="20" class="cursor-pointer" @click="$router.forward()" />
      </div>

      <Popover v-if="activeDatabase">
        <PopoverTrigger as-child>
          <button
            type="button"
            class="flex items-center gap-2 rounded-lg border border-slate-200 bg-slate-50 px-3 py-1.5 text-sm font-medium text-slate-700 shadow-sm transition-colors hover:bg-slate-100 hover:border-slate-300 focus:outline-none"
          >
            <Database class="size-3.5 text-slate-500 shrink-0" />
            <span class="max-w-[140px] truncate">{{ activeDatabase.name }}</span>
            <ChevronsUpDown class="size-3.5 text-slate-400 shrink-0" />
          </button>
        </PopoverTrigger>
        <PopoverContent class="w-64 p-1.5" align="end">
          <p class="px-2 py-1.5 text-xs font-semibold text-slate-400 uppercase tracking-wider">
            {{ t("database.status.switch-title") }}
          </p>
          <div class="space-y-0.5">
            <button
              v-for="db in databases"
              :key="db.id"
              type="button"
              class="w-full flex items-center gap-2.5 rounded-md px-2 py-2 text-sm text-left transition-colors"
              :class="db.is_active ? 'bg-slate-100 text-slate-900 font-medium' : 'text-slate-600 hover:bg-slate-50 hover:text-slate-900'"
              :disabled="db.is_active"
              @click="!db.is_active && switchDatabase(db.id)"
            >
              <Database class="size-3.5 shrink-0" :class="db.is_active ? 'text-slate-700' : 'text-slate-400'" />
              <span class="flex-1 truncate">{{ db.name }}</span>
              <Check v-if="db.is_active" class="size-3.5 text-slate-600 shrink-0" />
            </button>
          </div>
        </PopoverContent>
      </Popover>
    </div>
  </header>
</template>
