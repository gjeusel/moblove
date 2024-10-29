<script lang="ts">
  import type { PageData } from "./$types"
  let { data = { mangas: [] } }: { data: PageData } = $props()
</script>

<div>
  <div class="flex flex-col-reverse gap-y-3 px-4 text-center">
    {#each data.mangas as manga}
      <div class="">
        <h2 class="mb-1 text-lg font-medium capitalize">{manga.name.split("-").join(" ")}</h2>
        <div class="inline-flex flex-row gap-x-2">
          {#each manga.chapters.slice(manga.chapters.length - 4, manga.chapters.length) as chapt}
            <a
              aria-label={`${manga.name} - ${chapt.url}`}
              href={`/mangas/chapter?url=${chapt.url}`}
              class="rounded-lg bg-gray-100 px-2 py-1.5 dark:bg-gray-700"
            >
              {chapt.num}
            </a>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>
