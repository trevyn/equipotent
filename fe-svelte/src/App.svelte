<script lang="ts">
 import initwasm from "../../common-rs/pkg";
 import AppContents from "./AppContents.svelte";
</script>

{#await initwasm() then _}
 <AppContents />
{:catch error}
 <p style="color: red">Error: {error.message}</p>
{/await}
