<script setup lang="ts">
import { commands, type NewSupplier } from "@/bindings";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { toast } from "vue-sonner";
import { z } from "zod";

const { t } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const CreateSupplierSchema = z.object({
  id: z.string().optional(),
  full_name: z.string().min(2).max(50),
  email: z.string().optional(),
  phone_number: z.string().optional(),
  address: z.string().optional(),
  image: z.string().optional(),
  credit: z.number().optional(),
});

const supplierSchema = toTypedSchema(CreateSupplierSchema);

const form = useForm({
  validationSchema: supplierSchema,
});

const image = ref<string | null>(null);

async function createNewSupplier(supplier: NewSupplier) {
  try {
    const result = await commands.createSupplier({
      full_name: supplier.full_name,
      email: supplier.email ?? null,
      phone_number: supplier.phone_number ?? null,
      address: supplier.address ?? null,
      image: image.value ?? null,
    });
    if (result.status === "error") throw result.error;
    //
    Logger.info(
      `CREATE SUPPLIER: ${JSON.stringify({
        ...supplier,
        image: image.value,
      })}`,
    );
    //
    toast.success(t("notifications.supplier.created", { name: supplier.full_name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-create-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR CREATE SUPPLIER: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  createNewSupplier({
    full_name: values.full_name,
    email: values.email ?? null,
    phone_number: values.phone_number ?? null,
    address: values.address ?? null,
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
            <p class="card-modal-eyebrow">{{ t("routes.suppliers") }}</p>
            <h2 class="card-modal-title">{{ t("titles.suppliers.create") }}</h2>
            <p class="card-modal-description">{{ t("modalDescriptions.suppliers.create") }}</p>
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
        <FormField v-slot="{ componentField }" name="full_name">
          <FormItem>
            <FormLabel>{{ t("fields.full-name") }}</FormLabel>
            <FormControl>
              <Input :placeholder="t('fields.full-name')" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="email">
          <FormItem>
            <FormLabel>{{ t("fields.email") }}</FormLabel>
            <FormControl>
              <Input placeholder="example@gmail.com" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="phone_number">
          <FormItem>
            <FormLabel>{{ t("fields.phone") }}</FormLabel>
            <FormControl>
              <Input placeholder="+2126********" v-bind="componentField" />
            </FormControl>
          </FormItem>
        </FormField>
        <FormField v-slot="{ componentField }" name="address">
          <FormItem>
            <FormLabel>{{ t("fields.address") }}</FormLabel>
            <FormControl>
              <Input :placeholder="t('fields.address')" v-bind="componentField" />
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


