import { writable } from "svelte/store";
import CONSTANTS from "../constants";

export const isOverlayOpen = writable(false);
export const user = writable({
  name: String,
  company: String,
});

const createUser = (name: string, company: string) => {
  localStorage.setitem(CONSTANTS.USER_NAME, name);
};
