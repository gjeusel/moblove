<script lang="ts">
  import type { PageData } from "./$types"

  let { data = { mangas: [] } }: { data: PageData } = $props()

  let names = $derived([...new Set(data.mangas.map((e) => e.name))])
</script>

<div>
  <h1 class="my-2 text-center text-lg font-normal">Mangas Chapters</h1>

  <div class="flex flex-col gap-y-2">
    {#each data.mangas as manga}
      {#each manga.chapters.slice(manga.chapters.length - 2, manga.chapters.length) as chapt}
        <a
          href={chapt.url}
          target="_blank"
          class="rounded-lg bg-gray-100 px-2 py-1.5 dark:bg-gray-700"
        >
          {chapt.num} - {manga.name}
        </a>
      {/each}
    {/each}
  </div>
</div>
