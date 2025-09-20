<script lang="ts" setup>
import Section from "./Section.vue";
import {useExpeditionsStore} from "../stores/expeditions";
import {Icon} from "@iconify/vue";
import Timer from "./Timer.vue";
import {formatNumber} from "../pkg/utils.ts";

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
      </template>
      <Section v-else class="flex-1 flex flex-col p-2">
        <div
            class="relative flex-1 flex items-center justify-center p-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
        >
          <Timer/>
          <div class="absolute left-4 bottom-4 leading-none text-center">
            <div class="text-sm leading-none text-zinc-500 mb-1">Gained Experience:</div>
            <span class="font-timer">{{ formatNumber(expeditionsStore.gainedExperience) }}</span>
            Exp
          </div>
          <div class="absolute right-4 bottom-4 leading-none text-center">
            <div class="text-sm leading-none text-zinc-500 mb-1">Gained Cin:</div>
            <span class="font-timer">{{ formatNumber(expeditionsStore.gainedCin) }}</span>
            Cin
          </div>
        </div>
      </Section>
    </div>
  </div>
</template>
