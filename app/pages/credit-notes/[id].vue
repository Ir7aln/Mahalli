<script setup lang="ts">
definePageMeta({
  layout: "default",
});

const route = useRoute();
const { t } = useI18n();

const creditNoteId = computed(() => route.params.id as string);
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-3xl font-bold">{{ t("sidebar.credit-notes") }}</h1>
      <p class="text-gray-600 mt-1">{{ creditNoteId }}</p>
    </div>

    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
      <p class="text-yellow-800">{{ t("messages.info.credit-note-details-coming-soon") }}</p>
    </div>
  </div>
</template>
