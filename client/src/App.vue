<script setup lang="ts">
import { Window } from "@tauri-apps/api/window";

function minimize() {
  Window.getCurrent().minimize();
}

function close() {
  Window.getCurrent().close();
}
</script>

<template>
  <div class="flex justify-between h-[42px] select-none">
    <div class="drag flex-1 flex items-center">
      <span
          class="uppercase text-primary px-4 font-semibold text-lg leading-none"
      >
        Wild Heart
      </span>
    </div>
    <div class="flex">
      <button
          class="text-zinc-500 flex items-center justify-center h-[34px] px-4 leading-none pt-2 transition-colors hover:bg-silver/20 hover:text-zinc-200"
          @click="minimize"
      >
        —
      </button>
      <button
          class="text-zinc-500 flex items-center justify-center h-[34px] px-4 leading-none transition-colors hover:bg-danger hover:text-zinc-200"
          @click="close"
      >
        ✕
      </button>
    </div>
  </div>
  <RouterView />
</template>

<style scoped>
.drag {
  -webkit-app-region: drag;
}
</style>
