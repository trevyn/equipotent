<script lang="ts">
 import * as wasm from "../fe-rs/pkg";
 import ResultItem from "./ResultItem.svelte";

 let items = [wasm.return_named_struct("one"), wasm.return_named_struct("two")];

 let query = "banana";

 $: wasm.set_search(query);
</script>

<div
 class="min-h-screen bg-gray-100 py-6 flex flex-col justify-center sm:py-12"
>
 <div class="relative py-3 sm:max-w-xl sm:mx-auto">
  <div
   class="absolute -inset-10 bg-gradient-to-r from-pink-200 to-sky-200 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl"
  />
  <div class="relative px-4 py-10 bg-white shadow-lg sm:rounded-3xl sm:p-20">
   <div class="max-w-md mx-auto">
    <div class="divide-y divide-gray-200">
     <div
      class="py-8 text-base leading-6 space-y-4 text-gray-700 sm:text-lg sm:leading-7"
     >
      <input bind:value={query} />
      <p>
       query is: {query}
       {#each items as item (item)}
        <!-- (item.rowid) -->
        <ResultItem {item} />
       {/each}
      </p>
     </div>
    </div>
   </div>
  </div>
 </div>
</div>
