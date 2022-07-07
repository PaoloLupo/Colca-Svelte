<script>
  import { isPropsEditable, isDimsConfOpen } from "../store/user";
  import { createColumn, listColumns } from "../store/colProps";
  import { deleteColumn } from "../store/colProps";
  import { invoke } from "@tauri-apps/api/tauri"
  import DimsIcon from "../assets/icons/DimsIcon.svelte";
  import SteelIcon from "../assets/icons/SteelIcon.svelte";

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

  const columns_descriptions_dims = ["Nombre", "Altura (cm)", "Ancho (cm)"]
  const columns_dims = []
</script>

<div class="plot" >
  {#if $isPropsEditable}

    <div class="p-2 ">
      <ul class=" bg-neutral p-2 rounded ">
        <div class="flex flex-row justify-center space-x-4 ">
          <button class="btn btn-square disabled:bg-info" on:click={()=> isDimsConfOpen.set(true)} disabled={$isDimsConfOpen}>
            <DimsIcon />
          </button>
          <button class="btn btn-square gap-2">
            <SteelIcon />
          </button>
        </div>


        {#if $isDimsConfOpen}
        <!-- Sidebar content here -->
        <button class="btn btn-success btn-square" on:click={createColumn}>
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
        <table>
          <tr>
            {#each columns_descriptions_dims as column_description}
              <th class="text-sm text-gray-600">{column_description}</th>
            {/each}
          </tr>
          {#each $listColumns as column}
            <tr>
              {#each column as cell}
                <td class="text-sm text-gray-600">{cell}</td>
              {/each}
              <button disabled> jal</button>
            </tr>
          {/each}

        </table>



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
              class="input input-bordered w-20"
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
      {/if}
      </ul>
    </div>
  {/if}
</div>
