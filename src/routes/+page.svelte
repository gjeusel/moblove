<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"

  let name = $state("Boby")
  let greetMsg = $state("")

  async function greet(event: Event) {
    event.preventDefault()

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>

<div class="flex flex-col items-center px-4">
  <img
    src="/icon.png"
    class="m-4 h-20 w-20 self-center rounded-full p-3 transition-all duration-700 hover:drop-shadow-[0_0_2em_#24c8db] hover:filter"
    style="will-change: filter"
    alt="MobLove"
  />
  <h1 class="text-center text-lg font-normal">Mangas Chapters</h1>

  <form class="flex items-center space-x-4 text-base font-medium" onsubmit={greet}>
    <input type="text" class="px-2" placeholder="Enter a name..." bind:value={name} />
    <button class="btn-cyan py-1 px-2" type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
