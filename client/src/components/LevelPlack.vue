<script setup lang="ts">
import Section from "./Section.vue";

type Props = {
  level?: number;
};

const { level = 1 } = defineProps<Props>();
</script>

<template>
  <Section
      class="px-2 inline-flex items-center justify-center w-[52px] py-[1px] text-3xl leading-none border border-secondary"
  >
    {{ level }}
  </Section>
</template>
