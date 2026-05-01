<script setup lang="ts">
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { commands, type UpdateSellerProfileDTO } from "@/bindings";

const { t } = useI18n();
const { showErrorToast } = useCommandError();

const saving = ref(false);
const form = reactive<UpdateSellerProfileDTO>({
  legal_name: "",
  commercial_name: null,
  address: null,
  city: null,
  phone_number: null,
  email: null,
  ice: null,
  if_number: null,
  rc: null,
  patente: null,
  logo: null,
  default_currency: "MAD",
  default_payment_terms_days: 30,
  invoice_footer: null,
});

const { data, refresh } = useAsyncData("settings-seller-profile", async () => {
  const result = await commands.getSellerProfile();
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`GET SELLER PROFILE: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

watch(
  data,
  (profile) => {
    if (!profile) return;
    form.legal_name = profile.legal_name;
    form.commercial_name = profile.commercial_name;
    form.address = profile.address;
    form.city = profile.city;
    form.phone_number = profile.phone_number;
    form.email = profile.email;
    form.ice = profile.ice;
    form.if_number = profile.if_number;
    form.rc = profile.rc;
    form.patente = profile.patente;
    form.logo = profile.logo;
    form.default_currency = profile.default_currency;
    form.default_payment_terms_days = profile.default_payment_terms_days;
    form.invoice_footer = profile.invoice_footer;
  },
  { immediate: true },
);

function emptyToNull(value: string | number | null) {
  if (typeof value === "number") return value;
  return value?.trim() ? value.trim() : null;
}

async function save() {
  saving.value = true;
  const result = await commands.updateSellerProfile({
    legal_name: emptyToNull(form.legal_name),
    commercial_name: emptyToNull(form.commercial_name),
    address: emptyToNull(form.address),
    city: emptyToNull(form.city),
    phone_number: emptyToNull(form.phone_number),
    email: emptyToNull(form.email),
    ice: emptyToNull(form.ice),
    if_number: emptyToNull(form.if_number),
    rc: emptyToNull(form.rc),
    patente: emptyToNull(form.patente),
    logo: emptyToNull(form.logo),
    default_currency: emptyToNull(form.default_currency),
    default_payment_terms_days: form.default_payment_terms_days,
    invoice_footer: emptyToNull(form.invoice_footer),
  });

  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`UPDATE SELLER PROFILE: ${JSON.stringify(result.error)}`);
    saving.value = false;
    return;
  }

  toast.success(t("notifications.seller-profile.updated"), { closeButton: true });
  await refresh();
  saving.value = false;
}
</script>

<template>
  <section class="rounded-md border border-slate-200 bg-white p-6 shadow-sm">
    <div class="flex flex-col gap-2 text-left rtl:text-right">
      <h2 class="text-lg font-semibold text-slate-900">
        {{ t("seller-profile.title") }}
      </h2>
      <p class="text-sm text-slate-500">
        {{ t("seller-profile.description") }}
      </p>
    </div>

    <div class="mt-5 grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <div class="space-y-2">
        <Label>{{ t("seller-profile.fields.legal-name") }}</Label>
        <Input v-model="form.legal_name" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("seller-profile.fields.commercial-name") }}</Label>
        <Input v-model="form.commercial_name" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.email") }}</Label>
        <Input v-model="form.email" type="email" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.phone") }}</Label>
        <Input v-model="form.phone_number" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.address") }}</Label>
        <Input v-model="form.address" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("seller-profile.fields.city") }}</Label>
        <Input v-model="form.city" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.ice") }}</Label>
        <Input v-model="form.ice" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.if-number") }}</Label>
        <Input v-model="form.if_number" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.rc") }}</Label>
        <Input v-model="form.rc" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("fields.patente") }}</Label>
        <Input v-model="form.patente" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("seller-profile.fields.currency") }}</Label>
        <Input v-model="form.default_currency" />
      </div>
      <div class="space-y-2">
        <Label>{{ t("seller-profile.fields.payment-terms") }}</Label>
        <Input v-model.number="form.default_payment_terms_days" type="number" min="0" />
      </div>
      <div class="space-y-2 md:col-span-2 xl:col-span-3">
        <Label>{{ t("seller-profile.fields.invoice-footer") }}</Label>
        <Textarea v-model="form.invoice_footer" />
      </div>
    </div>

    <div class="mt-5 flex justify-end">
      <Button :disabled="saving" :loading="saving" @click="save">
        {{ t("buttons.save") }}
      </Button>
    </div>
  </section>
</template>
