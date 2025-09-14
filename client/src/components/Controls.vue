<script lang="ts" setup>
import {Icon} from "@iconify/vue";
import Section from "./Section.vue";
import Slot from "./Slot.vue";
import Tooltip from "./Tooltip.vue";
import {useSlotsStore} from "../stores/slots";
import Item from "./Item.vue";
import GlassCover from "./GlassCover.vue";
import {usePlayerStateStore} from "../stores/player-state";
import {useExpeditionsStore} from "../stores/expeditions.ts";
import {usePlayerResourceStore} from "../stores/player-resource.ts";
import {computed} from "vue";

const slotsStore = useSlotsStore();
const expeditionsStore = useExpeditionsStore();
const playerStateStore = usePlayerStateStore();
const playerResourceStore = usePlayerResourceStore();

const canResurrect = computed(() => {
  return playerResourceStore.hp <= 0
})
</script>

<template>
  <div class="flex justify-center gap-2">
    <Section class="p-2">
      <div class="relative z-10 grid grid-cols-8 gap-2 content-start">
        <Slot v-for="slot in slotsStore.rune" :key="slot.index" :slot="slot">
          <Item v-if="slot.item" :item="slot.item"/>
        </Slot>
      </div>
    </Section>
    <Section class="p-2">
      <div class="relative z-10 grid grid-cols-4 gap-2 content-start">
        <Slot v-for="slot in slotsStore.consumable" :key="slot.index" :slot="slot">
          <Item v-if="slot.item" :item="slot.item"/>
        </Slot>
      </div>
    </Section>
    <Section class="p-2 flex-1">
      <div class="relative z-10 flex flex-col gap-2 content-start">
        <div class="flex flex-wrap gap-2">
          <Slot
              v-if="slotsStore.compass"
              :key="slotsStore.compass.index"
              :slot="slotsStore.compass"
              :icon-size="64"
              :size="88"
              icon="game-icons:compass"
          >
            <Item
                v-if="slotsStore.compass.item"
                :icon-size="64"
                :item="slotsStore.compass.item"
            />
          </Slot>
          <div class="flex flex-col gap-2">
            <div>
              <Tooltip
                  :class="{
                  'pointer-events-none opacity-50':
                    !slotsStore.compass?.item || expeditionsStore.duration > 0,
                }"
                  icon="game-icons:twirl-center"
                  title="Enter Expedition"
                  type="action"
              >
                <template #trigger>
                  <Section
                      class="w-[40px] h-[40px] flex items-center justify-center text-zinc-500 active:scale-95 active:brightness-100 transition-all hover:brightness-130"
                      @click="expeditionsStore.start"
                  >
                    <Icon :width="24" icon="game-icons:twirl-center"/>
                  </Section>
                </template>
                <div class="text-sm text-zinc-500 leading-none">
                  Enter Expedition.
                </div>
              </Tooltip>
            </div>
            <div>
              <Tooltip
                  :class="{
                  'pointer-events-none opacity-50':
                    !slotsStore.compass?.item || expeditionsStore.duration === 0,
                }"
                  icon="game-icons:run"
                  title="Leave Expedition"
                  type="action"
              >
                <template #trigger>
                  <Section
                      class="w-[40px] h-[40px] flex items-center justify-center text-zinc-500 active:scale-95 active:brightness-100 transition-all hover:brightness-130"
                      @click="expeditionsStore.leave"
                  >
                    <Icon :width="24" icon="game-icons:run"/>
                  </Section>
                </template>
                <div class="text-sm text-zinc-500 leading-none">
                  Leave Expedition.
                </div>
              </Tooltip>
            </div>
          </div>
        </div>
        <div class="flex gap-2 items-center">
          <div>
            <Tooltip
                :class="{
                'pointer-events-none opacity-50': expeditionsStore.duration === 0,
              }"
                icon="game-icons:pointy-sword"
                title="Start Attacking"
                type="action"
            >
              <template #trigger>
                <Section
                    class="w-[40px] h-[40px] flex items-center justify-center text-zinc-500 active:scale-95 active:brightness-100 transition-all hover:brightness-130"
                    @click="playerStateStore.attack"
                >
                  <Icon :width="24" icon="game-icons:pointy-sword"/>
                  <Transition name="fade">
                    <GlassCover v-if="playerStateStore.state?.is_attacking"/>
                  </Transition>
                </Section>
              </template>
              <div class="text-sm text-zinc-500 leading-none">
                Start Attacking.
              </div>
            </Tooltip>
          </div>
          <div>
            <Tooltip
                :class="{
                  'pointer-events-none opacity-50': expeditionsStore.duration === 0,
                }"
                icon="game-icons:card-pickup"
                title="Auto Loot Items"
                type="action"
            >
              <template #trigger>
                <Section
                    class="relative w-[40px] h-[40px] flex items-center justify-center text-zinc-500 active:scale-95 active:brightness-100 transition-all hover:brightness-130"
                    @click="playerStateStore.loot"
                >
                  <Icon :width="24" icon="game-icons:card-pickup"/>
                  <Transition name="fade">
                    <GlassCover v-if="playerStateStore.state?.is_looting"/>
                  </Transition>
                </Section>
              </template>
              <div class="text-sm text-zinc-500 leading-none">
                Start Looting Items.
              </div>
            </Tooltip>
          </div>
          <div>
            <Tooltip
                :class="{
                  'pointer-events-none opacity-50': !canResurrect,
                }"
                icon="game-icons:ankh"
                title="Resurrect"
                type="action"
            >
              <template #trigger>
                <Section
                    class="relative w-[40px] h-[40px] flex items-center justify-center text-zinc-500 active:scale-95 active:brightness-100 transition-all hover:brightness-130"
                    @click="playerStateStore.resurrect"
                >
                  <Icon :width="24" icon="game-icons:ankh"/>
                </Section>
              </template>
              <div class="text-sm text-zinc-500 leading-none">
                Brings the character back to life with 25% HP and 25% MP, but at
                the cost of 10% of total experience.
              </div>
            </Tooltip>
          </div>
        </div>
      </div>
    </Section>
  </div>
</template>
