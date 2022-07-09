import { writable } from "svelte/store";

let autoNameId = 1;

export const refSteelKind = [
  "6mm",
  "8mm",
  "3/8",
  "12mm",
  "1/2",
  "5/8",
  "3/4",
  "1",
  "1-3/8",
];

// Variables for exporting to rust
export const analysisType = writable([]);
export const normative = writable("ACI");
export const deadLoad = writable(0);
export const liveLoad = writable(0);
export const dimensions = writable([]);
export const concreteMaterial = writable("21 MPa");
export const steelMaterial = writable("Grado 60");
export const selectedRefSteel = writable([]);
export const percentage = writable(0.02);
export const stirrups = writable("rectangulares");

export let listColumns = writable([
  { selected: true, name: "Col1", height: 30, width: 30 },
]);

export const createColumn = () => {
  autoNameId++;
  const newColumn = {
    selected: false,
    name: "Col" + autoNameId + "",
    height: 0,
    width: 0,
  };
  listColumns.update((columns) => [...columns, newColumn]);
};

export const deleteColumn = (column) => {
  listColumns.update((columns) =>
    columns.filter((col) => col.name !== column.name)
  );
};
