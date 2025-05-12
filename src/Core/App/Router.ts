import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Home from "./Pages/Home.vue";

const routes: RouteRecordRaw[] = [
	{
		path: "/",
		name: "Home",
		component: Home
	}
];

export const router = createRouter({
	history: createWebHashHistory(),
	routes
});
