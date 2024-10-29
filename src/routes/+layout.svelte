<script lang="ts">
  import { onMount } from "svelte"
  import { BookText, Gamepad, House } from "lucide-svelte"

  // import { warn, info, error } from "@tauri-apps/plugin-log"

  // console.info = info
  // console.warn = warn
  // console.error = error

  let isVisible = $state(true)
  let lastScrollY = $state(0)

  let scrollTimer: any | null = null
  const SCROLL_THRESHOLD = 20 // Minimum scroll distance to trigger hide/show
  const SCROLL_DELAY = 50 // Debounce delay in milliseconds

  $effect(() => {
    const isAtBottom = () => {
      // Get total height including overflow
      const totalHeight = document.documentElement.scrollHeight
      // Get current scroll position
      const scrollPosition = window.scrollY + window.innerHeight
      // Add small buffer (20px) to account for rounding and small gaps
      return totalHeight - scrollPosition <= 20
    }

    const handleScroll = () => {
      if (scrollTimer) clearTimeout(scrollTimer)

      scrollTimer = setTimeout(() => {
        const currentScrollY = window.scrollY

        // Always show at top of page
        if (currentScrollY === 0 || isAtBottom()) {
          isVisible = true
          return
        }

        // Calculate scroll direction and distance
        const delta = currentScrollY - lastScrollY
        const distance = Math.abs(delta)

        // Only trigger if scroll distance exceeds threshold
        if (distance > SCROLL_THRESHOLD) {
          // Scrolling down = hide, scrolling up = show
          isVisible = delta < 0
          lastScrollY = currentScrollY
        }
      }, SCROLL_DELAY)
    }

    onMount(() => {
      window.addEventListener("scroll", handleScroll, { passive: true })
      return () => window.removeEventListener("scroll", handleScroll)
    })
  })
</script>

<div class="flex min-h-screen flex-col">
  <main class="flex-1 p-4">
    <slot></slot>
  </main>

  <div
    class="fixed right-0 bottom-0 left-0 flex justify-around border-t border-gray-300 bg-white py-3 transition-transform duration-300 ease-in-out dark:border-gray-700 dark:bg-gray-800"
    class:translate-y-0={isVisible}
    class:translate-y-full={!isVisible}
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
</div>
