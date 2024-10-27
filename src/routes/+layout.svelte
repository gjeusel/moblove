<script lang="ts">
  import { BookText, Gamepad, House } from "lucide-svelte"
</script>

<div class="pb-[4.5rem]">
  <slot></slot>
</div>

<div
  class="fixed right-0 bottom-0 left-0 flex justify-around border-t border-gray-300 bg-white py-3 dark:border-gray-700 dark:bg-gray-800"
>
  <a href="/" class="">
    <House class="size-8" />
  </a>
  <a href="/manga" class="">
    <BookText class="size-8" />
  </a>
  <a href="/timesup" class="">
    <Gamepad class="size-8" />
  </a>
</div>
