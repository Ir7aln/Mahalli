<script setup lang="ts">
import * as Logger from "@tauri-apps/plugin-log";
import { Database, LoaderCircle } from "lucide-vue-next";
import { toast } from "vue-sonner";
import { commands } from "@/bindings";

const route = useRoute();
const localePath = useLocalePath();
const { t } = useI18n();
const { status, pending, loaded, refreshStatus } = useDatabaseBootstrap();
const isSwitching = ref<string | null>(null);

onMounted(async () => {
  if (!loaded.value) {
    await refreshStatus();
  }
});

const isOnboardingRoute = computed(() => route.path === localePath("/onboarding"));
const isSettingsRoute = computed(() => route.path === localePath("/settings"));
const shouldShowModal = computed(
  () =>
    loaded.value &&
    status.value.has_any_database &&
    !status.value.has_active_database &&
    !isOnboardingRoute.value &&
    !isSettingsRoute.value,
);

async function switchDatabase(id: string) {
  isSwitching.value = id;
  const result = await commands.switchDatabase(id);

  if (result.status === "error") {
    toast.error(t("database.notifications.open-error"), {
      description: t("database.notifications.choose-another"),
      closeButton: true,
    });
    Logger.error(`SWITCH DATABASE: ${JSON.stringify(result.error)}`);
    isSwitching.value = null;
    return;
  }

  await refreshStatus();
  isSwitching.value = null;
  await navigateTo(localePath("/"));
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="shouldShowModal"
      class="fixed inset-0 z-[80] flex items-center justify-center bg-black/40 p-4"
    >
      <Card class="w-full max-w-2xl">
        <CardHeader>
          <div class="flex items-center gap-3 rtl:flex-row-reverse">
            <Database class="text-muted-foreground" :size="18" />
            <div class="text-left rtl:text-right">
              <CardTitle>{{ t("database.selector.title") }}</CardTitle>
              <CardDescription>
                {{ t("database.selector.description") }}
              </CardDescription>
            </div>
          </div>
        </CardHeader>

        <CardContent class="space-y-3">
          <button
            v-for="database in status.databases"
            :key="database.id"
            class="flex w-full items-center justify-between gap-4 rounded-md border px-4 py-4 text-left rtl:text-right transition hover:bg-accent rtl:flex-row-reverse"
            @click="switchDatabase(database.id)"
          >
            <div class="min-w-0 flex-1">
              <p class="font-medium">{{ database.name }}</p>
              <p class="mt-1 truncate text-xs text-muted-foreground">{{ database.file_path }}</p>
            </div>
            <span class="shrink-0 inline-flex items-center gap-2 text-xs font-medium text-muted-foreground">
              <LoaderCircle v-if="isSwitching === database.id" class="animate-spin" :size="14" />
              {{
                isSwitching === database.id
                  ? t("database.selector.opening")
                  : t("database.selector.use-database")
              }}
            </span>
          </button>
        </CardContent>

        <CardFooter>
          <Button variant="outline" class="col-span-1" :disabled="pending" @click="refreshStatus">
            {{ t("database.actions.refresh") }}
          </Button>
          <Button class="col-span-2" @click="navigateTo(localePath('/settings'))">
            {{ t("database.selector.manage") }}
          </Button>
        </CardFooter>
      </Card>
    </div>
  </Teleport>
</template>
