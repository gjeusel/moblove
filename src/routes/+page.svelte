<script lang="ts">
  import { Button } from "$lib/components/ui/button"
</script>

<div class="flex flex-col items-center">
  <img
    src="/icon.png"
    class="mt-2 size-20 self-center rounded-full p-2 transition-all duration-700 hover:drop-shadow-[0_0_2em_#24c8db] hover:filter"
    style="will-change: filter"
    alt="MobLove"
  />
  <p>Hi there, how can I help you today?</p>
  <Button onclick={() => console.log("HELLO THERE")}>Click me</Button>
</div>
