<script lang="ts">
  import { isOverlayOpen } from "../store/user";
  const closeOverlay = () => {
    isOverlayOpen.set(false);
  };
</script>

<div>
  {#if $isOverlayOpen}
    <div
      class="w-screen h-screen fixed top-0 left-0 flex justify-center items-center bg-gray-100 opacity-90"
    >
      <div
        class="font-bold bg-base-100 rounded-md px-8 py-10 relative max-w-fit "
      >
        <h1>Crear usuario</h1>
        <button
          class="btn btn-square btn-outline absolute top-2 right-3 btn-sm"
          on:click={closeOverlay}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            ><path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            /></svg
          >
        </button>
        <div class="form-control w-full max-w-xs">
          <label class="label">
            <span class="label-text">Nombre</span>
          </label>
          <input
            type="text"
            placeholder="Nombre"
            class="input input-bordered w-full max-w-xs"
          />
          <label class="label">
            <span class="label-text">Company</span>
          </label>
          <input
            type="text"
            placeholder="Nombre"
            class="input input-bordered w-full max-w-xs"
          />
        </div>
      </div>
    </div>
  {/if}
</div>
