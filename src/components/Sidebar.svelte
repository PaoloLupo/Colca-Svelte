<script>
    import AxialIcon from "../assets/sidebar/AxialIcon.svelte";
    import { isAxialDesignEnabled} from "../store/axial";

    const axialDesignEnabled = () => {
        isAxialDesignEnabled.set(!$isAxialDesignEnabled);
    }

</script>
<div class="sidebar bg-neutral flex flex-col p-2 space-y-4">
    <button class="btn btn-square gap-2 disabled:bg-info " on:click={axialDesignEnabled} disabled={$isAxialDesignEnabled}>
        <AxialIcon/>
    </button>
    <button class="btn btn-square gap-2">
    </button>
</div>