import { writable } from "svelte/store";
import CONSTANTS from "../constants";

// Variable that will be used in user registration
export const isOverlayOpen = writable(false);
export const isPropsEditable = writable(false);
export const isDimsConfOpen = writable(false);

export const user = writable({
  name: String,
  company: String,
});

const createUser = (name: string, company: string) => {
  localStorage.setitem(CONSTANTS.USER_NAME, name);
};
