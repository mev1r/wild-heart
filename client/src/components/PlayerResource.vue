<script lang="ts" setup>
import Tooltip from "./Tooltip.vue";
import ResourceBar from "./ResourceBar.vue";
import LevelPlack from "./LevelPlack.vue";
import {usePlayerStore} from "../stores/player";
import {usePlayerResourceStore} from "../stores/player-resource";
import {formatNumber} from "../pkg/utils";
import {usePlayerStatsStore} from "../stores/player-stats.ts";

const playerStore = usePlayerStore();
const playerResourceStore = usePlayerResourceStore();
const playerStatsStore = usePlayerStatsStore()
</script>

<template>
  <div class="flex gap-2">
    <LevelPlack :level="playerStore.level"/>
    <div class="relative flex flex-col flex-1">
      <Tooltip icon="game-icons:power-lightning" title="Energy" type="resource">
        <template #trigger>
          <ResourceBar
              :current="playerResourceStore.energy"
              :max="playerResourceStore.energyMax"
              :size="24"
              color="energy-bg"
          />
        </template>
        <div class="text-sm text-zinc-500 leading-none">
          A main resource that fuels compass expeditions.
        </div>
        <div class="flex flex-col gap-1">
          <div class="text-sm text-zinc-400 leading-none">
            Max Energy: {{ formatNumber(playerResourceStore.energyMax) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Remaining Energy: {{ formatNumber(playerResourceStore.energy) }}
          </div>
          <div class="text-sm text-zinc-400 leading-none">
            Energy Regeneration Rate: {{ formatNumber(playerStatsStore.energyPerSecond) }}/s
          </div>
        </div>
      </Tooltip>
    </div>
  </div>
</template>
