<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { downloadDir, pictureDir, sep } from "@tauri-apps/api/path";
import { TauriEvent, type UnlistenFn, listen } from "@tauri-apps/api/event";
import { FileCheck, Upload, X } from "lucide-vue-next";
import * as Logger from "@tauri-apps/plugin-log";
import { toast } from "vue-sonner";
import { convertFileSrc } from "@tauri-apps/api/core";

interface Props {
  extensions: string[];
  name: string;
  maxFileSize?: number;
}

const props = withDefaults(defineProps<Props>(), {
  maxFileSize: 10 * 1024 * 1024,
});

const emits = defineEmits<{
  saveBytes: [payload: Uint8Array, name: string];
  savePath: [payload: string];
  clear: [];
}>();
const { t } = useI18n();
const dropZone = ref<HTMLDivElement>();
const isOverDropZone = ref(false);
const isFileSelected = ref(false);
const selectedFile = ref<string | null>();
const fileName = ref("");
const dragCounter = ref(0);

const unListenFns: UnlistenFn[] = [];

onMounted(async () => {
  const unListenDrop = await listen(TauriEvent.DRAG_DROP, (e: any) => {
    try {
      const filePath = e.payload.paths[0];
      isOverDropZone.value = false;
      dragCounter.value--;
      processFile(filePath);
    } catch (error) {
      Logger.error(`ERROR FILE DROP : ${error}`);
    }
  });
  const unListenDragLeave = await listen(TauriEvent.DRAG_LEAVE, () => {
    isOverDropZone.value = false;
    dragCounter.value--;
  });
  const unListenDragEnter = await listen(TauriEvent.DRAG_ENTER, () => {
    isOverDropZone.value = true;
    dragCounter.value++;
  });

  unListenFns.push(unListenDragLeave, unListenDragEnter, unListenDrop);
});

onBeforeUnmount(() => {
  for (const unListen of unListenFns) {
    unListen();
  }
});

function validateFile(file: File): boolean {
  if (file.size > props.maxFileSize) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.fileTooLarge"),
      closeButton: true,
    });
    return false;
  }

  const fileExt = file.name.split(".").pop()?.toLowerCase();
  if (!fileExt || !props.extensions.includes(fileExt)) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.invalidFileType"),
      closeButton: true,
    });
    return false;
  }
  return true;
}

async function processFile(filePath: string) {
  fileName.value = filePath.split(sep()).at(-1)!;
  const file = new File([(await getFileBytes(filePath))!], fileName.value);
  try {
    if (!validateFile(file)) {
      return;
    }
    emits("savePath", filePath);
    if (props.name === "Image") {
      selectedFile.value = convertFileSrc(filePath);
    } else {
      isFileSelected.value = true;
    }
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR PROSSESING FILE: ${err}`);
  }
}

async function handleOpenDialog() {
  try {
    const filePath = (await open({
      multiple: false,
      filters: [{ name: props.name, extensions: props.extensions }],
      defaultPath:
        props.name === "Image" ? await pictureDir() : await downloadDir(),
    })) as string | null;

    if (filePath) {
      await processFile(filePath);
    }
  } catch (err: any) {
    toast.error(t("notifications.error.title"), {
      description: t("notifications.error.description"),
      closeButton: true,
    });
    Logger.error(`ERROR OPENING DIALOG: ${err}`);
  }
}

function clearFile() {
  selectedFile.value = null;
  isFileSelected.value = false;
  fileName.value = "";
  emits("clear");
}
</script>

<template>
  <div class="relative w-full h-36">
    <button
      v-if="selectedFile || isFileSelected"
      class="absolute -top-2 -right-2 z-[90] size-6 bg-slate-100 rounded-full hover:bg-red-50 hover:text-red-500 transition-all duration-150 flex items-center justify-center group"
      @click="clearFile"
    >
      <X
        :size="16"
        class="group-hover:scale-110 transition-transform duration-150"
      />
    </button>

    <img
      v-if="name === 'Image' && selectedFile"
      :src="`${selectedFile}`"
      class="absolute top-0 w-full h-full object-cover rounded-md border border-gray-300 shadow-sm transition-all duration-200"
      alt="Selected image preview"
    >

    <div
      v-else
      ref="dropZone"
      class="w-full h-full rounded-md transition-all duration-200 transform relative border-2 border-dashed border-spacing-4 flex items-center justify-center"
      :class="[
        isOverDropZone || dragCounter > 0
          ? 'border-sky-500 bg-sky-50'
          : isFileSelected
            ? 'border-green-500 bg-green-50'
            : 'border-gray-300 bg-white hover:border-sky-400 hover:bg-sky-50',
      ]"
    >
      <button
        type="button"
        class="w-full h-full flex flex-col gap-3 justify-center items-center transition-all duration-200 transform disabled:opacity-50"
        :class="[
          isOverDropZone || dragCounter > 0
            ? 'text-sky-500'
            : isFileSelected
              ? 'text-green-500'
              : 'text-gray-400 hover:text-sky-500',
        ]"
        @click="handleOpenDialog"
      >
        <span v-if="!isFileSelected" class="text-sm font-medium">
          {{ t("dropZone") }}
        </span>
        <Upload
          v-if="!isFileSelected"
          :size="26"
          class="transition-transform duration-200 hover:scale-110"
        />
        <span
          v-if="isFileSelected && name === 'Pdf'"
          class="text-sm font-medium space-y-1"
        >
          <FileCheck
            :size="30"
            class="transition-transform duration-200 hover:scale-110 m-auto"
          />
          <span>{{ fileName }}</span>
        </span>
      </button>
    </div>
  </div>
</template>
