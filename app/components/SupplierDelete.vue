<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const props = defineProps<{
  id: string;
  fullName: string;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();

async function deleteTheSupplier() {
  try {
    await invoke<Res<string>>("delete_supplier", { id: props.id });
    // INFO
    Logger.info(`DELETE SUPPLIER: ${props.id}`);
    //
    toast.success(
      t("notifications.supplier.deleted", { name: props.fullName }),
      {
        closeButton: true,
      }
    );
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-delete-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR DELETE SUPPLIER: ${err.error ? err.error : err.message}`
    );
  } finally {
    close();
  }
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
        {{ t("titles.suppliers.delete") }} {{ fullName }} ?
      </CardTitle>
    </CardHeader>
    <CardContent>
      <div />
    </CardContent>
    <CardFooter>
      <Button variant="outline" @click="close">
        {{ t("buttons.cancel") }}
      </Button>
      <Button class="col-span-2" @click="deleteTheSupplier">
        {{ t("buttons.confirme") }}
      </Button>
    </CardFooter>
  </Card>
</template>
