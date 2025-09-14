import {defineStore} from "pinia";
import {ref, watch} from "vue";
import {EVENT_META} from "../pkg/events.ts";
import {useEchoStore} from "./echo.ts";

export const useMetaStore = defineStore("meta", () => {
    const echo = useEchoStore();

    const levelToExp = ref<LevelToExp>({});
    const baseStats = ref<BaseStats>()

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<Meta>(value);

            if (message.event === EVENT_META) {
                levelToExp.value = message.data.level_to_exp;
                baseStats.value = message.data.base_stats;
            }
        }
    );

    return {
        levelToExp,
        baseStats
    };
});

export type Meta = {
    level_to_exp: LevelToExp;
    base_stats: BaseStats;
}

export type LevelToExp = Record<number, number>;

export type BaseStats = {
    base_attack_speed: number
}
