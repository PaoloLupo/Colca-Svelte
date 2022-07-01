<script lang="ts">
  import { isPropsEditable } from "../store/user";
  import { createColumn, listColumns } from "../store/colProps";
</script>

<div>
  {#if $isPropsEditable}
    <div class="content fixed items-start justify-start ">
      <ul class="w-90 menu max-h-screen  bg-neutral p-4 text-base-content">
        <button class="btn btn-square" on:click={createColumn}>
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
              d="M12 6v6m0 0v6m0-6h6m-6 0H6"
            />
          </svg>
          Add
        </button>
        <!-- Sidebar content here -->
        <p class="text-lg font-bold">Dimensiones</p>
        {#each $listColumns as column}
          <div class="flex flex-row items-center space-x-2 space-y-4">
            <input
              type="checkbox"
              bind:checked={column.selected}
              class="checkbox"
            />
            <input
              type="text"
              bind:value={column.name}
              class="input input-bordered max-w-xs"
            />
            <div class="form-control ">
              <!-- svelte-ignore a11y-label-has-associated-control -->
              <label class="label">
                <span class="label-text"> Altura</span>
              </label>
              <label class="input-group">
                <input
                  type="number"
                  bind:value={column.height}
                  class="input input-bordered w-full max-w-xs"
                  min="0"
                  max="100"
                  step="5"
                />
                <span>cm</span>
              </label>
            </div>
            <div class="form-control ">
              <!-- svelte-ignore a11y-label-has-associated-control -->
              <label class="label">
                <span class="label-text"> Ancho</span>
              </label>
              <label class="input-group">
                <input
                  type="number"
                  bind:value={column.width}
                  class="input input-bordered w-full max-w-xs"
                  min="0"
                  max="100"
                  step="5"
                />
                <span>cm</span>
              </label>
            </div>
          </div>
        {/each}
      </ul>
    </div>
  {/if}
</div>
