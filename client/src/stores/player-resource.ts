import {defineStore} from "pinia";
import {EVENT_PLAYER_RESOURCE} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const usePlayerResourceStore = defineStore("player-resource", () => {
    const echo = useEchoStore();

    const energy = ref<number>(0);
    const energyMax = ref<number>(0);
    const hp = ref<number>(0);
    const hpMax = ref<number>(0);
    const mp = ref<number>(0);
    const mpMax = ref<number>(0);
    const weightLimit = ref<number>(0);

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<PlayerResource>(value);

            if (message.event === EVENT_PLAYER_RESOURCE) {
                energy.value = message.data.energy;
                energyMax.value = message.data.max_energy;
                hp.value = message.data.hp;
                hpMax.value = message.data.max_hp;
                mp.value = message.data.mp;
                mpMax.value = message.data.max_mp;
                weightLimit.value = message.data.weight_limit;
            }
        }
    );

    return {
        energy,
        energyMax,
        hp,
        hpMax,
        mp,
        mpMax,
        weightLimit
    };
});

export type PlayerResource = {
    energy: number;
    max_energy: number;
    hp: number;
    max_hp: number;
    mp: number;
    max_mp: number;
    weight_limit: number;
};
