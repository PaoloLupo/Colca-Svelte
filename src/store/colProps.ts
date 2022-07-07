import { writable } from "svelte/store";

let autoNameId = 1;
export const normative = writable("ACI");

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
