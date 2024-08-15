<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import * as ContextMenu from "$lib/components/ui/context-menu/index.js";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger class="flex h-screen w-full items-center justify-center">
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
        </a>
        <a href="https://kit.svelte.dev" target="_blank">
          <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

      <form class="row" on:submit|preventDefault={greet}>
        <input
          id="greet-input"
          placeholder="Enter a name..."
          bind:value={name}
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item>Profile</ContextMenu.Item>
    <ContextMenu.Item>Billing</ContextMenu.Item>
    <ContextMenu.Item>Team</ContextMenu.Item>
    <ContextMenu.Item>Subscription</ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>
