<script lang="ts">
  $: innerHeight = 0;
  $: innerWidth = 0;
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<div class="plot">
  <p>innerWidth : {innerWidth}</p>
  <p>innerHeight: {innerHeight}</p>
</div>
