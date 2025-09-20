<script lang="ts" setup>
import Section from "./Section.vue";
import Slot from "./Slot.vue";
import Item from "./Item.vue";
import {useSlotsStore} from "../stores/slots";
import {useExpeditionsStore} from "../stores/expeditions";
import {Icon} from "@iconify/vue";

const slotsStore = useSlotsStore();
const expeditionsStore = useExpeditionsStore();
</script>

<template>
  <div class="flex-1 flex flex-col gap-2">
    <div class="flex flex-1 gap-2">
      <template v-if="expeditionsStore.time === 0">
        <Section class="flex flex-1 items-center justify-center">
          <Icon
              :width="64"
              class="text-zinc-700/20"
              icon="game-icons:triple-lock"
          />
        </Section>
        <Section class="w-[202px] flex items-center justify-center">
          <Icon
              :width="64"
              class="text-zinc-700/20"
              icon="game-icons:triple-lock"
          />
        </Section>
      </template>
      <template v-else>
        <div class="flex-1 flex items-center justify-center">

        </div>
        <Section class="p-2">
          <div class="relative z-10 grid grid-cols-4 gap-2 content-start">
            <Slot v-for="slot in slotsStore.ground" :key="slot.index" :slot="slot">
              <Item v-if="slot.item" :item="slot.item"/>
            </Slot>
          </div>
        </Section>
      </template>
    </div>
  </div>
</template>
