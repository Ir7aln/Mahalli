<script setup lang="ts">
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { Download, Printer } from "lucide-vue-next";

const props = defineProps<{
  pdfContent: string;
  filename?: string;
}>();

const iframeRef = ref<HTMLIFrameElement | null>(null);

const blobUrl = computed(() => {
  if (!props.pdfContent) return "";
  if (!props.pdfContent.startsWith("data:")) return props.pdfContent;
  const base64 = props.pdfContent.split(",")[1];
  const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  const blob = new Blob([bytes], { type: "application/pdf" });
  return URL.createObjectURL(blob);
});

const iframeSrc = computed(() =>
  blobUrl.value ? `${blobUrl.value}#toolbar=0&navpanes=0&view=FitH` : ""
);

function print() {
  iframeRef.value?.contentWindow?.print();
}

async function download() {
  const path = await save({
    filters: [{ name: "PDF", extensions: ["pdf"] }],
    defaultPath: `${props.filename ?? "document"}.pdf`,
  });
  if (!path) return;

  let bytes: Uint8Array;
  if (props.pdfContent.startsWith("data:")) {
    const base64 = props.pdfContent.split(",")[1];
    bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  } else {
    const res = await fetch(props.pdfContent);
    bytes = new Uint8Array(await res.arrayBuffer());
  }
  await writeFile(path, bytes);
}

onUnmounted(() => {
  if (blobUrl.value) URL.revokeObjectURL(blobUrl.value);
});
</script>

<template>
  <div
    class="flex-1 h-[calc(100vh-67px)] sticky top-2 rounded-md border border-slate-200 bg-[radial-gradient(circle_at_1px_1px,_#e2e8f0_1px,_transparent_0)] [background-size:22px_22px] overflow-auto"
  >
    <div class="min-h-full w-full p-6 md:p-10 flex items-start justify-center">
      <div
        class="w-full max-w-[860px] rounded-md border border-slate-200 bg-white shadow-[0_20px_60px_rgba(15,23,42,0.12)] overflow-hidden"
      >
        <div class="h-9 border-b border-slate-200 bg-slate-50 flex items-center justify-between px-4">
          <div class="flex items-center gap-2">
            <span class="size-2.5 rounded-full bg-rose-400" />
            <span class="size-2.5 rounded-full bg-amber-400" />
            <span class="size-2.5 rounded-full bg-emerald-400" />
          </div>
          <div class="flex items-center gap-1">
            <button
              type="button"
              @click="print"
              class="flex items-center justify-center size-6 rounded hover:bg-slate-200 transition-colors"
            >
              <Printer class="size-3.5 text-slate-500" />
            </button>
            <button
              type="button"
              @click="download"
              class="flex items-center justify-center size-6 rounded hover:bg-slate-200 transition-colors"
            >
              <Download class="size-3.5 text-slate-500" />
            </button>
          </div>
        </div>
        <iframe ref="iframeRef" class="w-full h-[calc(100vh-180px)] bg-white" :src="iframeSrc" />
      </div>
    </div>
  </div>
</template>
