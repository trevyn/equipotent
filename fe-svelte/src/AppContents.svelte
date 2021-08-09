<script context="module">
 let set_items;
 export function set_json(json) {
  set_items(json);
 }
</script>

<script lang="ts">
 import * as wasm from "../../fe-rs/pkg";
 import ResultItem from "./ResultItem.svelte";
 import CodeMirror from "./CodeMirror.svelte";

 let items = [];
 let query = "";

 set_items = (json) => {
  items = JSON.parse(json);
 };

 // $: if (query.slice(-1) == " ") wasm.set_search(query.slice(0, -1));
</script>

<div class="min-h-screen bg-gray-900 py-6 flex flex-col justify-center">
 <div class="relative p-5 sm:mx-auto w-full">
  <div
   class="absolute -inset-10 bg-gradient-to-r from-pink-900 to-sky-700 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl"
  />
  <div class="relative px-4 py-10 bg-gray-900 shadow-lg sm:rounded-3xl sm:p-10">
   <div class="mx-auto">
    <div class="divide-y divide-gray-200">
     <div
      class="text-base leading-6 space-y-4 text-gray-700 sm:text-lg sm:leading-7"
     >
      <CodeMirror on:docChanged={(e) => console.log(e.detail)} />
      <hr />
      <CodeMirror />
      <!-- svelte-ignore a11y-autofocus -->
      <!-- <input
       class="p-5 text-5xl w-full"
       autofocus
       bind:value={query}
       on:keyup={(e) => {
        if (e.keyCode == 13) {
         wasm.send_command(wasm.Command.new(wasm.CommandType.OpenAi, query));
        } else {
         wasm.send_command(
          wasm.Command.new(wasm.CommandType.SearchInstant, query)
         );
        }
       }}
      /> -->
      <!-- <p> -->
      {#each items as item (item)}
       <!-- (item.rowid) -->
       <ResultItem {item} />
      {/each}
      <!-- </p> -->
     </div>
    </div>
   </div>
  </div>
 </div>
</div>
