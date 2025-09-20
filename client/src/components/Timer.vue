<script lang="ts" setup>
import {useExpeditionsStore} from "../stores/expeditions.ts";
import {computed} from "vue";

const expeditionsStore = useExpeditionsStore();

const formattedTime = computed(() => {
  let sec = expeditionsStore.time;
  const h = Math.floor(sec / 3600);
  sec %= 3600;
  const m = Math.floor(sec / 60);
  const s = sec % 60;

  const pad = (n: number) => n.toString().padStart(2, '0');

  if (h > 0) return `${pad(h)}:${pad(m)}:${pad(s)}`;
  if (m > 0) return `${pad(m)}:${pad(s)}`;
  return `00:${pad(s)}`;
});
</script>

<template>
  <span class="loader flex items-center justify-center">
    <span class="font-timer font-bold text-4xl text-zinc-400">{{ formattedTime }}</span>
  </span>
</template>

<style scoped>
.loader {
  position: relative;
  width: 300px;
  height: 300px;
}

.loader:before, .loader:after {
  content: '';
  border-radius: 50%;
  position: absolute;
  inset: 0;
  box-shadow: 0 0 10px 2px rgba(0, 0, 0, 0.3) inset;
}

.loader:after {
  box-shadow: 0 2px 0 #A68B00 inset;
  animation: rotate 2s linear infinite;
}

@keyframes rotate {
  0% {
    transform: rotate(0)
  }
  100% {
    transform: rotate(360deg)
  }
}
</style>