<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
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
    await invoke<Res<any>>("create_inventory", {
      transaction: {
        transaction_type: "IN",
        product_id: id,
        quantity: Number(quantity),
      },
    });
    // INFO
    Logger.info(
      `UPDATE PRODUCT INVENTORY: ${JSON.stringify({
        id,
        quantity: Number(quantity),
      })}`
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
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR UPDATE PRODUCT INVENTORY: ${err.error ? err.error : err.message}`
    );
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
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle>
          {{ t("titles.products.update-inventory") }}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <FormField v-slot="{ componentField }" name="quantity">
          <FormItem>
            <FormLabel>{{ t("fields.quantity") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('fields.quantity')"
                v-bind="componentField"
              >
                <template #unite>
                  {{ t("fields.item") }}
                </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
      </CardContent>
      <CardFooter>
        <Button type="button" variant="outline" @click="close">
          {{ t("buttons.cancel") }}
        </Button>
        <Button type="submit" class="col-span-2">
          {{ t("buttons.update", { name }) }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
