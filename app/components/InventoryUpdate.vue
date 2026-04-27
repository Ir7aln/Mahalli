<script setup lang="ts">
import { commands } from "@/bindings";
import type { NewInventory } from "@/bindings";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { toast } from "vue-sonner";
import { z } from "zod";

const props = defineProps<{
  id: string;
  name: string;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const inventory = z.object({
  quantity: z.number().default(0),
});

const inventorySchema = toTypedSchema(inventory);

const form = useForm({
  validationSchema: inventorySchema,
});

async function updateTheProduct({ quantity }: z.infer<typeof inventory>) {
  try {
    const id = props.id;
    const payload: NewInventory = {
      transaction_type: "IN",
      product_id: id,
      quantity: Number(quantity),
    };
    const result = await commands.createInventory(payload);
    if (result.status === "error") throw result.error;
    // INFO
    Logger.info(
      `UPDATE PRODUCT INVENTORY: ${JSON.stringify({
        id,
        quantity: Number(quantity),
      })}`,
    );
    //
    toast.success(t("notifications.product.updated", { name: props.name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-update-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR UPDATE PRODUCT INVENTORY: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  updateTheProduct(values);
});
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="card-modal-shell w-4/6 lg:w-1/2">
      <div class="card-modal-header">
        <div class="card-modal-header-inner">
          <div class="space-y-1">
            <p class="card-modal-eyebrow">{{ t("routes.inventory") }}</p>
            <h2 class="card-modal-title">{{ t("titles.products.update-inventory") }}</h2>
            <p class="card-modal-description">
              {{ t("modalDescriptions.products.update-inventory") }}
            </p>
          </div>
          <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
            <X class="size-5" />
          </Button>
        </div>
      </div>
      <CardContent class="card-modal-body">
        <FormField v-slot="{ componentField }" name="quantity">
          <FormItem>
            <FormLabel>{{ t("fields.quantity") }}</FormLabel>
            <FormControl>
              <Input type="number" :placeholder="t('fields.quantity')" v-bind="componentField">
                <template #unite>
                  {{ t("fields.item") }}
                </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
      </CardContent>
      <div class="card-modal-footer">
        <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-end">
          <Button type="button" variant="outline" @click="close">
            {{ t("buttons.cancel") }}
          </Button>
          <Button type="submit">
            {{ t("buttons.update", { name }) }}
          </Button>
        </div>
      </div>
    </Card>
  </form>
</template>


