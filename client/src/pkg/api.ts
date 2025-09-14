import { useAuthStore } from "../stores/auth";

const baseURL = import.meta.env.VITE_APP_REST_URL;

async function api<T = any>(
    url: string,
    method = "GET",
    body?: any
): Promise<T> {
    const authStore = useAuthStore();

    const res = await fetch(`${baseURL}${url}`, {
        method,
        headers: {
            "Content-Type": "application/json",
            Authorization: authStore.token ? `Bearer ${authStore.token}` : "",
        },
        body: body ? JSON.stringify(body) : undefined,
    });

    if (!res.ok) {
        const error = await res.json().catch(() => ({}));
        throw new Error(error.message || res.statusText);
    }

    const contentType = res.headers.get("content-type");
    if (contentType && contentType.includes("application/json")) {
        return res.json();
    }

    return res.text() as Promise<T>;
}

export const post = <T>(url: string, body: any) => api<T>(url, "POST", body);
