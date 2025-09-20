<script lang="ts" setup>
import {Icon} from "@iconify/vue";
import Tooltip from "./Tooltip.vue";
import {usePlayerResourceStore} from "../stores/player-resource";
import {formatNumber} from "../pkg/utils";
import {useMetaStore} from "../stores/meta";
import {usePlayerStore} from "../stores/player";
import {computed} from "vue";
import {useSlotsStore} from "../stores/slots";

const playerResourceStore = usePlayerResourceStore();
const playerStore = usePlayerStore();
const metaStore = useMetaStore();
const slotsStore = useSlotsStore();

const progress = computed(() => {
  const nextLevelExp = metaStore.levelToExp[playerStore.level + 1];

  if (nextLevelExp === 0 || !playerStore.player) {
    return (0).toFixed(4);
  }

  return ((playerStore.player.exp / nextLevelExp) * 100).toFixed(4);
});
</script>

<template>
  <div
      class="relative h-[24px] flex border-t border-x border-secondary pb-[2px] w-full max-w-[1790px] mx-auto px-2"
  >
    <div
        class="flex items-center leading-none text-sm px-2 gap-4 text-zinc-400 w-full"
    >
      <div>
        <Tooltip
            icon="game-icons:open-book"
            title="Experience"
            type="progression"
        >
          <template #trigger>
            <div class="flex items-center gap-2">
              <Icon icon="game-icons:open-book"/>
              {{ progress }}%
            </div>
          </template>
          <div class="text-sm text-zinc-500 leading-none">
            Gain experience points by hunting monsters.
          </div>
          <div v-if="playerStore.player" class="text-sm text-zinc-500 leading-none">
            {{ formatNumber(playerStore.player.exp) }} /
            {{ formatNumber(metaStore.levelToExp[playerStore.level + 1]) }}
          </div>
        </Tooltip>
      </div>
      <div class="flex items-center gap-4 ml-auto">
        <div>
          <Tooltip icon="game-icons:two-coins" title="Cin" type="currency">
            <template #trigger>
              <div class="flex items-center gap-2">
                <Icon icon="game-icons:two-coins"/>
                {{ formatNumber(slotsStore.cin) }}
              </div>
            </template>
            <div class="text-sm text-zinc-500 leading-none">
              The primary currency of the realm.
            </div>
          </Tooltip>
        </div>
        <div>
          <Tooltip icon="game-icons:weight" title="Weight" type="progression">
            <template #trigger>
              <div class="flex items-center gap-2">
                <Icon icon="game-icons:weight"/>
                {{ formatNumber(slotsStore.weight) }} /
                {{ formatNumber(playerResourceStore.weightLimit) }}
              </div>
            </template>
            <div class="text-sm text-zinc-500 leading-none">
              Each item has a weight to it, make sure you manage it well.
              Exceeding the weight limit will result in significant combat
              penalties.
            </div>
          </Tooltip>
        </div>
        <div>
          <Tooltip
              icon="game-icons:swap-bag"
              title="Inventory slots"
              type="progression"
          >
            <template #trigger>
              <div class="flex items-center gap-2">
                <Icon icon="game-icons:swap-bag"/>
                {{ formatNumber(slotsStore.filled) }} /
                {{ formatNumber(slotsStore.inventory.length) }}
              </div>
            </template>
            <div class="text-sm text-zinc-500 leading-none">
              Used inventory slots vs total available.
            </div>
          </Tooltip>
        </div>
      </div>
    </div>
    <div class="absolute left-0 bottom-0 h-[2px] bg-black w-screen">
      <div
          :style="{ maxWidth: `${progress}%` }"
          class="h-[2px] bg-primary w-screen transition-all"
      />
    </div>
  </div>
</template>
