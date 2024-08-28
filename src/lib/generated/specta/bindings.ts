/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

export function exportTypes(a: WindowData) {
    return invoke()<null>("export_types", { a })
}

export function exit() {
    return invoke()<null>("exit")
}

export function windowFocus() {
    return invoke()<null>("window_focus")
}

export function windowHide() {
    return invoke()<null>("window_hide")
}

export function openWindow(url: string, title: string | null, label: string | null) {
    return invoke()<null>("open_window", { url,title,label })
}

export function closeWindow(label: string) {
    return invoke()<null>("close_window", { label })
}

export function getTransparent() {
    return invoke()<boolean>("get_transparent")
}

export function togglePin() {
    return invoke()<boolean>("toggle_pin")
}

export type WindowData = { title: string; label: string; pin: boolean; zoom: number }
