import type { SAppState } from "$lib/generated/specta/bindings";
import { writable } from "svelte/store";

const state = writable<SAppState>();

export { state };
