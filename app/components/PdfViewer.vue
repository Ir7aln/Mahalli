<script setup lang="ts">
const props = defineProps<{
  pdfContent: string;
}>();

const iframeSrc = computed(() => {
  return props.pdfContent || "";
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
        <div class="h-9 border-b border-slate-200 bg-slate-50 flex items-center px-4">
          <div class="flex items-center gap-2">
            <span class="size-2.5 rounded-full bg-rose-400" />
            <span class="size-2.5 rounded-full bg-amber-400" />
            <span class="size-2.5 rounded-full bg-emerald-400" />
          </div>
        </div>
        <iframe class="w-full h-[calc(100vh-180px)] bg-white" :src="iframeSrc" />
      </div>
    </div>
  </div>
</template>
