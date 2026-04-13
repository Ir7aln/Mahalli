<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const props = defineProps<{
  id: string;
  name: string;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();

async function deleteTheProduct() {
  try {
    const result = await commands.deleteProduct(props.id);
    if (result.status === "error") throw result.error;
    // INFO
    Logger.info(`DELETE PRODUCT: ${props.id}`);
    //
    toast.success(t("notifications.product.deleted", { name: props.name }), {
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
    Logger.error(`ERROR DELETE PRODUCT: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle> {{ t("titles.products.delete") }} {{ name }} ? </CardTitle>
    </CardHeader>
    <CardContent>
      <div />
    </CardContent>
    <CardFooter>
      <Button variant="outline" @click="close">
        {{ t("buttons.cancel") }}
      </Button>
      <Button class="col-span-2" @click="deleteTheProduct">
        {{ t("buttons.confirme") }}
      </Button>
    </CardFooter>
  </Card>
</template>
