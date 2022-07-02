<script lang="ts">
  import { isOverlayOpen } from "../store/user";
  import { isPropsEditable } from "../store/user";

  const editProps = () => {
    isPropsEditable.set(!$isPropsEditable);
  };

  const openOverlay = () => {
    isOverlayOpen.set(true);
  };
</script>

<header>
  <div class="navbar space-x-2 ">
    <div class="navbar-start">
      <button class="btn btn-square btn-sm" on:click={editProps}>
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
        class="text-md input input-sm  font-bold"
        disabled={!$isPropsEditable}
      />
    </div>
    <!--     TODO centrar los iconos en el medio de manera responsiva-->
    <div class="navbar-center hidden space-x-4 lg:flex">
      <button class="btn btn-square"> Axial </button>
      <button class="btn btn-square"> Biaxial </button>
      <button class="btn btn-square"> Biaxial </button>
      <button class="btn btn-square"> Biaxial </button>
      <button class="btn btn-square"> Biaxial </button>
    </div>
  </div>
</header>
