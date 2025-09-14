<script lang="ts" setup>
import {ref} from "vue";
import Section from "./Section.vue";
import {onStartTyping} from "@vueuse/core";
import {ChatKind, useChatStore} from "../stores/chat";
import {usePlayerStore} from "../stores/player";
import {Icon} from "@iconify/vue";

const chatStore = useChatStore();
const playerStore = usePlayerStore();

const chatInputRef = ref<HTMLInputElement | null>(null);
const chatContainer = ref<HTMLDivElement | null>(null);

const content = ref("");
const recipient = ref<string>();
const kind = ref<ChatKind>("General");

function sendChatMessage() {
  if (!content.value.trim() || !playerStore.player) {
    return;
  }

  chatStore.chat({
    recipient: recipient.value,
    kind: kind.value,
    content: content.value,
  });

  content.value = "";
}

onStartTyping(() => {
  chatInputRef.value?.focus();
});
</script>

<template>
  <Section class="relative flex mt-auto flex-col overflow-hidden">
    <div class="relative z-10 flex flex-col py-2 px-2 overflow-hidden">
      <div
          ref="chatContainer"
          class="flex flex-col gap-1 scroller h-[308px] p-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc]"
      >
        <TransitionGroup appear name="chat" tag="div">
          <div
              v-for="msg in chatStore.chats"
              :key="`${msg.id}`"
              class="leading-none py-[1px] font-chat text-global-chat"
          >
            <span> {{ msg.sender }}: </span>
            {{ msg.content }}
          </div>
        </TransitionGroup>
      </div>
    </div>
    <div class="px-2 mb-2 relative">
      <input
          ref="chatInputRef"
          v-model="content"
          class="w-full pl-4 pr-14 py-2 shadow-inner-custom bg-slot/60 border border-[#2a2a2acc] border-t-[#191919cc] border-b-[#3a3a3acc] text-global-chat focus:bg-input-bg/30 transition-colors relative z-10"
          placeholder="Send chat message..."
          @keypress.enter="sendChatMessage"
      />
      <Icon
          :width="28"
          class="absolute right-6 top-1/2 -translate-y-1/2 active:scale-95 active:brightness-100 text-zinc-500 transition-all hover:brightness-130 z-20"
          icon="game-icons:paper-plane"
          @click="sendChatMessage"
      />
    </div>
  </Section>
</template>
