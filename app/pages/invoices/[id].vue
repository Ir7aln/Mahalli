<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { INVOICE_STATUSES } from "~/consts";

const { t } = useI18n();
const id = useRoute().params.id;
const pdfContent = ref("");

const { config, generatePdf } = usePdfGenerator();
const { data: invoice } = await useAsyncData(async () => {
  try {
    const res = await invoke<Res<any>>("get_invoice_details", {
      id,
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR INVOICE DETAILS: ${err.error ? err.error : err.message}`
    );
  }
});

async function handleGeneratePdf() {
  try {
    const pdfDataUri = await generatePdf(invoice.value, "invoice");
    if (pdfDataUri) {
      pdfContent.value = pdfDataUri;
    }
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR INVOICE DETAILS: ${err.error ? err.error : err.message}`
    );
  }
}

async function saveConfig() {
  try {
    let filePath: string | null = null;
    if (config.template.bytes && config.template.name) {
      filePath = await uploadFileToDataDir(
        "pdf-templates",
        config.template.bytes,
        config.template.name
      );
    }
    await invoke("create_template", {
      template: {
        values_json: JSON.stringify({
          ...config,
          template: {
            path: filePath,
            name: config.template.name,
          },
        }),
      },
    });
    toast(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(
      `ERROR CREATE TEMPLATE: ${err.error ? err.error : err.message}`
    );
  }
}

async function updateConfig(configAndValues: any) {
  const { documentValues, ...configValues } = configAndValues;
  invoice.value = { ...invoice.value, ...documentValues };

  config.fields = configValues.fields;
  config.marginBottom = configValues.marginBottom;
  config.marginTop = configValues.marginTop;
  config.vat = configValues.vat;
  if (configAndValues.template) {
    const fileBytes = await getFileBytes(configAndValues.template);
    const fileName = configAndValues.template;
    config.template = {
      bytes: fileBytes,
      name: fileName,
    };
  } else {
    config.template = {
      bytes: null,
      name: null,
    };
  }
  handleGeneratePdf();
}

handleGeneratePdf();
</script>

<template>
  <main class="flex space-x-2 h-full flex-1">
    <PdfViewer :pdf-content="pdfContent" />
    <TemplateForm
      :config="config"
      :document="invoice"
      document-type="invoice"
      :statues="INVOICE_STATUSES"
      @update-config="updateConfig"
      @save-config="saveConfig"
    />
  </main>
</template>
