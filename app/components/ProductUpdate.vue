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
  purchasePrice: number;
  sellingPrice: number;
  description?: string;
  minQuantity: number;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();
const productSchema = toTypedSchema(
  z.object({
    name: z
      .string()
      .min(2)
      .max(50)
      .default(props.name as string),
    purchase_price: z
      .number()
      .min(0)
      .default(Number(props.purchasePrice ?? 0)),
    selling_price: z
      .number()
      .min(0)
      .default(Number(props.sellingPrice ?? 0)),
    description: z
      .string()
      .min(2)
      .default((props.description as string) ?? ""),
    min_quantity: z.number().default(Number(props.minQuantity) ?? 0),
  })
);

const form = useForm({
  validationSchema: productSchema,
});

async function updateTheProduct(product: ProductT) {
  try {
    const id = props.id;
    await invoke<Res<string>>("update_product", {
      product: {
        name: product.name,
        selling_price: Number(product.selling_price),
        purchase_price: Number(product.purchase_price),
        description: product.description,
        min_quantity: Number(product.min_quantity),
        image: "",
        id,
      },
    });

    Logger.info(
      `UPDATE PRODUCT: ${JSON.stringify({
        name: product.name,
        selling_price: Number(product.selling_price),
        purchase_price: Number(product.purchase_price),
        description: product.description,
        min_quantity: Number(product.min_quantity),
        id,
      })}`
    );
    //
    toast.success(t("notifications.product.updated", { name: product.name }), {
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
      `ERROR UPDATE PRODUCT: ${err.error ? err.error : err.message}`
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
          {{ t("titles.products.update") }}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <FormField v-slot="{ componentField }" name="name">
          <FormItem>
            <FormLabel>{{ t("fields.name") }}</FormLabel>
            <FormControl>
              <Input :placeholder="t('fields.name')" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="purchase_price">
          <FormItem>
            <FormLabel>{{ t("fields.purchase-price") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('fields.purchase-price')"
                v-bind="componentField"
              >
                <template #unite>
                  DH
                </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="selling_price">
          <FormItem>
            <FormLabel>{{ t("fields.selling-price") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('fields.selling-price')"
                v-bind="componentField"
              >
                <template #unite>
                  DH
                </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="min_quantity">
          <FormItem>
            <FormLabel>{{ t("fields.min-quantity") }}</FormLabel>
            <FormControl>
              <Input
                type="number"
                :placeholder="t('fields.min-quantity')"
                v-bind="componentField"
              >
                <template #unite>
                  {{ t("fields.item") }}
                </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="description">
          <FormItem>
            <FormLabel>
              {{ t("fields.description") }}
            </FormLabel>
            <FormControl>
              <Textarea
                :placeholder="t('fields.description')"
                v-bind="componentField"
              />
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
