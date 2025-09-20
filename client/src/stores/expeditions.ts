import {defineStore} from "pinia";
import {EVENT_END_EXPEDITION, EVENT_EXPEDITION_COUNTUP, EVENT_START_EXPEDITION} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";

export const useExpeditionsStore = defineStore("expeditions", () => {
    const echo = useEchoStore();

    const time = ref<number>(-1);

    function start() {
        echo.sendMessage(EVENT_START_EXPEDITION);
    }

    function leave() {
        echo.sendMessage(EVENT_END_EXPEDITION);
        time.value = -1;
    }

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<number>(value);

            if (message.event === EVENT_EXPEDITION_COUNTUP) {
                time.value = message.data;
            }
        }
    );

    return {
        time,

        start,
        leave
    };
});