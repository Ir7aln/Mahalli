// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-08-31",
  devtools: { enabled: true },
  modules: ["@nuxtjs/tailwindcss", "shadcn-nuxt", "@nuxtjs/i18n"],
  ssr: false,
  devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },
  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
    },
  },
  routeRules: { "/": { prerender: true } },
  components: [{ path: "~/components", extensions: ["vue"] }],
  shadcn: { prefix: "", componentDir: "./app/components/ui" },
  tailwindcss: {
    cssPath: "~/assets/css/tailwind.css",
    configPath: "tailwind.config.ts",
    viewer: false,
  },
  i18n: {
    locales: [
      { code: "en", file: "en-US.json" },
      { code: "fr", file: "fr-FR.json" },
      { code: "ar", file: "ar-AE.json" },
      { code: "de", file: "de-DE.json" },
    ],
    langDir: "lang",
    defaultLocale: "en",
    vueI18n: "./i18n.config.ts",
  },
});
