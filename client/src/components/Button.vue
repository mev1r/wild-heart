<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { Variant } from "../types";

type Props = {
  disabled?: boolean;
  variant?: Variant;
  loading?: boolean;
  icon?: string;
};

const { variant = "primary" } = defineProps<Props>();
</script>

<template>
  <button
      class="relative whitespace-nowrap leading-none active:scale-95 flex items-center justify-center gap-1 px-4 h-[32px] transition-all cursor-pointer"
      :class="{
      'pointer-events-none opacity-50': disabled || loading,
      'main-border-hoverable-danger text-danger/80 hover:text-danger':
        variant === 'danger',
      'main-border-hoverable text-zinc-400 hover:text-zinc-300':
        variant === 'primary' || variant === 'success',
      [variant]: true,
    }"
  >
    <template v-if="!loading">
      <Icon
          v-if="icon"
          :icon="icon"
          :width="12"
          class="-translate-x-[2px] min-w-[12px]"
      />
      <slot />
    </template>
    <template v-else>Loading...</template>
  </button>
</template>

<style scoped>
button {
  text-shadow: 0px 1px 1px rgba(0, 0, 0, 0.5);
}
</style>
