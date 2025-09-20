import {defineStore} from "pinia";
import {EVENT_PLAYER_RESOURCE} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const usePlayerResourceStore = defineStore("player-resource", () => {
    const echo = useEchoStore();

    const energy = ref<number>(0);
    const energyMax = ref<number>(0);
    const weightLimit = ref<number>(0);

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<PlayerResource>(value);

            if (message.event === EVENT_PLAYER_RESOURCE) {
                energy.value = message.data.energy;
                energyMax.value = message.data.max_energy;
                weightLimit.value = message.data.weight_limit;
            }
        }
    );

    return {
        energy,
        energyMax,
        weightLimit
    };
});

export type PlayerResource = {
    energy: number;
    max_energy: number;
    weight_limit: number;
};
