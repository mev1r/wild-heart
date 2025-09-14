import {defineStore} from "pinia";
import {EVENT_PLAYER_STATS} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const usePlayerStatsStore = defineStore("player-stats", () => {
    const echo = useEchoStore();

    const attack = ref<number>(0);
    const attackSpeed = ref<number>(0);
    const hpRegeneration = ref<number>(0);
    const hpRegenerationInterval = ref<number>(0);

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<PlayerStats>(value);

            if (message.event === EVENT_PLAYER_STATS) {
                attack.value = message.data.attack;
                attackSpeed.value = message.data.attack_speed;
                hpRegeneration.value = message.data.hp_regeneration;
                hpRegenerationInterval.value = message.data.hp_regeneration_interval;
            }
        }
    );

    return {
        attack,
        attackSpeed,
        hpRegeneration,
        hpRegenerationInterval
    };
});

export type PlayerStats = {
    attack: number;
    attack_speed: number;
    hp_regeneration: number;
    hp_regeneration_interval: number;
};
