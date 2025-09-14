<script setup lang="ts">
import { Icon } from "@iconify/vue";
import { ref } from "vue";

type Props = {
  placeholder: string;
  type?: string;
  disabled?: boolean;
  autofocus?: boolean;
  readonly?: boolean;
};

const { type = "text", placeholder, disabled } = defineProps<Props>();
const modelValue = defineModel();
const emit = defineEmits(["focus", "blur"]);

const isFocus = ref(false);

function onFocus() {
  isFocus.value = true;
  emit("focus");
}

function onBlur() {
  isFocus.value = false;
  emit("blur");
}

function validateIntegerInput(e: any) {
  if (type === "integer") {
    modelValue.value = e.target.value.replace(/[^0-9]/g, "");
  }
}
</script>

<template>
  <div
      class="relative"
      :class="{
      'pointer-events-none': disabled,
    }"
  >
    <input
        v-model="modelValue"
        :type="type"
        :disabled="disabled"
        autocomplete="off"
        autocorrect="off"
        autocapitalize="off"
        spellcheck="false"
        :autofocus="autofocus"
        :placeholder="placeholder"
        :readonly="readonly"
        class="main-border-inverted bg-input-bg pr-8 py-2 pl-4 w-full transition-colors"
        :class="{
        'pointer-events-none opacity-70': disabled,
      }"
        @focus="onFocus"
        @blur="onBlur"
        @input="validateIntegerInput"
    />
    <Icon
        v-if="modelValue"
        icon="mdi:close"
        :width="18"
        class="absolute right-3 top-1/2 -mt-[9px] hover:text-primary cursor-pointer transition-colors"
        :class="{ 'text-zinc-500': !isFocus }"
        @click="modelValue = ''"
    />
  </div>
</template>
