<script lang="ts">
  import { fade } from "svelte/transition"
  import { cubicIn, cubicOut } from "svelte/easing"

  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js"
</script>

<Breadcrumb.Root>
  <Breadcrumb.List>
    <Breadcrumb.Item>
      <Breadcrumb.Link href="/manga">Mangas</Breadcrumb.Link>
    </Breadcrumb.Item>
    <!-- {#if focusChapt && focusManga} -->
    <!--   <Breadcrumb.Separator /> -->
    <!--   <Breadcrumb.Item> -->
    <!--     <Breadcrumb.Page> -->
    <!--       <span class="capitalize">{sanitize.name(focusManga.name)}</span> -->
    <!--       - -->
    <!--       <span>{focusChapt.num}</span> -->
    <!--     </Breadcrumb.Page> -->
    <!--   </Breadcrumb.Item> -->
    <!-- {/if} -->
  </Breadcrumb.List>
</Breadcrumb.Root>

<div
  in:fade={{ easing: cubicOut, duration: 100, delay: 100 }}
  out:fade={{ easing: cubicIn, duration: 200 }}
>
  <slot></slot>
</div>
