<script setup lang="ts">
import Tooltip from "./Tooltip.vue";
import ResourceBar from "./ResourceBar.vue";
import LevelPlack from "./LevelPlack.vue";
import { usePlayerStore } from "../stores/player";
import { usePlayerResourceStore } from "../stores/player-resource";
import { formatNumber } from "../pkg/utils";

const playerStore = usePlayerStore();
const playerResourceStore = usePlayerResourceStore();
</script>

<template>
  <div class="flex gap-2">
    <LevelPlack :level="playerStore.level" />
    <div class="relative flex flex-col flex-1">
      <Tooltip title="Health" icon="game-icons:heart-beats" type="resource">
        <template #trigger>
          <ResourceBar
              :max="playerResourceStore.hpMax"
              :current="playerResourceStore.hp"
              color="health-bg"
              :size="24"
          />
        </template>
        <div class="text-sm text-zinc-500 leading-none">
          Represents your vitality. If it drops to zero, you die. Health
          regenerates slowly over time and can be restored through healing
          effects.
        </div>
        <div class="flex flex-col gap-1">
          <div class="text-sm text-zinc-400 leading-none">
            Max Health: {{ formatNumber(playerResourceStore.hpMax) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Remaining Health: {{ formatNumber(playerResourceStore.hp) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Health Regeneration: 5
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Health Regeneration Rate: 10s
          </div>
        </div>
      </Tooltip>
      <Tooltip title="Mana" icon="game-icons:water-drop" type="resource">
        <template #trigger>
          <ResourceBar
              :max="playerResourceStore.mpMax"
              :current="playerResourceStore.mp"
              color="mana-bg"
          />
        </template>
        <div class="text-sm text-zinc-500 leading-none">
          Used to cast spells and activate abilities. Mana regenerates over time
          and is essential for offensive and defensive magic.
        </div>
        <div class="flex flex-col gap-1">
          <div class="text-sm text-zinc-400 leading-none">
            Max Mana: {{ formatNumber(playerResourceStore.mpMax) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Remaining Mana: {{ formatNumber(playerResourceStore.mp) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Mana Regeneration: 5
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Mana Regeneration Rate: 10s
          </div>
        </div>
      </Tooltip>
      <Tooltip title="Energy" icon="game-icons:power-lightning" type="resource">
        <template #trigger>
          <ResourceBar
              :max="playerResourceStore.energyMax"
              :current="playerResourceStore.energy"
              color="energy-bg"
              :size="6"
              :numbers="false"
          />
        </template>
        <div class="text-sm text-zinc-500 leading-none">
          A civil resource that fuels activities like crafting, researching, and
          exploration. While Energy is above zero, you gain increased experience
          from all actions. It is consumed gradually through most non-combat
          tasks, and partially when killing enemies.
        </div>
        <div class="flex flex-col gap-1">
          <div class="text-sm text-zinc-400 leading-none">
            Max Energy: {{ formatNumber(playerResourceStore.energyMax) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Remaining Energy: {{ formatNumber(playerResourceStore.energy) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Energy Regeneration: 5
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Energy Regeneration Rate: 10s
          </div>
        </div>
      </Tooltip>
    </div>
  </div>
</template>
