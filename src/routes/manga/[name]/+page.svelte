<script lang="ts">
  import type { PageData } from "./$types"

  let { data }: { data: PageData } = $props()
</script>

{#await data.images}
  <div class="flex flex-col gap-y-4">
    <div class="h-80 w-full animate-pulse bg-gray-200 dark:bg-gray-700"></div>
    <div class="h-80 w-full animate-pulse bg-gray-200 dark:bg-gray-700"></div>
    <div class="h-80 w-full animate-pulse bg-gray-200 dark:bg-gray-700"></div>
  </div>
{:then images}
  <div class="flex flex-col">
    {#each images as img_data, i}
      <img src={img_data} alt={`img-${i}`} />
    {/each}
  </div>
{:catch error}
  <p>Something went wrong: {error.message}</p>
{/await}
