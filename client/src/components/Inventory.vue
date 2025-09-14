<script setup lang="ts">
import Section from "./Section.vue";
import Slot from "./Slot.vue";
import { useSlotsStore } from "../stores/slots";
import Item from "./Item.vue";

const slots = useSlotsStore();
</script>

<template>
  <Section class="relative flex flex-col flex-1 overflow-hidden">
    <div class="relative z-10 flex flex-col flex-1 p-2">
      <div class="flex flex-1 flex-wrap gap-2 content-start scroller">
        <Slot v-for="slot in slots.inventory" :key="slot.index" :slot="slot">
          <Item v-if="slot.item" :item="slot.item" />
        </Slot>
      </div>
    </div>
  </Section>
</template>
