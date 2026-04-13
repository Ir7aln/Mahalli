<script lang="ts" setup>
import { inject } from "vue";
import { modalInjectionKey, useModal } from "@/composables/useModal";

const modalState = inject(modalInjectionKey);

const { isOpen, reset } = useModal();
</script>

<template>
  <component
    :is="modalState.component"
    v-if="modalState"
    v-bind="modalState.props"
    v-model="isOpen"
    @after-leave="reset"
  />
</template>
