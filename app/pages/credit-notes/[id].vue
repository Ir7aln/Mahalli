<script setup lang="ts">
import { commands } from "@/bindings";
import { sep } from "@tauri-apps/api/path";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";

const { t } = useI18n();
const { showErrorToast } = useCommandError();
const id = useRoute().params.id;
const pdfContent = ref("");

const { config, generatePdf } = usePdfGenerator();
const { data: creditNote } = await useAsyncData(async () => {
  const result = await commands.getCreditNote(id as string);
  if (result.status === "error") {
    showErrorToast(result.error);
    Logger.error(`ERROR CREDIT NOTE DETAILS: ${JSON.stringify(result.error)}`);
    return null;
  }
  return result.data.data;
});

async function handleGeneratePdf() {
  try {
    const pdfDataUri = await generatePdf(creditNote.value, "credit-note");
    if (pdfDataUri) {
      pdfContent.value = pdfDataUri;
    }
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR CREDIT NOTE PDF: ${err.error ? err.error : err.message}`);
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
  creditNote.value = { ...creditNote.value, ...documentValues };

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
      :document="creditNote"
      document-type="credit-note"
      @update-config="updateConfig"
      @save-config="saveConfig"
    />
  </main>
</template>
