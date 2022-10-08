<script>
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'

  onMount(async () => {
    console.log(await invoke('check_mpd'))
  })
</script>

<main  class="bg-gray-300 w-screen h-full relative">
  <p class="p-5 relative">
    Hello World
  </p>
</main>
