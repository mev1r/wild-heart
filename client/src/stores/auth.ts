import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";
import { post } from "../pkg/api";

export const useAuthStore = defineStore("auth", () => {
    const token = useLocalStorage("token", "");

    async function register(payload: AuthPayload) {
        return await post("/auth/register", payload);
    }

    async function login(payload: AuthPayload) {
        return await post<Record<string, string>>("/auth/login", payload);
    }

    return {
        token,

        register,
        login,
    };
});

export type AuthPayload = {
    email: string;
    username: string;
    password: string;
};
