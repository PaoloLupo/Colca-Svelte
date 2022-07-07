<script lang="ts">
    export let title;

</script>

<div class="p-4 rounded-md bg-neutral relative m-6">
    <div class="absolute left-1 -top-3 font-medium p-1 rounded bg-slate-600">{title}</div>
    <p class="text-gray-700">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Donec euismod, nisi eu consectetur egestas, nisl nisi consectetur nisi,
        euismod egestas nisi nisi eu consectetur.
    </p>
    <button class="absolute right-0 -top-3 btn  btn-circle btn-error btn-xs " on:click>
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
    </button>

</div>