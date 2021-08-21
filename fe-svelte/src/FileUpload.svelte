<script lang="ts">
 import * as wasm from "../../middle-rs/pkg";

 function uploadFile(target) {
  Array.from((target as HTMLInputElement).files).forEach((file) => {
   let reader = new FileReader();
   reader.onload = (e) => {
    console.log(e.target.result);
    if (e.target.result instanceof ArrayBuffer) {
     wasm.process_upload(new Uint8Array(e.target.result));
    }
   };
   reader.readAsArrayBuffer(file);
  });
 }
</script>

<input type="file" on:change={(e) => uploadFile(e.target)} />
