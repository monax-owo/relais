import type { SerDeAppState } from "$lib/generated/specta/bindings";
import { writable } from "svelte/store";

const appState = writable<SerDeAppState | null>(null);

export { appState };
