import {defineStore} from "pinia";
import {useWebSocket} from "@vueuse/core";
import {ref, watch} from "vue";
import {useAuthStore} from "./auth";

const {VITE_APP_WS_URL} = import.meta.env;
const MAX_RETRIES = 5;

export const useEchoStore = defineStore("echo", () => {
    const connected = ref(false);

    let ws: ReturnType<typeof useWebSocket> | null = null;
    const data = ref<string>("");

    function connect(token: string) {
        if (ws) {
            return;
        }

        const auth = useAuthStore();

        ws = useWebSocket(`${VITE_APP_WS_URL}?token=${token}`, {
            autoReconnect: {
                retries: MAX_RETRIES,
                delay: 2000,
            },
            onConnected: () => {
                connected.value = true;
            },
            onDisconnected: () => {
                connected.value = false;
            },
            onError: () => {
                ws?.close();
                auth.token = "";
                window.location.reload();
            }
        });

        watch(ws.data, (val) => {
            data.value = val as string;
        });
    }

    function sendMessage(event: string, data?: any) {
        if (!ws) {
            return;
        }
        ws.send(JSON.stringify({event, data}));
    }

    function parsePayload<T>(payload: string): SocketResponse<T> {
        return JSON.parse(payload);
    }

    return {
        connect,
        sendMessage,
        parsePayload,
        connected,
        data,
    };
});

export type SocketResponse<T> = {
    id: string;
    event: string;
    data: T;
};
