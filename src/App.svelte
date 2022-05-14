<script lang="ts">
  import PlayButton from "./components/PlayButton.svelte";
  import SwitchButton from "./components/SwitchButton.svelte";
  import LoadButton from "./components/LoadButton.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });

  export let name: string;
</script>

<main>
  <SwitchButton />
  <PlayButton />
  <LoadButton />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
