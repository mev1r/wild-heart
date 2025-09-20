import {defineStore} from "pinia";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";
import {EVENT_PLAYER_STATE, EVENT_TOGGLE_LOOT} from "../pkg/events.ts";

export const usePlayerStateStore = defineStore("player-state", () => {
    const echo = useEchoStore();

    const state = ref<PlayerState>();

    function loot() {
        echo.sendMessage(EVENT_TOGGLE_LOOT);
    }

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<PlayerState>(value);

            if (message.event === EVENT_PLAYER_STATE) {
                state.value = message.data;
            }
        }
    );

    return {
        state,

        loot,
    };
});

export type PlayerState = {
    is_looting: boolean;
};
