<script lang="ts">
  import { isOverlayOpen } from "../store/user";
  const openOverlay = () => {
    isOverlayOpen.set(true);
  };
</script>

<header>
  <div class="navbar space-x-2 ">
    <button class="btn btn-square btn-sm">
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
          d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
        /></svg
      >
    </button>
    <input
      type="text"
      placeholder="Default"
      class="text-md input w-full max-w-xs font-bold"
    />
  </div>
</header>
