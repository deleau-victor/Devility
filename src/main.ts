import { createHead } from "@unhead/vue/client";
import "reflect-metadata";
import { container } from "tsyringe";
import { createApp } from "vue";
import App from "./Core/App/App.vue";
import "./Ressources/Styles/Fonts.css";
import "./Ressources/Styles/Main.css";
import "./Ressources/Styles/Variables.css";

import { RegisterPortSlice } from "@/Features/PortForwarding/DependencyInjection";
import { RegisterThemeSlice } from "@/Features/Theme/DependencyInjection";
import { router } from "./Core/App/Router";

// Register the Theme slice
RegisterThemeSlice(container);
RegisterPortSlice(container);

const app = createApp(App);
const head = createHead();
app.use(head);
app.use(router);
app.mount("#app");
