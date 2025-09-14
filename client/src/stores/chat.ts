import {defineStore} from "pinia";
import {EVENT_CHAT_MESSAGE, EVENT_SEND_CHAT_MESSAGE} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const useChatStore = defineStore("chat", () => {
    const echo = useEchoStore();

    const chats = ref<Chat[]>([]);

    function chat(payload: ChatPayload) {
        echo.sendMessage(EVENT_SEND_CHAT_MESSAGE, payload);
    }

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<Chat>(value);

            if (message.event === EVENT_CHAT_MESSAGE) {
                chats.value.push(message.data);
            }
        }
    );

    return {
        chats,

        chat,
    };
});

export type Chat = {
    id: string;
    sender: string;
    recipient?: string;
    kind: ChatKind;
    content: string;
    timestamp: Date;
};

export type ChatPayload = {
    recipient?: string;
    kind: ChatKind;
    content: string;
};

export type ChatKind = "General" | "Trade" | "Whisper";
