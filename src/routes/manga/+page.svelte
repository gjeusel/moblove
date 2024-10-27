<script lang="ts">
  import type { PageData } from "./$types"

  let { data = { mangas: [] } }: { data: PageData } = $props()
</script>

<div>
  <h1 class="my-2 text-center text-lg font-normal">Mangas Chapters</h1>

  <div class="flex flex-col-reverse gap-y-3 px-4">
    {#each data.mangas as manga}
      <div class="">
        <h2 class="mb-1 text-lg font-medium capitalize">{manga.name.split("-").join(" ")}</h2>
        <div class="inline-flex flex-row-reverse gap-x-2">
          {#each manga.chapters.slice(manga.chapters.length - 4, manga.chapters.length) as chapt}
            <a
              href={chapt.url}
              target="_blank"
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
