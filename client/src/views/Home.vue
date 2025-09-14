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

const echo = useEchoStore();
const auth = useAuthStore();

onMounted(async () => {
  echo.connect(auth.token);

  const win = Window.getCurrent();
  await win.setSize(new LogicalSize(1600, 900));
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
      <Expedition/>
      <Logs class="mt-auto"/>
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
