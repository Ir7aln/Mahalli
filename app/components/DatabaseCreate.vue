<script setup lang="ts">
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type DatabaseRecord } from "@/bindings";

const props = defineProps<{
  databases: DatabaseRecord[];
  onCreated?: () => void | Promise<void>;
}>();

const { t } = useI18n();
const { close } = useModal();

const form = reactive({
  name: "",
  cloneFromDatabaseId: "",
  makeActive: true,
});

const loading = ref(false);

async function createDatabase() {
  if (!form.name.trim()) {
    toast.error(t("notifications.error.title"), {
      description: t("database.notifications.name-required"),
      closeButton: true,
    });
    return;
  }

  loading.value = true;

  const result = await commands.createDatabase({
    name: form.name.trim(),
    clone_from_database_id: form.cloneFromDatabaseId || null,
    make_active: form.makeActive,
  });

  loading.value = false;

  if (result.status === "error") {
    toast.error(t("notifications.error.title"), {
      description: t("database.notifications.create-error"),
      closeButton: true,
    });
    Logger.error(`CREATE DATABASE: ${JSON.stringify(result.error)}`);
    return;
  }

  toast.success(t("database.notifications.create-success"), {
    description: result.data.data?.name ?? form.name,
    closeButton: true,
  });

  if (props.onCreated) {
    await props.onCreated();
  }

  close();
}
</script>

<template>
  <form class="w-full flex justify-center" @submit.prevent="createDatabase">
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle class="text-center">{{ t("database.create.title") }}</CardTitle>
        <CardDescription class="text-center">
          {{ t("database.create.description") }}
        </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="database-name">{{ t("fields.name") }}</Label>
          <Input
            id="database-name"
            v-model="form.name"
            :placeholder="t('database.placeholders.name')"
          />
        </div>

        <div class="space-y-2">
          <Label for="database-source">{{ t("database.fields.clone-from") }}</Label>
          <select
            id="database-source"
            v-model="form.cloneFromDatabaseId"
            class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background"
          >
            <option value="">{{ t("database.common.start-empty") }}</option>
            <option v-for="database in props.databases" :key="database.id" :value="database.id">
              {{ database.name }}
            </option>
          </select>
        </div>

        <label
          class="flex items-start justify-between gap-3 rounded-md border border-slate-200 p-3 text-left rtl:text-right"
        >
          <div class="min-w-0 flex-1">
            <p class="text-sm font-medium text-slate-900">
              {{ t("database.create.make-active-title") }}
            </p>
            <p class="text-xs text-slate-500">
              {{ t("database.create.make-active-description") }}
            </p>
          </div>
          <Switch v-model:checked="form.makeActive" class="mt-0.5 shrink-0" />
        </label>
      </CardContent>
      <CardFooter class="grid grid-cols-3">
        <Button
          type="button"
          variant="outline"
          class="col-span-1"
          :disabled="loading"
          @click="close"
        >
          {{ t("buttons.cancel") }}
        </Button>
        <Button type="submit" class="col-span-2" :disabled="loading">
          <span
            v-if="loading"
            class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"
          />
          {{ loading ? t("database.actions.creating") : t("database.actions.create") }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
