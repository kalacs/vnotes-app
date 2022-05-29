<script lang="ts">
  import PlayButton from "./components/PlayButton.svelte";
  import SwitchButton from "./components/SwitchButton.svelte";
  import LoadButton from "./components/LoadButton.svelte";
  import Subtitle from "./components/Subtitle.svelte";
  import LeftPanel from "./components/LeftPanel.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  let content: string = "SUBTITLE";
  let vocabularyContent: string = "";
  let pronunciationContent: string = "";
  let referencesContent: string = "";

  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });
  webview.listen("videonotes://video-player-event", ({ payload }: any) => {
    console.log(payload);
    if (payload.name === "startCue") {
      if (payload.payload.data.payload.type === "default") {
        content = payload.payload.data.payload.content;
      }

      if (payload.payload.data.payload.type === "vocabulary") {
        vocabularyContent = payload.payload.data.payload.content;
      }

      if (payload.payload.data.payload.type === "pronunciation") {
        pronunciationContent = payload.payload.data.payload.content;
      }

      if (payload.payload.data.payload.type === "references") {
        referencesContent = payload.payload.data.payload.content;
      }
    }
    if (payload.name === "endCue") {
      if (payload.payload.data.payload.type === "default") {
        content = "";
      }

      if (payload.payload.data.payload.type === "vocabulary") {
        vocabularyContent = "";
      }

      if (payload.payload.data.payload.type === "pronunciation") {
        pronunciationContent = "";
      }

      if (payload.payload.data.payload.type === "references") {
        referencesContent = "";
      }
    }
  });

  export let name: string;
</script>

<section id="root" class="section">
  <section id="left-area" class="section">
    <LeftPanel {vocabularyContent} {pronunciationContent} {referencesContent} />
  </section>
  <section id="down-area" class="section">
    <SwitchButton />
    <PlayButton />
    <LoadButton />
  </section>
  <section id="down-area-2" class="section">
    <Subtitle {content} />
  </section>
</section>

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
  #down-area {
    margin-top: 31em;
    padding: 0;
  }
</style>
