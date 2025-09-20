import {defineStore} from "pinia";
import {EVENT_PLAYER_STATS} from "../pkg/events";
import {computed, ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const usePlayerStatsStore = defineStore("player-stats", () => {
    const echo = useEchoStore();

    const attack = ref<number>(0);
    const attackSpeed = ref<number>(0);
    const defense = ref<number>(0);
    const energyRegeneration = ref<number>(0);
    const energyRegenerationInterval = ref<number>(0);

    const energyPerSecond = computed(() => {
        if (!energyRegenerationInterval.value || energyRegenerationInterval.value === 0) {
            return 0;
        }

        return ((energyRegeneration.value / energyRegenerationInterval.value) * 1000).toFixed(2);
    })

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<PlayerStats>(value);

            if (message.event === EVENT_PLAYER_STATS) {
                attack.value = message.data.attack;
                attackSpeed.value = message.data.attack_speed;
                defense.value = message.data.defense;
                energyRegeneration.value = message.data.energy_regeneration;
                energyRegenerationInterval.value = message.data.energy_regeneration_interval;
            }
        }
    );

    return {
        attack,
        attackSpeed,
        defense,
        energyRegeneration,
        energyRegenerationInterval,
        energyPerSecond
    };
});

export type PlayerStats = {
    attack: number;
    attack_speed: number;
    defense: number;
    energy_regeneration: number;
    energy_regeneration_interval: number;
};
