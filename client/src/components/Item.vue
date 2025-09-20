<script lang="ts" setup>
import {Icon} from "@iconify/vue";
import {Item, ItemStats} from "../stores/slots";
import Tooltip from "./Tooltip.vue";
import Section from "./Section.vue";
import {formatNumber, toReadableText} from "../pkg/utils";
import {useMetaStore} from "../stores/meta.ts";
import {computed} from "vue";

type Props = {
  item: Item;
  iconSize?: number;
};

const {item, iconSize = 28} = defineProps<Props>();

const metaStore = useMetaStore();

const title = computed(() => {
  return `${item.enchanted > 0 ? `+${item.enchanted} ` : ""}${item.name}${item.kind === 'Currency' ? ` <span class='text-zinc-500'>(${formatNumber(item.quantity)})</span>` : ''}${
      item.level > 0 ? ` <span class='text-primary'>Lv.${item.level}<span>` : ""
  }`;
});

function getAttackSpeed(value?: number) {
  if (!value || !metaStore.baseStats) {
    return 0;
  }

  const effectiveMs = metaStore.baseStats.base_attack_speed + value;

  return (metaStore.baseStats.base_attack_speed / effectiveMs).toFixed(2);
}

function getStats(stats: ItemStats): Partial<ItemStats> {
  return Object.fromEntries(
      Object.entries(stats).filter(([_, value]) => value != null)
  ) as Partial<ItemStats>;
}
</script>

<template>
  <Tooltip
      :icon="item.icon"
      :icon-tier="item.tier"
      :tier="item.tier"
      :title="title"
      :type="item.kind"
  >
    <template #trigger>
      <Section
          :tier="item.tier"
          class="flex items-center justify-center w-full h-full"
      >
        <Icon :icon="item.icon" :width="iconSize" class="text-zinc-500"/>
        <span
            v-if="item.quantity > 1 && item.kind !== 'Currency'"
            class="absolute text-[9px] left-[2px] bottom-[2px] leading-none font-timer"
        >
          {{ item.quantity > 99 ? "99+" : item.quantity }}
        </span>
        <span
            v-if="item.enchanted > 0"
            class="absolute text-[9px] left-[2px] bottom-[2px] leading-none text-silver font-bold font-timer"
        >
          {{ `+${item.enchanted}` }}
        </span>
        <span
            v-if="item.level > 0"
            class="absolute text-[9px] right-[2px] bottom-[2px] leading-none text-silver font-timer"
        >
          Lv{{ `${item.level}` }}
        </span>
      </Section>
    </template>
    <div class="text-xs text-zinc-500 leading-none">
      {{ item.description }}
    </div>
    <div
        v-if="item.stats"
        class="text-sm text-zinc-500 leading-none flex flex-col gap-1 py-2 border-y border-secondary/50"
    >
      <div v-for="(value, name) in getStats(item.stats)">
        <template v-if="name === 'attack_speed' && typeof value === 'number'">
          {{ toReadableText(name) }}:
          <span class="text-silver">{{ getAttackSpeed(value) }}</span>
        </template>
        <template v-else>
          {{ toReadableText(name) }}:
          <span class="text-silver">{{ value }}</span>
        </template>
      </div>
    </div>
    <div class="flex items-center gap-1 text-xs text-zinc-500 leading-none">
      <Icon icon="game-icons:weight"/>
      {{ formatNumber(item.weight.toFixed(2)) }}
    </div>
  </Tooltip>
</template>
