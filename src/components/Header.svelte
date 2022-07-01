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
  <div class="navbar space-x-2  bg-neutral">
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
    <div class="navbar-center">
      <ul class="menu menu-horizontal p-0">
        <li><a>Item 1</a></li>
        <li tabindex="0">
          <a>
            Parent
            <svg
              class="fill-current"
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              ><path
                d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"
              /></svg
            >
          </a>
          <ul class="bg-base-100 p-2">
            <li><a>Submenu 1</a></li>
            <li><a>Submenu 2</a></li>
          </ul>
        </li>
        <li><a>Item 3</a></li>
      </ul>
    </div>
  </div>
</header>
