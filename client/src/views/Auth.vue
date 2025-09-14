<script setup lang="ts">
import { Window, LogicalSize } from "@tauri-apps/api/window";
import { useRouter } from "vue-router";
import { AuthPayload, useAuthStore } from "../stores/auth";
import { computed, nextTick, onMounted, reactive, ref, watch } from "vue";
import { resetObject } from "../pkg/utils";
import { onKeyStroke, onStartTyping } from "@vueuse/core";
import Input from "../components/Input.vue";
import Button from "../components/Button.vue";

const router = useRouter();

const authStore = useAuthStore();

const isRegister = ref(false);
const loading = ref(false);

const form = reactive<AuthPayload>({
  email: "",
  username: "",
  password: "",
});
const error = ref("");

const formRef = ref<HTMLDivElement | null>(null);

const isValidForm = computed(() => {
  if (isRegister.value) {
    return (
        form.email &&
        form.username &&
        form.password
    );
  }

  return form.username && form.password;
});

function submit() {
  isRegister.value ? register() : login();
}

async function register() {
  if (loading.value || !isValidForm.value) {
    return;
  }

  loading.value = true;

  try {
    await authStore.register(form);

    resetObject(form);

    isRegister.value = false;
  } catch (e) {
    const err = e as Error;
    error.value = err.message;
  } finally {
    loading.value = false;
  }
}

async function login() {
  if (loading.value || !isValidForm.value) {
    return;
  }

  loading.value = true;

  try {
    const res = await authStore.login(form);

    resetObject(form);

    authStore.token = res.token;

    await nextTick();

    await router.push("/");
  } catch (e) {
    const err = e as Error;
    error.value = err.message;
  } finally {
    loading.value = false;
  }
}

watch(isRegister, () => {
  resetObject(form);
});
watch(form, () => (error.value = ""), { deep: true });
onKeyStroke("Enter", submit);
onStartTyping(() => {
  const input = formRef.value?.getElementsByTagName("input")[0];
  input?.focus();
});

onMounted(async () => {
  const win = Window.getCurrent();
  await win.setSize(new LogicalSize(360, 440));
  await win.center();
});
</script>

<template>
  <div
      class="relative flex flex-col flex-1 items-center justify-center h-screen"
  >
    <div ref="formRef" class="w-screen max-w-[280px] flex flex-col gap-4">
      <Input v-if="isRegister" v-model="form.email" placeholder="Email" />
      <Input v-model="form.username" placeholder="Username" />
      <Input v-model="form.password" placeholder="Password" type="password" />
      <Button :disabled="!isValidForm" :loading="loading" @click="submit">
        {{ isRegister ? "Register" : "Login" }}
      </Button>
      <div class="text-sm text-center text-zinc-500">
        <template v-if="isRegister">
          Already have an account?
          <a href="#" class="text-primary" @click.prevent="isRegister = false">
            Sign in</a
          >.
        </template>
        <template v-else>
          Don't have an account?
          <a href="#" class="text-primary" @click.prevent="isRegister = true">
            Sign up</a
          >.
        </template>
      </div>
    </div>
    <div
        v-if="error"
        class="absolute left-0 bottom-4 text-center w-full leading-none text-danger"
    >
      {{ error }}
    </div>
  </div>
</template>
