<script lang="ts" setup>
import Section from "./Section.vue";
import Slot from "./Slot.vue";
import Item from "./Item.vue";
import {useSlotsStore} from "../stores/slots";
import {useExpeditionsStore} from "../stores/expeditions";
import {Icon} from "@iconify/vue";
import Timer from "./Timer.vue";

const slotsStore = useSlotsStore();
const expeditionsStore = useExpeditionsStore();
</script>

<template>
  <div class="flex-1 flex flex-col gap-2">
    <div class="flex flex-1 gap-2">
      <template v-if="expeditionsStore.time === -1">
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
        <Section class="flex-1 flex flex-col p-2">
          <div
              class="flex-1 flex items-center justify-center p-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
          >
            <Timer/>
          </div>
        </Section>
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
