<script>
  import { listColumns } from "../store/colProps";
</script>

<footer>
  <div class="flex flex-row bg-neutral">
    <div class="dropdown dropdown-top">
      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label tabindex="0" class="btn  btn-xs rounded-none">
        <svg
          class="h-5 w-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
          ><path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
          /></svg
        >
      </label>
      <ul tabindex="0" class="dropdown-content menu bg-base-100 p-2 shadow">
        <!-- svelte-ignore a11y-missing-attribute -->
        <li><a>Item 1</a></li>
        <!-- svelte-ignore a11y-missing-attribute -->
        <li><a>Item 2</a></li>
      </ul>
    </div>

    <h1>que tal</h1>
  </div>
</footer>
