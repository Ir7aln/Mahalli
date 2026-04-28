<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
import { toast } from "vue-sonner";

const props = defineProps<{
  id: string;
  identifier: string;
}>();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();

async function deleteTheOrders() {
  try {
    const result = await commands.deleteOrder(props.id);
    if (result.status === "error") throw result.error;
    // INFO
    Logger.info(`DELETE ORDER: ${props.id}`);
    //
    toast.success(t("notifications.order.deleted"), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-delete-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR DELETE ORDER: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}
</script>

<template>
  <Card class="card-modal-shell max-w-xl">
    <div class="card-modal-header">
      <div class="card-modal-header-inner">
        <div class="space-y-1">
          <p class="card-modal-eyebrow">{{ t("routes.orders") }}</p>
          <h2 class="card-modal-title">{{ t("titles.orders.delete") }}</h2>
          <p class="card-modal-description">{{ t("modalDescriptions.orders.delete") }}</p>
        </div>
        <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
          <X class="size-5" />
        </Button>
      </div>
    </div>
    <CardContent class="card-modal-body">
      <div class="rounded-xl border border-slate-200 bg-slate-50 px-4 py-3">
        <p class="text-sm text-slate-500">{{ t("fields.order") }}</p>
        <p class="mt-1 text-base font-semibold text-slate-900">{{ identifier }}</p>
      </div>
    </CardContent>
    <div class="card-modal-footer">
      <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-end">
        <Button variant="outline" @click="close">
          {{ t("buttons.cancel") }}
        </Button>
        <Button @click="deleteTheOrders">
          {{ t("buttons.confirme") }}
        </Button>
      </div>
    </div>
  </Card>
</template>
