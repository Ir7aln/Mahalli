<script setup lang="ts">
import * as Logger from "@tauri-apps/plugin-log";
import { Plus, RefreshCw } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { commands, type DatabaseRecord } from "@/bindings";
import { DatabaseCreate } from "#components";

const { t } = useI18n();
const { locale } = useI18n();
const { status, refreshStatus } = useDatabaseBootstrap();
const modal = useModal();

const refreshKey = ref(0);

const { data: databasesData, refresh: refreshDatabases, pending } = await useAsyncData(
  "settings-databases",
  async () => {
    const bootstrapResult = await commands.getDatabaseBootstrapStatus();

    if (bootstrapResult.status === "error") {
      Logger.error(`GET DATABASE BOOTSTRAP STATUS: ${JSON.stringify(bootstrapResult.error)}`);
      throw new Error("failed to fetch databases");
    }

    return {
      databases: bootstrapResult.data.data?.databases ?? [],
      activeDatabase: bootstrapResult.data.data?.active_database ?? null,
    };
  },
  {
    watch: [refreshKey],
  },
);

const databases = computed<DatabaseRecord[]>(() => databasesData.value?.databases ?? []);
const activeDatabase = computed<DatabaseRecord | null>(() => databasesData.value?.activeDatabase ?? null);

async function refreshPage() {
  refreshKey.value += 1;
  await refreshStatus();
  await refreshDatabases();
}

function openCreateDatabaseModal() {
  modal.open(DatabaseCreate, {
    databases: databases.value,
    onCreated: refreshPage,
  });
}

async function switchDatabase(id: string) {
  const result = await commands.switchDatabase(id);

  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("database.notifications.switch-error"),
      closeButton: true,
    });
    Logger.error(`SWITCH DATABASE: ${JSON.stringify(result.error)}`);
    return;
  }

  toast.success(t("database.notifications.switch-success"), {
    description: result.data.data?.name ?? "",
    closeButton: true,
  });

  await refreshPage();
}
</script>

<template>
  <main class="w-full h-full">
    <div class="flex h-full flex-col gap-6">
      <section class="rounded-md border border-slate-200 bg-white p-6 shadow-sm">
        <div class="flex items-start justify-between gap-4 rtl:flex-row-reverse">
          <div class="text-left rtl:text-right">
            <h1 class="text-2xl font-semibold text-slate-900">{{ t("database.settings.title") }}</h1>
            <p class="mt-1 text-sm text-slate-500">
              {{ t("database.settings.description") }}
            </p>
          </div>
          <Button class="gap-2" variant="outline" @click="refreshPage">
            <RefreshCw :size="16" />
            {{ t("database.actions.refresh") }}
          </Button>
        </div>

        <div class="mt-5 grid gap-4 md:grid-cols-2">
          <div class="rounded-md border border-slate-200 bg-slate-50 p-4 text-left rtl:text-right">
            <p class="text-xs font-medium uppercase tracking-[0.2em] text-slate-500">
              {{ t("database.settings.active-title") }}
            </p>
            <p class="mt-3 text-lg font-semibold text-slate-900">
              {{ activeDatabase?.name ?? t("database.common.none") }}
            </p>
            <p class="mt-1 text-sm text-slate-500">
              {{ activeDatabase?.file_path ?? t("database.settings.no-active-description") }}
            </p>
          </div>
          <div class="rounded-md border border-slate-200 bg-slate-50 p-4 text-left rtl:text-right">
            <p class="text-xs font-medium uppercase tracking-[0.2em] text-slate-500">
              {{ t("database.settings.catalog-title") }}
            </p>
            <p class="mt-3 text-lg font-semibold text-slate-900">
              {{ t("database.settings.catalog-count", { count: databases.length }) }}
            </p>
            <p class="mt-1 text-sm text-slate-500">
              {{ t("database.settings.catalog-description") }}
            </p>
          </div>
        </div>
      </section>

      <section class="grid gap-6 xl:grid-cols-[380px_minmax(0,1fr)]">
        <div class="rounded-md border border-slate-200 bg-white p-6 shadow-sm text-left rtl:text-right">
          <h2 class="text-lg font-semibold text-slate-900">{{ t("database.create.title") }}</h2>
          <p class="mt-1 text-sm text-slate-500">
            {{ t("database.settings.create-card-description") }}
          </p>

          <div class="mt-5 rounded-md border border-slate-200 bg-slate-50 p-4">
            <p class="text-sm text-slate-600">
              {{ t("database.settings.create-card-note") }}
            </p>
            <Button class="mt-4 w-full gap-2 rtl:flex-row-reverse" :disabled="pending" @click="openCreateDatabaseModal">
              <Plus :size="16" />
              {{ t("database.actions.new") }}
            </Button>
          </div>
        </div>

        <div class="min-w-0 rounded-md border border-slate-200 bg-white p-6 shadow-sm flex flex-col">
          <div class="flex items-center justify-between gap-4 rtl:flex-row-reverse">
            <div class="text-left rtl:text-right">
              <h2 class="text-lg font-semibold text-slate-900">
                {{ t("database.settings.registered-title") }}
              </h2>
              <p class="mt-1 text-sm text-slate-500">
                {{ t("database.settings.registered-description") }}
              </p>
            </div>
            <Badge variant="secondary">{{ databases.length }}</Badge>
          </div>

          <div class="mt-5 max-h-[520px] overflow-auto rounded-md border border-slate-200">
            <table :dir="locale === 'ar' ? 'rtl' : 'ltr'" class="min-w-full text-left rtl:text-right text-sm">
              <thead class="bg-slate-50 text-slate-500">
                <tr>
                  <th class="px-4 py-3 font-medium">{{ t("fields.name") }}</th>
                  <th class="px-4 py-3 font-medium">{{ t("database.fields.slug") }}</th>
                  <th class="px-4 py-3 font-medium">{{ t("database.fields.source") }}</th>
                  <th class="px-4 py-3 font-medium">{{ t("fields.status") }}</th>
                  <th class="px-4 py-3 font-medium ltr:text-right rtl:text-left">
                    {{ t("fields.actions") }}
                  </th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="database in databases" :key="database.id" class="border-t border-slate-200">
                  <td class="px-4 py-4 text-left rtl:text-right">
                    <p class="font-medium text-slate-900">{{ database.name }}</p>
                    <p class="max-w-[220px] truncate text-xs text-slate-500" :title="database.file_name">
                      {{ database.file_name }}
                    </p>
                  </td>
                  <td class="px-4 py-4 text-slate-600 text-left rtl:text-right">{{ database.slug }}</td>
                  <td class="px-4 py-4 text-slate-600 text-left rtl:text-right">
                    {{ database.created_from_database_id ?? t("database.common.empty") }}
                  </td>
                  <td class="px-4 py-4">
                    <Badge :variant="database.is_active ? 'default' : 'secondary'">
                      {{ database.is_active ? t("database.status.active") : t("database.status.idle") }}
                    </Badge>
                  </td>
                  <td class="px-4 py-4 ltr:text-right rtl:text-left">
                    <Button
                      v-if="!database.is_active"
                      size="sm"
                      variant="outline"
                      @click="switchDatabase(database.id)"
                    >
                      {{ t("database.actions.switch") }}
                    </Button>
                    <span v-else class="text-xs font-medium uppercase tracking-[0.2em] text-slate-400">
                      {{ t("database.status.current") }}
                    </span>
                  </td>
                </tr>
                <tr v-if="databases.length === 0">
                  <td colspan="5" class="px-4 py-8 text-center text-slate-500">
                    {{ t("database.settings.empty-state") }}
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>
    </div>
  </main>
</template>
