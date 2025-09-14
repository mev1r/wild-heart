<script setup lang="ts">
import { useSlotsStore } from "../stores/slots";
import Item from "./Item.vue";
import Section from "./Section.vue";
import Slot from "./Slot.vue";

const slots = useSlotsStore();
</script>

<template>
  <Section class="relative flex gap-3 p-2">
    <div class="relative z-10">
      <Slot
          v-if="slots.weapon"
          icon="game-icons:croc-sword"
          :slot="slots.weapon"
          :icon-size="84"
          :size="108"
      >
        <Item
            v-if="slots.weapon?.item"
            :item="slots.weapon.item"
            :icon-size="84"
        />
      </Slot>
    </div>
    <div class="grid grid-cols-3 gap-2 relative z-10">
      <Slot
          v-if="slots.shoulders"
          icon="game-icons:spiked-shoulder-armor"
          :slot="slots.shoulders"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.shoulders?.item"
            :item="slots.shoulders.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.helmet"
          icon="game-icons:elf-helmet"
          :slot="slots.helmet"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.helmet?.item"
            :item="slots.helmet.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.mask"
          icon="game-icons:carnival-mask"
          :slot="slots.mask"
          :icon-size="36"
          :size="44"
      >
        <Item v-if="slots.mask?.item" :item="slots.mask.item" :icon-size="36" />
      </Slot>
      <Slot
          v-if="slots.cloak"
          icon="game-icons:cloak"
          :slot="slots.cloak"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.cloak?.item"
            :item="slots.cloak.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.armor"
          icon="game-icons:armor-vest"
          :slot="slots.armor"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.armor?.item"
            :item="slots.armor.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.belt"
          icon="game-icons:belt"
          :slot="slots.belt"
          :icon-size="36"
          :size="44"
      >
        <Item v-if="slots.belt?.item" :item="slots.belt.item" :icon-size="36" />
      </Slot>
      <Slot
          v-if="slots.gloves"
          icon="game-icons:gloves"
          :slot="slots.gloves"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.gloves?.item"
            :item="slots.gloves.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.pants"
          icon="game-icons:armored-pants"
          :slot="slots.pants"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.pants?.item"
            :item="slots.pants.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.boots"
          icon="game-icons:steeltoe-boots"
          :slot="slots.boots"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.boots?.item"
            :item="slots.boots.item"
            :icon-size="36"
        />
      </Slot>
    </div>
    <div class="grid grid-cols-2 gap-2 relative z-10">
      <Slot
          v-for="slot in slots.ring"
          :key="slot.index"
          icon="game-icons:power-ring"
          :slot="slot"
          :icon-size="36"
          :size="44"
      >
        <Item v-if="slot?.item" :item="slot.item" :icon-size="36" />
      </Slot>
      <Slot
          v-for="slot in slots.earring"
          :key="slot.index"
          icon="game-icons:crystal-earrings"
          :slot="slot"
          :icon-size="36"
          :size="44"
      >
        <Item v-if="slot?.item" :item="slot.item" :icon-size="36" />
      </Slot>
      <Slot
          v-if="slots.pendant"
          icon="game-icons:rune-stone"
          :slot="slots.pendant"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.pendant?.item"
            :item="slots.pendant.item"
            :icon-size="36"
        />
      </Slot>
      <Slot
          v-if="slots.necklace"
          icon="game-icons:gem-pendant"
          :slot="slots.necklace"
          :icon-size="36"
          :size="44"
      >
        <Item
            v-if="slots.necklace?.item"
            :item="slots.necklace.item"
            :icon-size="36"
        />
      </Slot>
    </div>
  </Section>
</template>
