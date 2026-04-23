<script setup lang="ts">
import * as Logger from "@tauri-apps/plugin-log";
import { Check, Database, Sparkles } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { commands } from "@/bindings";

definePageMeta({
  layout: "onboarding",
});

const localePath = useLocalePath();
const { t } = useI18n();
const { status, refreshStatus } = useDatabaseBootstrap();

const form = reactive({
  name: "",
  cloneFromDatabaseId: "",
});
const isSubmitting = ref(false);

async function createInitialDatabase() {
  if (!form.name.trim()) {
    toast.error(t("database.notifications.name-required"), { closeButton: true });
    return;
  }

  isSubmitting.value = true;

  const result = await commands.createDatabase({
    name: form.name.trim(),
    clone_from_database_id: form.cloneFromDatabaseId || null,
    make_active: true,
  });

  isSubmitting.value = false;

  if (result.status === "error") {
    toast.error(t("database.notifications.initial-create-error"), {
      description: t("database.notifications.try-another-name"),
      closeButton: true,
    });
    Logger.error(`CREATE INITIAL DATABASE: ${JSON.stringify(result.error)}`);
    return;
  }

  await refreshStatus();
  toast.success(t("database.notifications.ready"), {
    description: result.data.data?.name ?? form.name,
    closeButton: true,
  });
  await navigateTo(localePath("/"));
}
</script>

<template>
  <div class="grid w-full gap-10 lg:grid-cols-[1.1fr_0.9fr]">
    <section class="space-y-8 text-left rtl:text-right">
      <div
        class="inline-flex items-center gap-2 rounded-full border border-sky-200 bg-white/80 px-4 py-2 text-sm text-sky-700 shadow-sm rtl:flex-row-reverse"
      >
        <Sparkles :size="16" />
        {{ t("database.onboarding.badge") }}
      </div>

      <div class="max-w-2xl space-y-5">
        <h1 class="font-cairo text-5xl font-semibold leading-tight text-slate-950">
          {{ t("database.onboarding.title") }}
        </h1>
        <p class="max-w-xl text-base leading-7 text-slate-600">
          {{ t("database.onboarding.description") }}
        </p>
      </div>
    </section>

    <section
      class="rounded-s border border-white/70 bg-white/90 p-7 shadow-xl shadow-sky-100/80 text-left rtl:text-right"
    >
      <div class="space-y-2">
        <p class="text-sm font-medium uppercase tracking-[0.25em] text-slate-500">
          {{ t("database.onboarding.initial-tenant") }}
        </p>
        <h2 class="text-2xl font-semibold text-slate-950">
          {{ t("database.onboarding.form-title") }}
        </h2>
        <p class="text-sm leading-6 text-slate-500">
          {{ t("database.onboarding.form-description") }}
        </p>
      </div>

      <div class="mt-6 space-y-5">
        <div class="space-y-2">
          <Label for="initial-db-name">{{ t("database.fields.database-name") }}</Label>
          <Input
            id="initial-db-name"
            v-model="form.name"
            :placeholder="t('database.placeholders.initial-name')"
          />
        </div>

        <div class="space-y-2">
          <Label for="initial-db-source">{{ t("database.fields.start-from") }}</Label>
          <select
            id="initial-db-source"
            v-model="form.cloneFromDatabaseId"
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background"
          >
            <option value="">{{ t("database.common.start-empty") }}</option>
            <option v-for="database in status.databases" :key="database.id" :value="database.id">
              {{ database.name }}
            </option>
          </select>
        </div>

        <div class="rounded-md border border-slate-200 bg-slate-50 p-4 text-sm text-slate-600">
          {{ t("database.onboarding.active-note") }}
        </div>

        <Button class="w-full" :disabled="isSubmitting" @click="createInitialDatabase">
          {{
            isSubmitting
              ? t("database.onboarding.creating")
              : t("database.onboarding.create-and-continue")
          }}
        </Button>
      </div>
    </section>
  </div>
</template>
