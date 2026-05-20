<script setup lang="ts">
import { commands } from "@/bindings";
import * as Logger from "@tauri-apps/plugin-log";

const { showErrorToast } = useCommandError();
const id = useRoute().params.id;
const pdfContent = ref("");
const scale = ref(1);

const { config, generatePdf, isGenerating } = usePdfGenerator();
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
    const url = await generatePdf(invoice.value, "invoice", scale.value);
    if (url) pdfContent.value = url;
  } catch (err: any) {
    showErrorToast(err);
    Logger.error(`ERROR INVOICE PDF: ${err.error ? err.error : err.message}`);
  }
}

handleGeneratePdf();
</script>

<template>
  <main class="h-full flex-1 flex">
    <PdfViewer
      :pdf-content="pdfContent"
      :filename="invoice?.identifier ?? undefined"
      :scale="scale"
      :loading="isGenerating"
      :config="config"
      :document="invoice"
      document-type="invoice"
      @update:scale="(v) => { scale = v; handleGeneratePdf(); }"
      @regenerate="handleGeneratePdf"
    />
  </main>
</template>
