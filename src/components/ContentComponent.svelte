<script lang="ts">
  export let title;
</script>

<div class="relative m-6 rounded-md bg-neutral p-4">
  <div class="absolute left-1 -top-3 rounded bg-slate-600 p-1 font-medium">
    {title}
  </div>
  <p class="text-gray-700">
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec euismod, nisi
    eu consectetur egestas, nisl nisi consectetur nisi, euismod egestas nisi
    nisi eu consectetur.
  </p>
  <button
    class="btn btn-error btn-circle btn-xs  absolute right-0 -top-3 "
    on:click
  >
    <svg
      class="h-6 w-6"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      ><path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M6 18L18 6M6 6l12 12"
      /></svg
    >
  </button>
</div>
