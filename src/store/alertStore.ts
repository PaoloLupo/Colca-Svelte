import { writable } from "svelte/store";

export const ALERT_TYPES = {
  DANGER: String,
  INFO: String,
  SUCCESS: String,
};

Object.freeze(ALERT_TYPES);

export const alertMessage = writable("");
export const alertType = writable(ALERT_TYPES.INFO);

export const displayAlert = (
  message: string,
  type = ALERT_TYPES.INFO,
  resetTime: number
) => {
  alertMessage.set(message);
  alertType.set(type);
  if (resetTime) {
    setTimeout(() => {
      alertMessage.set("");
    }, resetTime);
  }
};
