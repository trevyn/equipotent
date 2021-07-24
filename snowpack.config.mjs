/** @type {import("snowpack").SnowpackUserConfig } */
export default {
  mount: {
    /* ... */
  },
  plugins: [
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-svelte',
    '@snowpack/plugin-postcss',
    [
      'snowpack-plugin-wasm-pack',
      {
        projectPath: './mywasm',
      },
    ],
  ],
  routes: [
    /* Enable an SPA Fallback in development: */
    // {"match": "routes", "src": ".*", "dest": "/index.html"},
  ],
  optimize: {
    /* Example: Bundle your final build: */
    // "bundle": true,
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    tailwindConfig: './tailwind.config.js',
  },
  buildOptions: {
    /* ... */
  },
};
