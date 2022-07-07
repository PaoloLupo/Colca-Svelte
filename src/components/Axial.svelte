<script lang="ts">
    import { isAxialDesignEnabled } from "../store/axial";
    import ContentComponent from "./ContentComponent.svelte";


</script>

{#if $isAxialDesignEnabled}
    <ContentComponent  title="Axial"  on:click={()=> isAxialDesignEnabled.set(false)}/>
{/if}