<script lang="ts">
  import { fade } from "svelte/transition"
  import { cubicIn, cubicOut } from "svelte/easing"
</script>

<div
  in:fade={{ easing: cubicOut, duration: 100, delay: 100 }}
  out:fade={{ easing: cubicIn, duration: 200 }}
>
  <slot></slot>
</div>
