<script>
 import init, { add } from "mywasm";

 const maths = async () => {
  await init();
  // throw new Error("foo");
  return add(10, 2);
 };

 let promise = maths();
</script>

<div class="App">
 <header class="App-header">
  {#await promise}
   <p>...waiting</p>
  {:then number}
   <p>The number is cool {number}</p>
  {:catch error}
   <p style="color: red">Error: {error.message}</p>
  {/await}
  <a
   class="App-link"
   href="https://svelte.dev"
   target="_blank"
   rel="noopener noreferrer"
  >
   Learn Svelte
  </a>
 </header>
</div>

<style>
</style>
