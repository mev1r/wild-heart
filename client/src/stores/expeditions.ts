import {defineStore} from "pinia";
import {EVENT_END_EXPEDITION, EVENT_EXPEDITION_COUNTDOWN, EVENT_START_EXPEDITION} from "../pkg/events";
import {ref, watch} from "vue";
import {useEchoStore} from "./echo";
import {useSlotsStore} from "./slots";

export const useExpeditionsStore = defineStore("expeditions", () => {
    const echo = useEchoStore();
    const slots = useSlotsStore();

    const duration = ref<number>(0);
    const endTime = ref<number>(0);

    function start() {
        const huntDuration = slots.compass?.item?.stats?.expedition_duration;
        if (!huntDuration) {
            return;
        }

        duration.value = huntDuration;
        endTime.value = Date.now() + huntDuration;

        echo.sendMessage(EVENT_START_EXPEDITION);
    }

    function leave() {
        duration.value = 0
        endTime.value = 0;
        echo.sendMessage(EVENT_END_EXPEDITION);
    }

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<number>(value);

            if (message.event === EVENT_EXPEDITION_COUNTDOWN) {
                if (message.data <= 0) {
                    duration.value = 0
                    endTime.value = 0;
                    return;
                }
                const secsLeft = message.data + 1;
                if (duration.value === 0 && slots.compass?.item?.stats?.expedition_duration) {
                    duration.value = slots.compass.item.stats.expedition_duration;
                }
                endTime.value = Date.now() + secsLeft * 1000;
            }
        }
    );

    return {
        duration,
        endTime,

        start,
        leave
    };
});
