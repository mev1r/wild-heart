import {defineStore} from "pinia";
import {EVENT_SLOTS} from "../pkg/events";
import {computed, ref, watch} from "vue";
import {useEchoStore} from "./echo";
import {ItemKind, SlotKind, Tier} from "../types";
import {
    ARMOR_SLOT,
    BELT_SLOT,
    BOOKS_SLOT,
    CLOAK_SLOT,
    COMPASS_SLOT,
    CONSUMABLE_SLOT,
    EARRING_SLOT,
    GLOVES_SLOT,
    GROUND_SLOT,
    HAND_SLOT,
    HELMET_SLOT,
    INVENTORY_SLOT,
    MASK_SLOT,
    NECKLACE_SLOT,
    PANTS_SLOT,
    PENDANT_SLOT,
    RING_SLOT,
    RUNE_SLOT,
    SHOULDERS_SLOT,
    WEAPON_SLOT,
} from "../pkg/slots";

export const useSlotsStore = defineStore("slots", () => {
    const echo = useEchoStore();

    const hand = ref<Slot>();
    const inventory = ref<Slot[]>([]);
    const compass = ref<Slot>();
    const weapon = ref<Slot>();
    const shoulders = ref<Slot>();
    const ring = ref<Slot[]>([]);
    const pendant = ref<Slot>();
    const pants = ref<Slot>();
    const necklace = ref<Slot>();
    const mask = ref<Slot>();
    const helmet = ref<Slot>();
    const gloves = ref<Slot>();
    const earring = ref<Slot[]>();
    const cloak = ref<Slot>();
    const boots = ref<Slot>();
    const belt = ref<Slot>();
    const armor = ref<Slot>();
    const rune = ref<Slot[]>([]);
    const consumable = ref<Slot[]>([]);
    const ground = ref<Slot[]>([]);

    const filled = computed(() => {
        return inventory.value.reduce((memo, current) => {
            if (current.item) {
                memo++;
            }
            return memo;
        }, 0);
    });

    const weight = computed(() => {
        return inventory.value.reduce((memo, current) => {
            if (current.item) {
                memo += current.item.weight;
            }
            return memo;
        }, 0);
    });

    watch(
        () => echo.data,
        async (value: string) => {
            const message = echo.parsePayload<Slot[]>(value);

            if (message.event === EVENT_SLOTS) {
                hand.value = message.data.find((item) => item.kind === HAND_SLOT);
                inventory.value = message.data.filter(
                    (item) => item.kind === INVENTORY_SLOT
                );
                rune.value = message.data.filter((item) => item.kind === RUNE_SLOT);
                consumable.value = message.data.filter(
                    (item) => item.kind === CONSUMABLE_SLOT
                );
                weapon.value = message.data.find((item) => item.kind === WEAPON_SLOT);
                shoulders.value = message.data.find(
                    (item) => item.kind === SHOULDERS_SLOT
                );
                ring.value = message.data.filter((item) => item.kind === RING_SLOT);
                pendant.value = message.data.find((item) => item.kind === PENDANT_SLOT);
                pants.value = message.data.find((item) => item.kind === PANTS_SLOT);
                necklace.value = message.data.find(
                    (item) => item.kind === NECKLACE_SLOT
                );
                mask.value = message.data.find((item) => item.kind === MASK_SLOT);
                helmet.value = message.data.find((item) => item.kind === HELMET_SLOT);
                gloves.value = message.data.find((item) => item.kind === GLOVES_SLOT);
                earring.value = message.data.filter(
                    (item) => item.kind === EARRING_SLOT
                );
                cloak.value = message.data.find((item) => item.kind === CLOAK_SLOT);
                boots.value = message.data.find((item) => item.kind === BOOKS_SLOT);
                belt.value = message.data.find((item) => item.kind === BELT_SLOT);
                armor.value = message.data.find((item) => item.kind === ARMOR_SLOT);
                compass.value = message.data.find((item) => item.kind === COMPASS_SLOT);
                ground.value = message.data.filter((item) => item.kind === GROUND_SLOT);
            }
        }
    );

    return {
        hand,
        inventory,
        rune,
        consumable,
        weapon,
        shoulders,
        ring,
        pendant,
        pants,
        necklace,
        mask,
        helmet,
        gloves,
        earring,
        cloak,
        boots,
        belt,
        armor,
        compass,
        ground,

        filled,
        weight
    };
});

export type Slot = {
    index: number;
    item?: Item;
    kind: SlotKind;
};

export type Item = {
    name: string;
    kind: ItemKind;
    tier: Tier;
    icon: string;
    quantity: number;
    level: number;
    enchanted: number;
    description: string;
    weight: number;
    stats?: ItemStats
};

export type ItemStats = {
    attack?: number,
    attack_speed?: number,
    energy_regeneration?: number,
    energy_regeneration_interval?: number,
    expedition_kind?: string
}