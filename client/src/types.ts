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
} from "./pkg/slots";

export type Tier = "Common" | "Uncommon" | "Rare" | "Epic" | "Legendary";

export type Variant = "primary" | "danger" | "success" | "bordered";

export type SlotKind =
    | typeof HAND_SLOT
    | typeof INVENTORY_SLOT
    | typeof WEAPON_SLOT
    | typeof SHOULDERS_SLOT
    | typeof RING_SLOT
    | typeof PENDANT_SLOT
    | typeof PANTS_SLOT
    | typeof NECKLACE_SLOT
    | typeof MASK_SLOT
    | typeof HELMET_SLOT
    | typeof GLOVES_SLOT
    | typeof EARRING_SLOT
    | typeof CLOAK_SLOT
    | typeof BOOKS_SLOT
    | typeof BELT_SLOT
    | typeof RUNE_SLOT
    | typeof CONSUMABLE_SLOT
    | typeof COMPASS_SLOT
    | typeof GROUND_SLOT
    | typeof ARMOR_SLOT;

export type ItemKind = "Weapon" | "Consumable";
export type ItemSubKind = "hunter_compass" | "one_handed_sword";

export type MobTier = "Common" | "Magic" | "Rare" | "Epic";
