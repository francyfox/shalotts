// https://nuxt.com/docs/api/configuration/nuxt-config
import "./env";
export default defineNuxtConfig({
	ssr: false,
	compatibilityDate: "2025-07-15",
	devtools: { enabled: true },
	css: ["~/globals.css", "~/styles.pcss"],
	modules: [
		"@vueuse/nuxt",
		"@nuxtjs/tailwindcss",
		"@nuxt/image",
		"@pinia/nuxt",
		[
			"shadcn-nuxt",
			{
				prefix: "Ui",
				componentDir: "@/components/ui",
			},
		],
	],
});
