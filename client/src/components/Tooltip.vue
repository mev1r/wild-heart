<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { Tier } from "../types";
import { nextTick, ref, useSlots } from "vue";
import { useElementBounding } from "@vueuse/core";
import Section from "./Section.vue";

type Props = {
  title: string;
  type: string;
  icon?: string;
  tier?: Tier;
  iconTier?: Tier;
};

const { tier = "common", iconTier = "common" } = defineProps<Props>();
const slots = useSlots();

const CONTAINER = document.getElementById("app");

const isVisible = ref(false);
const tooltipRef = ref<HTMLElement | null>(null);
const tooltipWrapperRef = ref<HTMLElement | null>(null);

const positionX = ref(0);
const positionY = ref(0);
const opacity = ref(0);

const {
  left: windowLeft,
  right: windowRight,
  top: windowTop,
} = useElementBounding(CONTAINER);

async function onHover() {
  isVisible.value = true;

  await nextTick();

  const {
    left: tooltipWrapperLeft,
    top: tooltipWrapperTop,
    bottom: tooltipWrapperBottom,
    width: tooltipWrapperWidth,
  } = useElementBounding(tooltipWrapperRef);
  const { width: tooltipWidth, height: tooltipHeight } =
      useElementBounding(tooltipRef);

  positionX.value =
      tooltipWrapperLeft.value +
      tooltipWrapperWidth.value / 2 -
      tooltipWidth.value / 2;
  positionY.value = tooltipWrapperTop.value - tooltipHeight.value - 8;

  const isHitLeft = positionX.value <= windowLeft.value;
  const isHitRight = positionX.value + tooltipWidth.value >= windowRight.value;
  const isHitTop = positionY.value <= windowTop.value;

  if (isHitLeft) {
    positionX.value = 16;
  }

  if (isHitRight) {
    positionX.value = windowRight.value - tooltipWidth.value - 16;
  }

  if (isHitTop) {
    positionY.value = tooltipWrapperBottom.value + 8;
  }

  opacity.value = 1;
}

function onLeave() {
  isVisible.value = false;
}
</script>

<template>
  <div
      ref="tooltipWrapperRef"
      class="relative w-full h-full z-10"
      @mouseenter="onHover"
      @mouseleave="onLeave"
      @mousedown="onLeave"
  >
    <slot name="trigger" />
    <Teleport to="#tooltips">
      <Section
          v-if="isVisible"
          ref="tooltipRef"
          :tier="tier"
          class="fixed flex flex-col gap-3 z-50 p-2 w-content max-w-[340px]"
          :style="{
          left: `${positionX}px`,
          top: `${positionY}px`,
          opacity,
        }"
      >
        <div v-if="icon || title" class="flex gap-4 items-start">
          <Section
              v-if="icon"
              :tier="iconTier"
              class="w-[32px] h-[32px] flex items-center justify-center text-zinc-500"
          >
            <Icon :icon="icon" :width="22" />
          </Section>
          <div class="flex flex-col gap-1">
            <div
                class="leading-none text-zinc-300 whitespace-nowrap"
                v-html="title"
            />
            <div class="leading-none text-zinc-500 text-xs">{{ type }}</div>
          </div>
        </div>
        <div v-if="slots.default" class="flex flex-col gap-2">
          <slot />
        </div>
      </Section>
    </Teleport>
  </div>
</template>
