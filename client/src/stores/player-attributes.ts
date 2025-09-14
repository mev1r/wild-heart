import { defineStore } from "pinia";
import { EVENT_PLAYER_ATTRIBUTES } from "../pkg/events";
import { ref, watch } from "vue";
import { useEchoStore } from "./echo";

export const usePlayerAttributesStore = defineStore("player-attributes", () => {
    const echo = useEchoStore();

    const strength = ref<number>(0);
    const dexterity = ref<number>(0);
    const vitality = ref<number>(0);
    const intelligence = ref<number>(0);
    const spirit = ref<number>(0);
    const luck = ref<number>(0);

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<UserAttributes>(value);

            if (message.event === EVENT_PLAYER_ATTRIBUTES) {
                strength.value = message.data.strength;
                dexterity.value = message.data.dexterity;
                vitality.value = message.data.vitality;
                intelligence.value = message.data.intelligence;
                spirit.value = message.data.spirit;
                luck.value = message.data.luck;
            }
        }
    );

    return {
        strength,
        dexterity,
        vitality,
        intelligence,
        spirit,
        luck,
    };
});

export type UserAttributes = {
    strength: number;
    dexterity: number;
    vitality: number;
    intelligence: number;
    spirit: number;
    luck: number;
};
