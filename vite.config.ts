import vue from "@vitejs/plugin-vue";
import { resolve } from "path";
import RekaResolver from "reka-ui/resolver";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [
		vue(),
		Components({
			dts: "src/Types/Components.d.ts",
			resolvers: [RekaResolver()],
			dirs: ["src/Core/App/Components", "src/Features/**/Components"],
			deep: true
		}),
		AutoImport({
			dts: "src/Types/AutoImport.d.ts",
			imports: ["vue"],
			dirs: [],
			vueTemplate: true,
			include: [/\.vue$/, /\.vue\?vue/, /\.vue\.[tj]sx?\?vue/, /\.ts$/]
		})
	],
	resolve: {
		alias: {
			"@": resolve(__dirname, "./src"),
			"@Core": resolve(__dirname, "./src/Core"),
			"@Features": resolve(__dirname, "./src/Features")
		}
	},
	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: "ws",
					host,
					port: 1421
			  }
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ["**/src-tauri/**"]
		}
	}
}));
