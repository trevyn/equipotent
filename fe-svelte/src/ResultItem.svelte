<script lang="ts">
 import * as wasm from "../../fe-rs/pkg";
 export let item: wasm.SearchQueryResultItem;
</script>

<div class="border border-gray-800 p-4 rounded-xl mt-4">
 <p class="font-bold text-2xl text-gray-300">{item.title}</p>
 <p class="text-gray-500 font-bold">{item.url}</p>
 <p class="text-gray-400 text-xl">{item.snippet}</p>
 <p class="text-gray-700 font-bold">rank: {item.rank}</p>
</div>
