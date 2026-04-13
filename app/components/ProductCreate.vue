<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { useForm } from "vee-validate";
import { toast } from "vue-sonner";
import { z } from "zod";

const { t } = useI18n();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const quantity = ref<number>(0);

const CreateProductSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(2).max(50),
  selling_price: z.number().min(0),
  purchase_price: z.number().min(0),
  image: z.string().optional(),
  description: z.string().optional(),
  min_quantity: z.number().min(0),
  inventory: z.number().optional(),
  created_at: z.number().optional(),
});

const productSchema = toTypedSchema(CreateProductSchema);
const form = useForm({
  validationSchema: productSchema,
});

const image = ref<string | null>(null);

async function createNewProduct(product: ProductT) {
  try {
    const createRes = await invoke<Res<string>>("create_product", {
      product: {
        name: product.name,
        selling_price: Number(product.selling_price),
        purchase_price: Number(product.purchase_price),
        description: product.description,
        min_quantity: product.min_quantity,
        image: image.value,
      },
    });
    await invoke<Res<string>>("create_inventory", {
      transaction: {
        transaction_type: "IN",
        product_id: createRes.data,
        quantity: Number(quantity.value),
      },
    });
    Logger.info(
      `CREATE PRODUCT: ${JSON.stringify({
        ...product,
        image: image.value,
        quantity: quantity.value,
      })}`
    );
    //
    toast.success(t("notifications.product.created", { name: product.name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-create-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR CREATE PRODUCT: ${err.error ? err.error : err.message}`
    );
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  createNewProduct(values);
});

function setImage(imagePath: string | null) {
  image.value = imagePath;
}

function cleanImage() {
  image.value = null;
}
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="w-4/6 lg:w-1/2">
      <CardHeader>
        <CardTitle>
          {{ t("titles.products.create") }}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <UiUploader
          name="Image"
          :extensions="['png', 'jpeg', 'webp', 'jpg']"
          @clear="cleanImage"
          @save-path="setImage"
        />
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
        <FormField name="">
          <FormItem>
            <FormLabel>{{ t("fields.init-quantity") }}</FormLabel>
            <FormControl>
              <Input
                v-model="quantity"
                type="number"
                :placeholder="t('fields.init-quantity')"
              >
                <template #unite>
                  {{ t("fields.item") }}
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
          {{ t("buttons.add") }}
        </Button>
      </CardFooter>
    </Card>
  </form>
</template>
