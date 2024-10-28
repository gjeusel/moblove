import { invoke } from "@tauri-apps/api/core"
import type { PageLoad } from "./$types"

export const load: PageLoad = async (event) => {
  const url = event.url.searchParams.get("url")
  const images: string[] = await invoke("get_manga_chapter_images", { url })

  return {
    images,
  }
}
