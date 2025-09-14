<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { Slot as SlotType, useSlotsStore } from "../stores/slots";
import { useEchoStore } from "../stores/echo";
import { EVENT_DROP_ITEM, EVENT_TAKE_ITEM } from "../pkg/events";

type Props = {
  slot?: SlotType;
  icon?: string;
  iconSize?: number;
  size?: number;
};

const { icon, iconSize, slot, size = 40 } = defineProps<Props>();

const echo = useEchoStore();
const slots = useSlotsStore();

function takeDrop() {
  if (!slot) {
    return;
  }

  const payload = {
    kind: slot.kind,
    index: slot.index,
  };

  if (slots.hand?.item) {
    echo.sendMessage(EVENT_DROP_ITEM, payload);
  } else {
    if (slot.item) {
      echo.sendMessage(EVENT_TAKE_ITEM, payload);
    }
  }
}
</script>

<template>
  <div
      class="relative shadow-inner-custom bg-slot/80 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc] transition-colors"
      :class="{
      'hover:border-primary/40': slots.hand?.item,
    }"
      :style="{
      width: `${size}px`,
      height: `${size}px`,
    }"
      @click="takeDrop"
  >
    <Icon
        v-if="icon"
        :icon="icon"
        :width="iconSize"
        class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 pointer-events-none opacity-10"
    />
    <slot />
  </div>
</template>
