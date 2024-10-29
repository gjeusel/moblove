import { invoke } from "@tauri-apps/api/core"
import type { PageLoad } from "./$types"

export const load: PageLoad = async (event) => {
  const url = event.url.searchParams.get("url")

  return {
    images: invoke("get_manga_chapter_images", { url }) as Promise<string[]>,
  }
}
