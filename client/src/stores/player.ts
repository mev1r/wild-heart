import {defineStore} from "pinia";
import {computed, ref, watch} from "vue";
import {useEchoStore} from "./echo";
import {EVENT_PLAYER_INFO} from "../pkg/events";
import {useMetaStore} from "./meta.ts";

export const usePlayerStore = defineStore("player", () => {
    const echo = useEchoStore();
    const metaStore = useMetaStore();

    const player = ref<Player>();

    const level = computed<number>(() => {
        if (!player.value) {
            return 1;
        }

        for (const item of Object.entries(metaStore.levelToExp).reverse()) {
            const level = Number(item[0]);
            if (item[1] < player.value.exp) {
                return level;
            }
        }

        return 1;
    });

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<Player>(value);

            if (message.event === EVENT_PLAYER_INFO) {
                player.value = message.data;
            }
        }
    );

    return {
        player,
        level,
    };
});

export type Player = {
    email: string;
    name: string;
    exp: number;
    timestamp: Date;
};
