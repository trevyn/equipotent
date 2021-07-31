<script lang="ts">
 import * as wasm from "../../fe-rs/pkg";
 export let item: wasm.ResultItem;
</script>

<div class="border border-gray-200 p-4 rounded-xl mt-4">
 <p class="font-bold text-2xl text-gray-500">{item.title}</p>
 <p class="text-gray-300 font-bold">{item.url}</p>
 <p>{item.snippet}</p>
 <p class="text-gray-300 font-bold">turbosql rowid: {item.rowid}</p>
</div>
