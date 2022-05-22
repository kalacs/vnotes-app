<script lang="ts">
  import PlayButton from "./components/PlayButton.svelte";
  import SwitchButton from "./components/SwitchButton.svelte";
  import LoadButton from "./components/LoadButton.svelte";
  import Subtitle from "./components/Subtitle.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  let content: string = "SUBTITLE";
  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });
  webview.listen("videonotes://video-player-event", ({ payload }: any) => {
    if (payload.name === "cue") {
      const data = payload.payload.data;
      const delay = Math.round((data.endTime - data.startTime) * 1000);
      content = payload.payload.data.payload.content;
      console.log({ delay, data });
      setTimeout(() => {
        content = "";
      }, delay);
    }
  });

  export let name: string;
</script>

<main>
  <SwitchButton />
  <PlayButton />
  <LoadButton />
  <Subtitle {content} />
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
