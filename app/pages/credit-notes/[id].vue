<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

const { showErrorToast } = useCommandError();
const id = useRoute().params.id;
const pdfContent = ref("");
const scale = ref(1);

const { config, generatePdf, isGenerating } = usePdfGenerator();
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
    const url = await generatePdf(creditNote.value, "credit-note", scale.value);
    if (url) pdfContent.value = url;
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR CREDIT NOTE PDF: ${err.error ? err.error : err.message}`);
  }
}

handleGeneratePdf();
</script>

<template>
  <main class="h-full flex-1 flex">
    <PdfViewer
      :pdf-content="pdfContent"
      :filename="creditNote?.identifier ?? undefined"
      :scale="scale"
      :loading="isGenerating"
      :config="config"
      :document="creditNote"
      document-type="credit-note"
      @update:scale="(v) => { scale = v; handleGeneratePdf(); }"
      @regenerate="handleGeneratePdf"
    />
  </main>
</template>
