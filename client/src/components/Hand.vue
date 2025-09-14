<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { useSlotsStore } from "../stores/slots";
import { useMouse } from "@vueuse/core";
import { onMounted } from "vue";
import Section from "./Section.vue";

const slots = useSlotsStore();
const { x: mouseX, y: mouseY } = useMouse();

onMounted(() => {
  const e = window.event as MouseEvent;
  if (e) {
    mouseX.value = e.clientX;
    mouseY.value = e.clientY;
  }
});
</script>

<template>
  <Section
      v-if="slots.hand?.item"
      :tier="slots.hand.item.tier"
      class="fixed w-[40px] h-[40px] flex items-center justify-center pointer-events-none"
      :style="{
      left: `${mouseX - 20}px`,
      top: `${mouseY - 20}px`,
    }"
  >
    <Icon :icon="slots.hand.item.icon" :width="28" class="text-zinc-500" />
    <span
        v-if="slots.hand.item.quantity > 1"
        class="absolute text-xs left-[3px] bottom-[3px] leading-none"
    >
      {{ slots.hand.item.quantity > 99 ? "99+" : slots.hand.item.quantity }}
    </span>
    <span
        v-if="slots.hand.item.enchanted > 0"
        class="absolute text-[9px] left-[2px] bottom-[2px] leading-none text-silver font-bold"
    >
      {{ `+${slots.hand.item.enchanted}` }}
    </span>
    <span
        v-if="slots.hand.item.level > 0"
        class="absolute text-[9px] right-[2px] bottom-[2px] leading-none text-silver"
    >
      Lv{{ `${slots.hand.item.level}` }}
    </span>
  </Section>
</template>
