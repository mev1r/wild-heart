<script setup lang="ts">
import { computed } from "vue";
import { formatNumber } from "../pkg/utils";

type Props = {
  max: number;
  current: number;
  color: string;
  size?: number;
  wrapper?: boolean;
  numbers?: boolean;
};

const {
  max,
  current,
  color,
  wrapper,
  numbers = true,
  size = 16,
} = defineProps<Props>();

const progress = computed(() => {
  const coef = current / max;
  const percent = Math.min(coef * 100, 100);

  return Math.max(0, percent).toFixed(2);
});
</script>

<template>
  <div
      class="w-full z-10 flex items-center px-[1px]"
      :class="{
      'bg-slot/50 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc] relative':
        !wrapper,
      'absolute top-0 left-0 w-full': wrapper,
    }"
      :style="{ height: `${size}px` }"
  >
    <div
        class="transition-all"
        :class="{ [color]: true, 'ring-1 ring-inset bg-shield/20': wrapper }"
        :style="{ width: `${progress}%`, height: `${size - 2}px` }"
    />
    <div
        v-if="max && numbers"
        class="absolute left-1/2 -translate-x-1/2 top-1/2 -translate-y-1/2 text-zinc-300 text-[12px]"
    >
      {{ formatNumber(current) }} / {{ formatNumber(max) }}
    </div>
  </div>
</template>
