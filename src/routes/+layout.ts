// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import type { LayoutLoad } from "./$types";
import { invoke } from "@tauri-apps/api/core";

export const load: LayoutLoad = async () => {
  return {
    api_url: await invoke("api_url"),
  };
};
