<script lang="ts">
  import { fetch } from "@tauri-apps/plugin-http"
  // import { invoke } from "@tauri-apps/api/core"
  import { onMount } from "svelte"

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

  let mangas = $state<Manga[]>([])
  let loading = $state<boolean>(true)

  onMount(async () => {
    try {
      // mangas = await invoke("fetch_neatpush")
      const response = await fetch("https://messy.s3.fr-par.scw.cloud/neatpush.json", {
        mode: "cors",
      })
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      mangas = (await response.json()).reverse()
    } catch (e) {
      console.error(e)
    } finally {
      loading = false
    }
  })
</script>

<div class="flex flex-col items-center">
  <img
    src="/icon.png"
    class="mt-2 size-20 self-center rounded-full p-2 transition-all duration-700 hover:drop-shadow-[0_0_2em_#24c8db] hover:filter"
    style="will-change: filter"
    alt="MobLove"
  />
  <h1 class="my-2 text-center text-lg font-normal">Mangas Chapters</h1>

  {#if loading}
    <div>Loading...</div>
  {:else}
    <div class="flex flex-col gap-y-2">
      {#each mangas as manga}
        {#each manga.chapters.slice(manga.chapters.length - 2, manga.chapters.length) as chapt}
          <a href={chapt.url} target="_blank" class="rounded-lg bg-gray-100 dark:bg-gray-700 py-1.5 px-2">
            {chapt.num} - {manga.name}
          </a>
        {/each}
      {/each}
    </div>
  {/if}
</div>
