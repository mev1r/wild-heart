<script lang="ts" setup>
import {computed, onMounted, onUnmounted, ref} from "vue";
import {useExpeditionsStore} from "../stores/expeditions";

const expeditionsStore = useExpeditionsStore();

const now = ref(Date.now());
let rafId: number;

const progress = computed(() => {
  if (!expeditionsStore.duration) {
    return 0;
  }

  const remaining = Math.max(expeditionsStore.endTime - now.value, 0);
  const coef = remaining / (expeditionsStore.duration);

  return Math.min(Math.max(coef * 100, 0), 100);
});

onUnmounted(() => {
  cancelAnimationFrame(rafId);
});
onMounted(() => {
  const loop = () => {
    now.value = Date.now();
    rafId = requestAnimationFrame(loop);
  };

  rafId = requestAnimationFrame(loop);
});
</script>

<template>
  <div
      class="w-full z-10 flex items-center px-[1px] bg-slot/50 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc] relative h-[10px]"
  >
    <div
        :style="{ width: `${progress}%` }"
        class="ring-1 ring-inset ring-shield/20 bg-shield/30 h-[8px]"
    />
  </div>
</template>
