// TauriのAPI(Rust/TS)のラッパー関数をまとめるファイル
import { open as openWithTauri } from "@tauri-apps/plugin-shell";
import { invoke } from "@tauri-apps/api/core";
import type { Result } from "$lib/generated/specta/bindings";

const mainWindowFocus = async () => {
  await invoke<void>("main_window_focus");
};

const open = async (path: string): Promise<void> => {
  // config ファイルから設定を読む or ユーザーが指定したサービスを使う
  await openWithTauri(path);
};

const err = (err: string) => {
  console.error(err);
};

const unwrap = <T>(v: Result<T, string>): T => {
  switch (v.status) {
    case "ok":
      return v.data;
    case "error":
      throw err(v.error);
  }
};

export { unwrap, open, err, mainWindowFocus };
