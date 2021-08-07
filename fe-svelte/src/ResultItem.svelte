<script lang="ts">
 import * as wasm from "../../fe-rs/pkg";
 import { encodeHTML } from "entities";

 export let item: wasm.SearchQueryResultItem;

 function highlight(s: string) {
  return encodeHTML(s)
   .replaceAll(
    "EQUIPOTENTHIGHLIGHTOPEN",
    '<span class="bg-yellow-300 text-gray-800">'
   )
   .replaceAll("EQUIPOTENTHIGHLIGHTCLOSE", "</span>");
 }
</script>

<div class="border border-gray-800 p-4 rounded-xl mt-4">
 <p class="font-bold text-2xl text-gray-300">
  {@html highlight(item.title)}
 </p>
 <p class="text-gray-500 font-bold">
  {@html highlight(item.search_highlighted_url)}
 </p>
 <p class="text-gray-400 text-xl">{@html highlight(item.snippet)}</p>
 <p class="text-gray-700 font-bold">rank: {item.rank}</p>
</div>
