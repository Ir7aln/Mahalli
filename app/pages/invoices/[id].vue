<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { INVOICE_STATUSES } from "~/consts";

const { t } = useI18n();
const { showErrorToast } = useCommandError();
const id = useRoute().params.id;
const pdfContent = ref("");

const { config, generatePdf } = usePdfGenerator();
const { data: invoice } = await useAsyncData(async () => {
  const result = await commands.getInvoiceDetails(id as string);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR INVOICE DETAILS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

async function handleGeneratePdf() {
  try {
    const pdfDataUri = await generatePdf(invoice.value, "invoice");
    if (pdfDataUri) {
      pdfContent.value = pdfDataUri;
    }
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR INVOICE DETAILS: ${err.error ? err.error : err.message}`);
  }
}

async function saveConfig() {
  let filePath: string | null = null;
  if (config.template.bytes && config.template.name) {
    filePath = await uploadFileToDataDir(
      "pdf-templates",
      config.template.bytes,
      config.template.name,
    );
  }
  const result = await commands.createTemplate({
    values_json: JSON.stringify({
      ...config,
      template: {
        path: filePath,
        name: config.template.name,
      },
    }),
  });
  if (result.status === "error") {
    Logger.error(`ERROR: ${JSON.stringify(result.error)}`);
    return;
  }
  toast(t("notifications.error.title"), {
    description: t("notifications.error.description"),
    closeButton: true,
  });
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
