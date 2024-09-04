
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async exit() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("exit") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getWindows() : Promise<WindowData[]> {
    return await TAURI_INVOKE("get_windows");
},
async viewCreate(url: string, label: string | null) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_create", { url, label }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async windowFocus() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("window_focus") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async windowHide() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("window_hide") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getIgnoreCursorEvents() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_ignore_cursor_events") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getPin() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_pin") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getTransparent() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_transparent") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setIgnoreCursorEvents(value: boolean) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_ignore_cursor_events", { value }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setPin(value: boolean) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_pin", { value }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setTransparent(alpha: number) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_transparent", { alpha }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async toggleIgnoreCursorEvents() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("toggle_ignore_cursor_events") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async togglePin() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("toggle_pin") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async toggleTransparent(alpha: number) : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("toggle_transparent", { alpha }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async viewClose(label: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_close", { label }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async viewDrag() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_drag") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async viewMinimize() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_minimize") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async viewZoomin() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_zoomin") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async viewZoomout() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("view_zoomout") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/



/** user-defined constants **/

export const CTRL_LABEL_PREFIX = "ctrl_" as const;
export const WINDOW_LABEL_PREFIX = "window_" as const;

/** user-defined types **/

export type WindowData = { title: string; label: string; ignore: boolean; pin: boolean; zoom: number }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: T extends null
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
