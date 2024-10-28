<script lang="ts">
  import type { PageData } from "./$types"

  let { data }: { data: PageData } = $props()
</script>

<div class="flex flex-col">
  {#each data.images as img_data}
    <img src={img_data} alt="manga img" />
  {/each}
</div>
