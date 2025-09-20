<script lang="ts" setup>
import {usePlayerStatsStore} from "../stores/player-stats";
import Section from "./Section.vue";
import {formatNumber} from "../pkg/utils";
import {usePlayerAttributesStore} from "../stores/player-attributes";
import {computed} from "vue";
import {useMetaStore} from "../stores/meta.ts";

const playerStatsStore = usePlayerStatsStore();
const playerAttributesStore = usePlayerAttributesStore();
const metaStore = useMetaStore()

const attackSpeed = computed(() => {
  if (!metaStore.baseStats) {
    return 0;
  }

  const effectiveMs = playerStatsStore.attackSpeed ?? metaStore.baseStats.base_attack_speed;

  return (1000 / effectiveMs).toFixed(2);
});
</script>

<template>
  <Section class="p-2 flex flex-col flex-1 gap-2">
    <div
        class="flex flex-col flex-1 gap-2 p-2 h-[280px] scroller shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
    >
      <div
          class="flex-1 grid grid-cols-2 gap-4 text-sm leading-none text-silver"
      >
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between">
            <div>Attack:</div>
            <div>{{ formatNumber(playerStatsStore.attack) }}</div>
          </div>
          <div class="flex items-center justify-between">
            <div>Attack Speed:</div>
            <div>{{ formatNumber(attackSpeed) }}/s</div>
          </div>
          <div class="flex items-center justify-between">
            <div>Energy Regeneration:</div>
            <div>{{ formatNumber(playerStatsStore.energyPerSecond) }}/s</div>
          </div>
        </div>
      </div>
    </div>
    <div
        class="flex flex-col gap-2 p-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
    >
      <div class="flex gap-2 leading-none">
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">STR</div>
          <div class="text-2xl leading-none">{{ playerAttributesStore.strength }}</div>
        </Section>
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">DEX</div>
          <div class="text-2xl leading-none">
            {{ playerAttributesStore.dexterity }}
          </div>
        </Section>
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">VIT</div>
          <div class="text-2xl leading-none">{{ playerAttributesStore.vitality }}</div>
        </Section>
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">INT</div>
          <div class="text-2xl leading-none">
            {{ playerAttributesStore.intelligence }}
          </div>
        </Section>
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">SPI</div>
          <div class="text-2xl leading-none">{{ playerAttributesStore.spirit }}</div>
        </Section>
        <Section class="flex flex-col items-center flex-1 p-2">
          <div class="text-silver leading-none">LUC</div>
          <div class="text-2xl leading-none">{{ playerAttributesStore.luck }}</div>
        </Section>
      </div>
    </div>
  </Section>
</template>
