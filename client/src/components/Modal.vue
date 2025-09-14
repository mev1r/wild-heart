<script lang="ts" setup>
import {useDraggable, useStorage, useWindowSize} from "@vueuse/core";
import {computed, ref, watch} from "vue";
import Section from "./Section.vue";
import {Icon} from "@iconify/vue";

type Props = {
  id: string;
  title: string;
  width: number;
};
const props = defineProps<Props>();
const emits = defineEmits(["close"]);

const {width: windowWidth, height: windowHeight} = useWindowSize();

const saved = useStorage<{ x: number; y: number }>(`modal-pos:${props.id}`, {
  x: 100,
  y: 100,
});

const modalRef = ref<{ el: HTMLElement | null } | null>(null);

const x = ref(saved.value.x);
const y = ref(saved.value.y);

const {style} = useDraggable(
    computed(() => modalRef.value?.el ?? null),
    {
      initialValue: {x: x.value, y: y.value},
      onMove: (position) => {
        const el = modalRef.value?.el;
        if (!el) {
          return;
        }

        const rect = el.getBoundingClientRect();
        const maxX = windowWidth.value - rect.width;
        const maxY = windowHeight.value - rect.height;

        position.x = Math.max(0, Math.min(position.x, maxX));
        position.y = Math.max(42, Math.min(position.y, maxY));

        x.value = position.x;
        y.value = position.y;
      },
    }
);

watch([x, y], ([newX, newY]) => {
  saved.value = {x: newX, y: newY};
});
</script>

<template>
  <Teleport to="#modals">
    <Section
        ref="modalRef"
        :style="[{ width: props.width + 'px' }, style]"
        class="fixed text-zinc-100"
    >
      <div
          class="text-center leading-none px-2 py-2 flex items-center justify-between"
      >
        <div class="w-[16px]"/>
        {{ title }}
        <Icon
            class="text-zinc-500 hover:text-zinc-300 transition-all active:scale-[0.95]"
            icon="game-icons:tire-iron-cross"
            @click="emits('close')"
        />
      </div>
      <slot/>
    </Section>
  </Teleport>
</template>
