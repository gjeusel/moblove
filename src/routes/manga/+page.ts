import type { PageLoad } from "./$types"

import { fetch } from "@tauri-apps/plugin-http"

interface MangaChapter {
  url: string
  num: number
  timestamp: string
}

interface Manga {
  name: string
  source: string
  chapters: MangaChapter[]
}

export const load: PageLoad = async () => {
  const response = await fetch("https://messy.s3.fr-par.scw.cloud/neatpush.json")
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`)
  }
  const mangas = (await response.json()) as Manga[]

  return { mangas }
}
