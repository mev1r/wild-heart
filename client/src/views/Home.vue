<script lang="ts" setup>
import {LogicalSize, Window} from "@tauri-apps/api/window";
import {onMounted} from "vue";
import {useAuthStore} from "../stores/auth";
import {useEchoStore} from "../stores/echo.ts";
import PlayerResource from "../components/PlayerResource.vue";
import Inventory from "../components/Inventory.vue";
import Hand from "../components/Hand.vue";
import Gear from "../components/Gear.vue";
import BottomBar from "../components/BottomBar.vue";
import PlayerInfo from "../components/PlayerInfo.vue";
import Chat from "../components/Chat.vue";
import GameLoader from "../components/GameLoader.vue";
import Expedition from "../components/Expedition.vue";
import Controls from "../components/Controls.vue";
import Logs from "../components/Logs.vue";
import {useSlotsStore} from "../stores/slots.ts";
import Section from "../components/Section.vue";
import Item from "../components/Item.vue";
import Slot from "../components/Slot.vue";
import {Icon} from "@iconify/vue";
import {useExpeditionsStore} from "../stores/expeditions.ts";

const echo = useEchoStore();
const authStore = useAuthStore();
const slotsStore = useSlotsStore()
const expeditionsStore = useExpeditionsStore();

onMounted(async () => {
  echo.connect(authStore.token);

  const win = Window.getCurrent();
  await win.setSize(new LogicalSize(1600, 870));
  await win.center();
});
</script>

<template>
  <main class="flex-1 flex justify-between gap-2 px-2 pb-2">
    <div class="w-full max-w-[408px] flex flex-col gap-2">
      <PlayerResource/>
      <PlayerInfo/>
      <Chat/>
    </div>
    <div class="flex flex-1 flex-col gap-2">
      <div class="flex gap-2 flex-1 w-full">
        <div class="flex-1 flex flex-col gap-2">
          <Expedition/>
          <Logs class="mt-auto"/>
        </div>
        <div class="flex flex-col">
          <Section v-if="expeditionsStore.time === -1" class="w-[202px] flex flex-1 items-center justify-center">
            <Icon
                :width="64"
                class="text-zinc-700/20"
                icon="game-icons:triple-lock"
            />
          </Section>
          <Section v-else class="p-2 flex-1">
            <div class="relative z-10 grid grid-cols-4 gap-2 content-start">
              <Slot v-for="slot in slotsStore.ground" :key="slot.index" :slot="slot">
                <Item v-if="slot.item" :item="slot.item"/>
              </Slot>
            </div>
          </Section>
        </div>
      </div>
      <Controls/>
    </div>
    <div class="w-full max-w-[394px] flex flex-col gap-2">
      <Gear/>
      <Inventory/>
    </div>
  </main>
  <Hand/>
  <BottomBar/>
  <GameLoader v-if="!echo.connected"/>
</template>
