<script lang="ts">
  import data from "../store/data";
  import IntDiag from "./IntDiag.svelte";
</script>

<div class="plot">
  <h2>Diagrama</h2>
  <IntDiag points={data.a} />
</div>
