import {defineStore} from "pinia";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";
import {EVENT_PLAYER_RESURRECT, EVENT_PLAYER_STATE, EVENT_TOGGLE_ATTACK, EVENT_TOGGLE_LOOT} from "../pkg/events.ts";

export const usePlayerStateStore = defineStore("player-state", () => {
    const echo = useEchoStore();

    const state = ref<PlayerState>();

    function attack() {
        echo.sendMessage(EVENT_TOGGLE_ATTACK);
    }

    function loot() {
        echo.sendMessage(EVENT_TOGGLE_LOOT);
    }

    function resurrect() {
        echo.sendMessage(EVENT_PLAYER_RESURRECT);
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

        attack,
        loot,
        resurrect,
    };
});

export type PlayerState = {
    in_combat: boolean;
    is_attacking: boolean;
    is_looting: boolean;
};
