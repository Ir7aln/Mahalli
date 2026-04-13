<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { sep } from "@tauri-apps/api/path";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const { t } = useI18n();
const id = useRoute().params.id;
const pdfContent = ref("");

const { config, generatePdf } = usePdfGenerator();
const { data: quote } = await useAsyncData(async () => {
  try {
    const res = await invoke<Res<any>>("get_quote_details", {
      id,
    });
    return res.data;
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR QUOTE DETAILS: ${err.error ? err.error : err.message}`);
  }
});

async function handleGeneratePdf() {
  try {
    const pdfDataUri = await generatePdf(quote.value, "quote");
    if (pdfDataUri) {
      pdfContent.value = pdfDataUri;
    }
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR QUOTE DETAILS: ${err.error ? err.error : err.message}`);
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
  quote.value = { ...quote.value, ...documentValues };

  config.fields = configValues.fields;
  config.marginBottom = configValues.marginBottom;
  config.marginTop = configValues.marginTop;
  config.vat = configValues.vat;
  if (configAndValues.template) {
    const fileBytes = await getFileBytes(configAndValues.template);
    const fileName = configAndValues.template.split(sep()).at(-1);
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
  <main class="w-full h-full flex gap-2 min-h-[calc(100vh-68px)]">
    <PdfViewer :pdf-content="pdfContent" />
    <TemplateForm
      :config="config"
      :document="quote"
      document-type="quote"
      @update-config="updateConfig"
      @save-config="saveConfig"
    />
  </main>
</template>
