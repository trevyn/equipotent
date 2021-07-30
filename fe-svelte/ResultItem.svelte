<script lang="ts">
 import * as wasm from "../fe-rs/pkg";
 export let item: wasm.ResultItem;
</script>

<div>
 ResultItem: {item.rowid} - {item.url} - {item.title}
</div>
