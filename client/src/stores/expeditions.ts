import {defineStore} from "pinia";
import {
    EVENT_END_EXPEDITION,
    EVENT_EXPEDITION_COUNTUP,
    EVENT_GAINED_CIN,
    EVENT_GAINED_EXPERIENCE,
    EVENT_START_EXPEDITION,
} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const useExpeditionsStore = defineStore("expeditions", () => {
    const echo = useEchoStore();

    const time = ref<number>(-1);
    const gainedExperience = ref<number>(0);
    const gainedCin = ref<number>(0);

    function start() {
        echo.sendMessage(EVENT_START_EXPEDITION);
    }

    function leave() {
        echo.sendMessage(EVENT_END_EXPEDITION);
        time.value = -1;
        reset()
    }

    function reset() {
        gainedExperience.value = 0
        gainedCin.value = 0
    }

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<number>(value);

            if (message.event === EVENT_EXPEDITION_COUNTUP) {
                time.value = message.data;

                if (time.value < 0) {
                    reset()
                }
            }
        }
    );

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<number>(value);

            if (message.event === EVENT_GAINED_EXPERIENCE) {
                gainedExperience.value += message.data;
            }
        }
    );

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<number>(value);

            if (message.event === EVENT_GAINED_CIN) {
                gainedCin.value += message.data;
            }
        }
    );

    return {
        time,
        gainedExperience,
        gainedCin,

        start,
        leave,
        reset
    };
});