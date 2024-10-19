import type { SerDeAppState } from "$lib/generated/specta/bindings";
import { writable } from "svelte/store";

const state = writable<SerDeAppState | null>(null);

export { state };
