<script>
  import PlayButton from "./components/PlayButton.svelte";
  import SwitchButton from "./components/SwitchButton.svelte";
  import LoadButton from "./components/LoadButton.svelte";
  import Subtitle from "./components/Subtitle.svelte";
  import LeftPanel from "./components/LeftPanel.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  let subtitleContent = "";
  let vocabularyContent = "";
  let pronunciationContent = "";
  let referencesContent = "";

  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });
  webview.listen("videonotes://start-notes", ({ payload: videoNotes }) => {
    for (const videoNote of videoNotes) {
      const {
        id,
        payload: { type, content },
      } = videoNote;
      switch (type) {
        case "default":
          subtitleContent = content;
          break;
        case "vocabulary":
          console.log("START VOCAB PANEL", id);
          vocabularyContent = content;
          break;
        case "pronunciation":
          pronunciationContent = content;
          break;
        case "references":
          referencesContent = content;
          break;

        default:
          break;
      }
    }
  });
  webview.listen("videonotes://end-notes", ({ payload: videoNotes }) => {
    for (const videoNote of videoNotes) {
      const {
        id,
        payload: { type },
      } = videoNote;

      switch (type) {
        case "default":
          subtitleContent = "";
          break;
        case "vocabulary":
          console.log("END VOCAB PANEL", id);
          vocabularyContent = "";
          break;
        case "pronunciation":
          pronunciationContent = "";
          break;
        case "references":
          referencesContent = "";
          break;

        default:
          break;
      }
    }
  });

  export let name;
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
    <Subtitle content={subtitleContent} />
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
