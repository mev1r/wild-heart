<script lang="ts" setup>
import {Icon} from "@iconify/vue";
import {Mob} from "../stores/expeditions.ts";
import Section from "./Section.vue";
import ResourceBar from "./ResourceBar.vue";
import {usePlayerStateStore} from "../stores/player-state.ts";

type Props = {
  mob: Mob
}

defineProps<Props>()

const playerStateStore = usePlayerStateStore()
</script>

<template>
  <Section :class="{'ring-2 ring-inset ring-primary': mob.id === playerStateStore.state?.target_id}"
           class="py-2 flex flex-col justify-between">
    <div class="px-4 leading-none text-zinc-500 text-center">{{ mob.name }}</div>
    <div class="flex items-center justify-center">
      <Icon :width="64" class="text-zinc-500" icon="game-icons:orc-head"/>
    </div>
    <div class="px-4">
      <ResourceBar
          :current="mob.hp"
          :max="mob.max_hp"
          :size="14"
          color="health-bg"
      />
    </div>
  </Section>
</template>