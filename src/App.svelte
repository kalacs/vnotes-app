<script>
  import PlayButton from "./components/PlayButton.svelte";
  import SwitchButton from "./components/SwitchButton.svelte";
  import LoadButton from "./components/LoadButton.svelte";
  import Subtitle from "./components/Subtitle.svelte";
  import LeftPanel from "./components/LeftPanel.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  let subtitleNote = null;
  let vocabularyNote = null;
  let pronunciationNote = null;
  let referencesNote = null;

  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });
  webview.listen("videonotes://start-notes", ({ payload: videoNotes }) => {
    for (const videoNote of videoNotes) {
      const {
        id,
        payload: { type },
      } = videoNote;
      switch (type) {
        case "default":
          subtitleNote = videoNote;

          if (vocabularyNote) {
            for (const { id, phrase } of vocabularyNote.payload.references) {
              const phraseStartIndex =
                subtitleNote.payload.content.indexOf(phrase);
              if (subtitleNote.id === id && phraseStartIndex > -1) {
                if (!subtitleNote) {
                  break;
                }
                const fragment = subtitleNote.payload.content.substring(
                  phraseStartIndex,
                  phraseStartIndex + phrase.length
                );
                const replaceString = `<span class="has-text-info">${fragment}</span>`;
                subtitleNote.payload.content =
                  subtitleNote.payload.content.replace(phrase, replaceString);
              }
            }
          }

          if (pronunciationNote) {
            for (const { id, phrase } of pronunciationNote.payload.references) {
              const phraseStartIndex =
                subtitleNote.payload.content.indexOf(phrase);
              if (subtitleNote.id === id && phraseStartIndex > -1) {
                if (!subtitleNote) {
                  break;
                }
                const fragment = subtitleNote.payload.content.substring(
                  phraseStartIndex,
                  phraseStartIndex + phrase.length
                );
                const replaceString = `<span class="has-text-danger">${fragment}</span>`;
                subtitleNote.payload.content =
                  subtitleNote.payload.content.replace(phrase, replaceString);
              }
            }
          }

          if (referencesNote) {
            for (const { id, phrase } of referencesNote.payload.references) {
              const phraseStartIndex =
                subtitleNote.payload.content.indexOf(phrase);
              if (subtitleNote.id === id && phraseStartIndex > -1) {
                if (!subtitleNote) {
                  break;
                }
                const fragment = subtitleNote.payload.content.substring(
                  phraseStartIndex,
                  phraseStartIndex + phrase.length
                );
                const replaceString = `<span class="has-text-primary">${fragment}</span>`;
                subtitleNote.payload.content =
                  subtitleNote.payload.content.replace(phrase, replaceString);
              }
            }
          }

          break;
        case "vocabulary":
          vocabularyNote = videoNote;
          break;
        case "pronunciation":
          pronunciationNote = videoNote;
          break;
        case "references":
          referencesNote = videoNote;
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
          subtitleNote = null;
          break;
        case "vocabulary":
          vocabularyNote = null;
          break;
        case "pronunciation":
          pronunciationNote = null;
          break;
        case "references":
          referencesNote = null;
          break;

        default:
          break;
      }
    }
  });
</script>

<section id="root" class="section">
  <section id="left-area" class="section">
    <LeftPanel {vocabularyNote} {pronunciationNote} {referencesNote} />
  </section>
  <section id="down-area" class="section">
    <SwitchButton />
    <PlayButton />
    <LoadButton />
  </section>
  <section id="down-area-2" class="section">
    <Subtitle note={subtitleNote} />
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
