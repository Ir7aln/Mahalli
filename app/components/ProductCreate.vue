<script setup lang="ts">
import { commands, type NewProduct } from "@/bindings";
import type { NewInventory } from "@/bindings";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
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

async function createNewProduct(product: NewProduct) {
  try {
    const createResult = await commands.createProduct({
      name: product.name,
      selling_price: Number(product.selling_price),
      purchase_price: Number(product.purchase_price),
      description: product.description ?? null,
      min_quantity: product.min_quantity,
      image: image.value ?? null,
    });
    if (createResult.status === "error") throw createResult.error;

    const invPayload: NewInventory = {
      transaction_type: "IN",
      product_id: createResult.data.data as string,
      quantity: Number(quantity.value),
    };
    const invResult = await commands.createInventory(invPayload);
    if (invResult.status === "error") throw invResult.error;
    Logger.info(
      `CREATE PRODUCT: ${JSON.stringify({
        ...product,
        image: image.value,
        quantity: quantity.value,
      })}`,
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
    Logger.error(`ERROR CREATE PRODUCT: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  createNewProduct({
    name: values.name,
    selling_price: Number(values.selling_price),
    purchase_price: Number(values.purchase_price),
    description: values.description ?? null,
    min_quantity: Number(values.min_quantity),
    image: null,
  });
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
    <Card class="card-modal-shell w-4/6 lg:w-1/2">
      <div class="card-modal-header">
        <div class="card-modal-header-inner">
          <div class="space-y-1">
            <p class="card-modal-eyebrow">{{ t("routes.products") }}</p>
            <h2 class="card-modal-title">{{ t("titles.products.create") }}</h2>
            <p class="card-modal-description">{{ t("modalDescriptions.products.create") }}</p>
          </div>
          <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
            <X class="size-5" />
          </Button>
        </div>
      </div>
      <CardContent class="card-modal-body">
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
                <template #unite> DH </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="selling_price">
          <FormItem>
            <FormLabel>{{ t("fields.selling-price") }}</FormLabel>
            <FormControl>
              <Input type="number" :placeholder="t('fields.selling-price')" v-bind="componentField">
                <template #unite> DH </template>
              </Input>
            </FormControl>
          </FormItem>
        </FormField>
        <FormField name="">
          <FormItem>
            <FormLabel>{{ t("fields.init-quantity") }}</FormLabel>
            <FormControl>
              <Input v-model="quantity" type="number" :placeholder="t('fields.init-quantity')">
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
              <Input type="number" :placeholder="t('fields.min-quantity')" v-bind="componentField">
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
              <Textarea :placeholder="t('fields.description')" v-bind="componentField" />
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
            {{ t("buttons.add") }}
          </Button>
        </div>
      </div>
    </Card>
  </form>
</template>
