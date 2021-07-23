<!-- App.svelte -->
<script>
  /* component logic will go here */
  import init, { add } from 'mywasm';

const maths = async () => {
  await init();

  console.log('Addition in rust:', add(1, 2));
};

maths();
</script>
<style>
  /* css will go here */
</style>
<div class="App">
  <header class="App-header">
    <a class="App-link" href="https://svelte.dev" target="_blank" rel="noopener noreferrer">
      Learn Svelte
    </a>
  </header>
</div>