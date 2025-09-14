<script lang="ts" setup>
import {ref} from "vue";
import {Tier} from "../types";

type Props = {
  tier?: Tier;
};

const {tier = "common"} = defineProps<Props>();

const rootEl = ref<HTMLElement | null>(null);

defineExpose({
  el: rootEl,
});
</script>

<template>
  <div
      ref="rootEl"
      :class="{
      'main-bg': tier === 'Common',
      'main-bg-uncommon ': tier === 'Uncommon',
      'main-bg-rare': tier === 'Rare',
      'main-bg-epic': tier === 'Epic',
      'main-bg-legendary': tier === 'Legendary',
    }"
      class="shadow main-border backdrop-blur-sm"
  >
    <slot/>
  </div>
</template>
