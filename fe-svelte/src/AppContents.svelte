<script context="module">
 import * as wasm from "../../middle-rs/pkg";
</script>

<script lang="ts">
 import CodeMirror from "./CodeMirror.svelte";
 import Epub from "./Epub.svelte";
 import FileUpload from "./FileUpload.svelte";
</script>

<div class="bg-black text-gray-500 flex w-screen min-h-screen max-h-screen">
 <div class="overflow-scroll max-h-screen w-2/5">
  <FileUpload />
  {#each [...Array(20).keys()] as key}
   <div class="text-5xl">card {key}</div>
   {#await wasm.Card.get(BigInt(key)) then card}
    <CodeMirror
     doc={card.question}
     on:docChanged={(e) => wasm.Card.set_question(BigInt(key), e.detail)}
    />
    <CodeMirror
     doc={card.answer}
     on:docChanged={(e) => wasm.Card.set_answer(BigInt(key), e.detail)}
    />
   {/await}
   <hr />
  {/each}
 </div>
 <div class="w-3/5"><Epub /></div>
</div>
