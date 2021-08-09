<script lang="ts">
 import { EditorState, EditorView, basicSetup } from "@codemirror/basic-setup";
 import { oneDark } from "@codemirror/theme-one-dark";

 let node;
 let view;

 $: view = new EditorView({
  state: EditorState.create({
   extensions: [basicSetup, oneDark, EditorView.lineWrapping],
  }),
  parent: node,
 });
</script>

<div class="text-5xl min-w-full max-w-full" bind:this={node} />
