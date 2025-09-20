<script lang="ts" setup>
import {nextTick, ref, watch} from "vue";
import {SocketResponse, useEchoStore} from "../stores/echo";
import {useDateFormat} from "@vueuse/core";
import {EVENT_LOG} from "../pkg/events";
import Section from "./Section.vue";

type Log = {
  timestamp: Date;
  message: string;
};

const echo = useEchoStore();

const logs = ref<SocketResponse<Log>[]>([]);

const logsContainer = ref<HTMLDivElement | null>(null);

watch(
    () => echo.data,
    async (value: string) => {
      const message = echo.parsePayload<Log>(value);
      if (message.event === EVENT_LOG) {
        logs.value.push(message);
        await nextTick();
        if (logsContainer.value) {
          logsContainer.value.scrollTop = logsContainer.value.scrollHeight;
        }
      }
    }
);

watch(logs, () => {
  if (logs.value.length > 150) {
    logs.value.shift();
  }
});
</script>

<template>
  <Section class="relative flex flex-col overflow-hidden h-[214px]">
    <div class="relative z-10 flex-1 flex flex-col py-2 px-2">
      <div
          ref="logsContainer"
          class="flex flex-col gap-1 overflow-auto scroller h-[196px] text-sm p-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
      >
        <TransitionGroup appear name="chat">
          <div
              v-for="log in logs"
              :key="`${log.id}`"
              class="leading-none text-global-chat"
          >
            <span class="text-zinc-500 mr-1">
              {{ useDateFormat(log.data.timestamp, "HH:mm:ss") }}:
            </span>
            <span v-html="log.data.message"/>
          </div>
        </TransitionGroup>
      </div>
    </div>
  </Section>
</template>
