import { writable } from "svelte/store";

export let listColumns = writable([
  { selected: true, name: "Col1", height: 30, width: 30 },
]);

export const createColumn = () => {
  const newColumn = {
    selected: false,
    name: "",
    height: 0,
    width: 0,
  };
  listColumns.update((columns) => [...columns, newColumn]);
};
