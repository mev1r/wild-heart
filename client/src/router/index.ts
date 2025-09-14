import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "../stores/auth";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/",
            name: "home",
            component: () => import("../views/Home.vue"),
        },
        {
            path: "/login",
            name: "login",
            component: () => import("../views/Auth.vue"),
        },
    ],
});

router.beforeEach(async (to, _, next) => {
    const auth = useAuthStore();

    if (auth.token && to.path === "/login") {
        next("/");
    } else if (!auth.token && to.path !== "/login") {
        next("/login");
    } else {
        next();
    }
});

export default router;
