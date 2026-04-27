<script setup lang="ts">
import { toTypedSchema } from "@vee-validate/zod";
import { commands } from "@/bindings";
import type { Supplier } from "@/bindings";
import { useForm } from "vee-validate";
import { z } from "zod";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
import { toast } from "vue-sonner";

const props = defineProps<{
  id: string;
  fullName: string;
  email?: string;
  phoneNumber?: string;
  address?: string;
}>();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const supplierSchema = toTypedSchema(
  z.object({
    full_name: z
      .string()
      .min(2)
      .max(50)
      .default(props.fullName as string),
    email: z.string().default((props.email as string) ?? ""),
    phone_number: z.string().default((props.phoneNumber as string) ?? ""),
    address: z.string().default((props.address as string) ?? ""),
  }),
);

const form = useForm({
  validationSchema: supplierSchema,
});

async function updateTheSupplier(supplier: {
  full_name: string;
  email?: string;
  phone_number?: string;
  address?: string;
}) {
  try {
    const payload: Supplier = {
      id: props.id,
      full_name: supplier.full_name,
      email: supplier.email ?? null,
      phone_number: supplier.phone_number ?? null,
      address: supplier.address ?? null,
      image: null,
    };
    const result = await commands.updateSupplier(payload);
    if (result.status === "error") throw result.error;
    //
    Logger.info(
      `UPDATE SUPPLIER: ${JSON.stringify({
        id: props.id,
        full_name: supplier.full_name,
        email: supplier.email,
        phone_number: supplier.phone_number,
        address: supplier.address,
      })}`,
    );
    //
    toast.success(t("notifications.supplier.updated", { name: supplier.full_name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-update-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR UPDATE SUPPLIER: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  updateTheSupplier(values);
});
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="card-modal-shell w-4/6 lg:w-1/2">
      <div class="card-modal-header">
        <div class="card-modal-header-inner">
          <div class="space-y-1">
            <p class="card-modal-eyebrow">{{ t("routes.suppliers") }}</p>
            <h2 class="card-modal-title">{{ t("titles.suppliers.update") }}</h2>
            <p class="card-modal-description">{{ t("modalDescriptions.suppliers.update") }}</p>
          </div>
          <Button type="button" variant="ghost" size="icon" class="rounded-full" @click="close">
            <X class="size-5" />
          </Button>
        </div>
      </div>
      <CardContent class="card-modal-body">
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
            {{ t("buttons.update", { name: fullName }) }}
          </Button>
        </div>
      </div>
    </Card>
  </form>
</template>


