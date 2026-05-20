<script setup lang="ts">
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import { Download, Printer } from "lucide-vue-next";

const SCALE_MIN = 0.5;
const SCALE_MAX = 1.2;
const SCALE_STEP = 0.1;

const props = withDefaults(
  defineProps<{
    pdfContent?: string;
    filename?: string;
    scale?: number;
    loading?: boolean;
    config?: { vat: number; fields: { vat: boolean; full_name: boolean; email: boolean; phone_number: boolean; address: boolean } };
    document?: any;
    documentType?: string;
  }>(),
  {
    pdfContent: "",
    config: () => ({ vat: 20, fields: { vat: true, full_name: true, email: true, phone_number: true, address: true } }),
  },
);

const emit = defineEmits<{
  "update:scale": [value: number];
  regenerate: [];
}>();

const { t } = useI18n();

// ── Scale ────────────────────────────────────────────────────────────────────

const localScale = ref(props.scale ?? 1);
watch(() => props.scale, (v) => { if (v !== undefined) localScale.value = v; });
const scaleLabel = computed(() => `${Math.round(localScale.value * 100)}%`);

let scaleTimer: ReturnType<typeof setTimeout> | null = null;

function scaleDown() {
  const next = Math.round((localScale.value - SCALE_STEP) * 10) / 10;
  if (next < SCALE_MIN) return;
  localScale.value = next;
  scheduleScaleEmit();
}

function scaleUp() {
  const next = Math.round((localScale.value + SCALE_STEP) * 10) / 10;
  if (next > SCALE_MAX) return;
  localScale.value = next;
  scheduleScaleEmit();
}

function scheduleScaleEmit() {
  if (scaleTimer) clearTimeout(scaleTimer);
  scaleTimer = setTimeout(() => emit("update:scale", localScale.value), 300);
}

// ── VAT ──────────────────────────────────────────────────────────────────────

let regenTimer: ReturnType<typeof setTimeout> | null = null;

function onVatToggle(value: boolean) {
  props.config.fields.vat = value;
  emit("regenerate");
}

function onFieldToggle(field: "full_name" | "email" | "phone_number" | "address", value: boolean) {
  props.config.fields[field] = value;
  emit("regenerate");
}

function onVatRateInput(e: Event) {
  const raw = Number((e.target as HTMLInputElement).value);
  props.config.vat = Number.isFinite(raw) ? raw : props.config.vat;
  if (regenTimer) clearTimeout(regenTimer);
  regenTimer = setTimeout(() => emit("regenerate"), 500);
}

// ── Document display ─────────────────────────────────────────────────────────

const clientName = computed(() =>
  props.document?.client?.full_name ?? props.document?.full_name ?? ""
);
const clientEmail = computed(() =>
  props.document?.client?.email ?? props.document?.email ?? ""
);
const clientPhone = computed(() =>
  props.document?.client?.phone_number ?? props.document?.phone_number ?? ""
);
const clientAddress = computed(() =>
  props.document?.client?.address ?? props.document?.address ?? ""
);
const showStatus = computed(
  () =>
    props.document?.status &&
    props.documentType !== "quote" &&
    props.documentType !== "credit-note",
);
const statusLabel = computed(() =>
  props.document?.status
    ? t(`status.${String(props.document.status).toLowerCase()}`)
    : "",
);

// ── Blob URL management ───────────────────────────────────────────────────────

const iframeRef = ref<HTMLIFrameElement | null>(null);
const blobUrl = ref("");

watch(
  () => props.pdfContent,
  (newContent) => {
    const prev = blobUrl.value;
    if (!newContent) {
      blobUrl.value = "";
    } else if (!newContent.startsWith("data:")) {
      blobUrl.value = newContent;
    } else {
      const base64 = newContent.split(",")[1];
      const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
      blobUrl.value = URL.createObjectURL(new Blob([bytes], { type: "application/pdf" }));
    }
    if (prev.startsWith("blob:")) URL.revokeObjectURL(prev);
  },
  { immediate: true },
);

const iframeSrc = computed(() =>
  blobUrl.value ? `${blobUrl.value}#toolbar=0&navpanes=0&view=FitH` : ""
);

// ── Actions ───────────────────────────────────────────────────────────────────

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
  if (scaleTimer) clearTimeout(scaleTimer);
  if (regenTimer) clearTimeout(regenTimer);
  if (blobUrl.value.startsWith("blob:")) URL.revokeObjectURL(blobUrl.value);
});
</script>

<template>
  <div class="flex-1 flex gap-2 h-[calc(100vh-67px)]">

    <!-- PDF viewer -->
    <div
      class="flex-1 sticky top-2 rounded-md border border-slate-200 bg-[radial-gradient(circle_at_1px_1px,_#e2e8f0_1px,_transparent_0)] [background-size:22px_22px] overflow-hidden"
    >
      <div class="h-full w-full p-6 md:p-10 flex items-start justify-center">
        <div
          class="w-full max-w-[860px] h-full rounded-md border border-slate-200 bg-white shadow-[0_20px_60px_rgba(15,23,42,0.12)] overflow-hidden flex flex-col"
        >
          <div class="h-9 shrink-0 border-b border-slate-200 bg-slate-50 flex items-center justify-between px-4">
            <div class="flex items-center gap-2">
              <span class="size-2.5 rounded-full bg-rose-400" />
              <span class="size-2.5 rounded-full bg-amber-400" />
              <span class="size-2.5 rounded-full bg-emerald-400" />
            </div>
            <div class="flex items-center gap-2">
              <div class="flex items-center gap-1">
                <button
                  type="button"
                  @click="scaleDown"
                  :disabled="localScale <= SCALE_MIN"
                  class="flex items-center justify-center size-6 rounded text-slate-500 hover:bg-slate-200 hover:text-slate-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors text-sm font-medium"
                >
                  -
                </button>
                <span class="text-xs text-slate-500 w-9 text-center tabular-nums select-none">{{ scaleLabel }}</span>
                <button
                  type="button"
                  @click="scaleUp"
                  :disabled="localScale >= SCALE_MAX"
                  class="flex items-center justify-center size-6 rounded text-slate-500 hover:bg-slate-200 hover:text-slate-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors text-sm font-medium"
                >
                  +
                </button>
              </div>
              <div class="w-px h-4 bg-slate-200" />
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

          <div class="relative flex-1">
            <iframe
              ref="iframeRef"
              class="absolute inset-0 w-full h-full bg-white"
              style="color-scheme: light"
              :src="iframeSrc"
            />
            <Transition
              enter-active-class="transition-opacity duration-150"
              enter-from-class="opacity-0"
              leave-active-class="transition-opacity duration-150"
              leave-to-class="opacity-0"
            >
              <div
                v-if="loading"
                class="absolute inset-0 bg-white/75 flex items-center justify-center"
              >
                <div class="flex items-center gap-1.5">
                  <span class="size-2 rounded-full bg-slate-400 animate-bounce [animation-delay:-0.3s]" />
                  <span class="size-2 rounded-full bg-slate-400 animate-bounce [animation-delay:-0.15s]" />
                  <span class="size-2 rounded-full bg-slate-400 animate-bounce" />
                </div>
              </div>
            </Transition>
          </div>
        </div>
      </div>
    </div>

    <!-- Config panel -->
    <div class="w-72 shrink-0 sticky top-2 overflow-y-auto flex flex-col gap-3">

      <!-- VAT -->
      <div class="rounded-md border border-slate-200 bg-white p-4 space-y-3">
        <p class="text-sm font-semibold text-slate-700">{{ t("fields.vat-rate") }}</p>
        <div class="flex items-center justify-between">
          <span class="text-sm text-slate-600">{{ t("fields.vat-rate") }}</span>
          <Switch
            :checked="config.fields.vat"
            @update:checked="onVatToggle"
          />
        </div>
        <div v-if="config.fields.vat" class="flex items-center gap-2">
          <input
            type="number"
            min="0"
            max="100"
            :value="config.vat"
            @input="onVatRateInput"
            class="w-full h-9 rounded-md border border-slate-200 bg-white px-3 text-sm text-slate-900 focus:outline-none focus:ring-2 focus:ring-slate-300"
          />
          <span class="text-sm text-slate-500 shrink-0">%</span>
        </div>
      </div>

      <!-- Client fields -->
      <div v-if="document" class="rounded-md border border-slate-200 bg-white p-4 space-y-3">
        <p class="text-sm font-semibold text-slate-700">{{ t("fields.bill-to") }}</p>

        <div class="flex items-center justify-between">
          <span class="text-sm text-slate-600">{{ t("fields.full-name") }}</span>
          <Switch :checked="config.fields.full_name" @update:checked="(v) => onFieldToggle('full_name', v)" />
        </div>

        <div class="flex items-center justify-between">
          <span class="text-sm text-slate-600">{{ t("fields.email") }}</span>
          <Switch :checked="config.fields.email" @update:checked="(v) => onFieldToggle('email', v)" />
        </div>

        <div class="flex items-center justify-between">
          <span class="text-sm text-slate-600">{{ t("fields.phone") }}</span>
          <Switch :checked="config.fields.phone_number" @update:checked="(v) => onFieldToggle('phone_number', v)" />
        </div>

        <div class="flex items-center justify-between">
          <span class="text-sm text-slate-600">{{ t("fields.address") }}</span>
          <Switch :checked="config.fields.address" @update:checked="(v) => onFieldToggle('address', v)" />
        </div>
      </div>

      <!-- Status -->
      <div v-if="showStatus" class="rounded-md border border-slate-200 bg-white p-4">
        <p class="text-sm font-semibold text-slate-700 mb-1.5">{{ t("fields.status") }}</p>
        <p class="text-sm text-slate-600">{{ statusLabel }}</p>
      </div>

    </div>
  </div>
</template>
