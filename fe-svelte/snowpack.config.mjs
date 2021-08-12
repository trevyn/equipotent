/** @type {import("snowpack").SnowpackUserConfig } */
export default {
 mount: {
  "./src": {
   url: "/",
  },
 },
 plugins: [
  "@snowpack/plugin-typescript",
  "@snowpack/plugin-svelte",
  "@snowpack/plugin-postcss",
  [
   "snowpack-plugin-wasm-pack",
   {
    projectPath: "../common-rs",
    // profile: "release",
   },
  ],
 ],
 routes: [
  /* Enable an SPA Fallback in development: */
  // {"match": "routes", "src": ".*", "dest": "/index.html"},
 ],
 optimize: {
  bundle: true,
  // minify: true,
  target: "es2020",
 },
 exclude: ["**/*.json", "**/*.md"],
 packageOptions: {
  /* ... */
 },
 devOptions: {
  tailwindConfig: "./tailwind.config.js",
  port: 8081,
 },
 buildOptions: {
  /* ... */
 },
};
