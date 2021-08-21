<script lang="ts">
 import { onMount, createEventDispatcher } from "svelte";
 import { EditorState, EditorView, basicSetup } from "@codemirror/basic-setup";
 import { oneDark } from "@codemirror/theme-one-dark";

 const dispatch = createEventDispatcher();

 export let doc;

 let node;
 let view: EditorView;

 onMount(async () => {
  view = new EditorView({
   state: EditorState.create({
    doc,
    extensions: [
     basicSetup,
     oneDark,
     EditorView.lineWrapping,
     EditorView.updateListener.of((update) => {
      if (update.docChanged) {
       dispatch("docChanged", view.state.doc.toString());
      }
     }),
    ],
   }),
   parent: node,
  });
 });
</script>

<div class="text-xl min-w-full max-w-full" bind:this={node} />
