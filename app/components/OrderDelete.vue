<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const props = defineProps<{
  id: string;
  identifier: string;
}>();
const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();

async function deleteTheOrders() {
  try {
    await invoke<Res<any>>("delete_order", { id: props.id });
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
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR DELETE ORDER: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        {{ t("titles.orders.delete") }}n° {{ identifier }} ?
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div />
    </CardContent>
    <CardFooter>
      <Button variant="outline" @click="close">
        {{ t("buttons.cancel") }}
      </Button>
      <Button class="col-span-2" @click="deleteTheOrders">
        {{ t("buttons.confirme") }}
      </Button>
    </CardFooter>
  </Card>
</template>
