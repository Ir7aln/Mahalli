<script setup lang="ts">
defineProps<{
  pdfContent: string;
}>();
</script>

<template>
  <iframe
    class="flex-1 rounded-md h-[calc(100vh-67px)] sticky top-2"
    :src="pdfContent"
  />
</template>
