<script>
  import { listColumns, normative } from "../store/colProps";
</script>

<footer>
  <div class="flex flex-row items-center bg-neutral">
    <!-- svelte-ignore a11y-label-has-associated-control -->
    <label tabindex="0" class="btn  btn-xs rounded-none bg-slate-600">
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
      Paolo Cesar
    </label>
    <div class="dropdown-top dropdown">
      <label tabindex="0" class="btn btn-xs rounded-none bg-red-800"
        >{$normative}</label
      >
      <ul
        tabindex="0"
        class="dropdown-content menu rounded-box w-52 bg-base-100 p-2 shadow"
      >
        <li><a>ACI</a></li>
        <li><a>NTP</a></li>
      </ul>
    </div>
  </div>
</footer>
