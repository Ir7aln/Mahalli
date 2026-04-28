<script setup lang="ts">
import { commands } from "@/bindings";
import type { Client } from "@/bindings";
import { toTypedSchema } from "@vee-validate/zod";
import * as Logger from "@tauri-apps/plugin-log";
import { X } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { toast } from "vue-sonner";
import { z } from "zod";

const props = defineProps<{
  id: string;
  fullName: string;
  email?: string;
  phoneNumber?: string;
  address?: string;
  ice?: string;
  ifNumber?: string;
  rc?: string;
  patente?: string;
}>();
const { t } = useI18n();
const { showErrorToast } = useCommandError();
const { updateQueryParams } = useUpdateRouteQueryParams();
const { close } = useModal();
const clientSchema = toTypedSchema(
  z.object({
    full_name: z
      .string()
      .min(2)
      .max(50)
      .default(props.fullName as string),
    email: z.string().default((props.email as string) ?? ""),
    phone_number: z.string().default((props.phoneNumber as string) ?? ""),
    address: z.string().default((props.address as string) ?? ""),
    ice: z.string().default((props.ice as string) ?? ""),
    if_number: z.string().default((props.ifNumber as string) ?? ""),
    rc: z.string().default((props.rc as string) ?? ""),
    patente: z.string().default((props.patente as string) ?? ""),
  }),
);

const form = useForm({
  validationSchema: clientSchema,
});

async function updateTheClient(client: {
  full_name: string;
  email?: string;
  phone_number?: string;
  address?: string;
  ice?: string;
  if_number?: string;
  rc?: string;
  patente?: string;
}) {
  try {
    const payload: Client = {
      id: props.id,
      full_name: client.full_name,
      email: client.email ?? null,
      phone_number: client.phone_number ?? null,
      address: client.address ?? null,
      image: null,
      ice: client.ice ?? null,
      if_number: client.if_number ?? null,
      rc: client.rc ?? null,
      patente: client.patente ?? null,
    };
    const result = await commands.updateClient(payload);
    if (result.status === "error") throw result.error;
    //
    Logger.info(
      `UPDATE CLIENT: ${JSON.stringify({
        id: props.id,
        full_name: client.full_name,
        email: client.email,
        phone_number: client.phone_number,
        address: client.address,
        ice: client.ice,
        if_number: client.if_number,
        rc: client.rc,
        patente: client.patente,
      })}`,
    );
    //
    toast.success(t("notifications.client.updated", { name: client.full_name }), {
      closeButton: true,
    });
    // toggle refresh
    updateQueryParams({
      refresh: `refresh-update-${Math.random() * 9999}`,
    });
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR UPDATE CLIENT: ${err.error ? err.error : err.message}`);
  } finally {
    close();
  }
}

const onSubmit = form.handleSubmit((values) => {
  updateTheClient(values);
});
</script>

<template>
  <form class="w-full flex justify-center" @submit="onSubmit">
    <Card class="card-modal-shell w-4/6 lg:w-1/2">
      <div class="card-modal-header">
        <div class="card-modal-header-inner">
          <div class="space-y-1">
            <p class="card-modal-eyebrow">{{ t("routes.clients") }}</p>
            <h2 class="card-modal-title">{{ t("titles.clients.update") }}</h2>
            <p class="card-modal-description">{{ t("modalDescriptions.clients.update") }}</p>
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
        <div class="grid gap-4 sm:grid-cols-2">
          <FormField v-slot="{ componentField }" name="ice">
            <FormItem>
              <FormLabel>{{ t("fields.ice") }}</FormLabel>
              <FormControl>
                <Input :placeholder="t('fields.ice')" v-bind="componentField" />
              </FormControl>
            </FormItem>
          </FormField>
          <FormField v-slot="{ componentField }" name="if_number">
            <FormItem>
              <FormLabel>{{ t("fields.if-number") }}</FormLabel>
              <FormControl>
                <Input :placeholder="t('fields.if-number')" v-bind="componentField" />
              </FormControl>
            </FormItem>
          </FormField>
          <FormField v-slot="{ componentField }" name="rc">
            <FormItem>
              <FormLabel>{{ t("fields.rc") }}</FormLabel>
              <FormControl>
                <Input :placeholder="t('fields.rc')" v-bind="componentField" />
              </FormControl>
            </FormItem>
          </FormField>
          <FormField v-slot="{ componentField }" name="patente">
            <FormItem>
              <FormLabel>{{ t("fields.patente") }}</FormLabel>
              <FormControl>
                <Input :placeholder="t('fields.patente')" v-bind="componentField" />
              </FormControl>
            </FormItem>
          </FormField>
        </div>
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
