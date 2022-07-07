<script>
  import { isPropsEditable } from "../store/user";
  import { createColumn, listColumns } from "../store/colProps";
  import { deleteColumn } from "../store/colProps";
  import { invoke } from "@tauri-apps/api/tauri"

  const invoke_tauri_example = () => {
    invoke("new_init_column", {
      name: "example",
      analysisType: ["example"],
      normative: "ACI",
      deadLoad: 240.0,
      liveLoad: 240.0,
      dimensions: [10,10],
      concreteMaterial: "21 MPa",
      steelMaterial: "Grado 60",
      selectedRefSteel: ["6mm", "8mm"],
      percentage: 0.02,
      stirrups: "rectangulares"

    }).then(response => {
      console.log(response);
    });
  }
</script>

<div>
  {#if $isPropsEditable}
    <div class="content fixed items-start justify-start ">
      <ul class="w-90 menu  bg-neutral p-4 text-base-content">
        <!-- Sidebar content here -->
        <p class="text-lg font-bold">Dimensiones</p>
        <button class="btn btn-success btn-square" on:click={invoke_tauri_example}>
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
        </button>

        {#each $listColumns as column}
          <div class="flex flex-row items-center  space-x-2 space-y-4">
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
            <div class="items-center ">
              <!-- svelte-ignore a11y-label-has-associated-control -->

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
            <button
              class="btn btn-error btn-square"
              on:click={deleteColumn(column)}
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
                  d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
                /></svg
              >
            </button>
          </div>
        {/each}
      </ul>
    </div>
  {/if}
</div>
