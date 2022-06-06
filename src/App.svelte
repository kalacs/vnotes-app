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

<div id="main" class="container is-fullhd">
  <section id="chapter-info">
    <div class="columns is-centered is-gapless is-multiline">
      <div class="column is-full">
        <div class="field has-addons">
          <p class="control">
            <button class="button is-rounded">
              <span class="icon is-large">
                <i class="mdi mdi-arrow-left mdi-24px" />
              </span>
              <span>Left</span>
            </button>
          </p>
          <p class="control">
            <button class="button">
              <span class="is-size-5">Chapter I: Name of the clip</span>
            </button>
          </p>
          <p class="control">
            <button class="button is-rounded">
              <span>Right</span>
              <span class="icon is-large">
                <i class="mdi mdi-arrow-right mdi-24px" />
              </span>
            </button>
          </p>
        </div>
      </div>
      <div class="column is-one-third">
        <progress class="progress is-success" value="60" max="100">60%</progress
        >
      </div>
    </div>
  </section>
  <section id="left-area" class="section">
    <LeftPanel {vocabularyNote} {pronunciationNote} {referencesNote} />
  </section>
  <section id="down-area" class="section">
    <div class="columns">
      <div class="column is-one-fifth">
        <SwitchButton />
        <PlayButton />
        <LoadButton />
      </div>
    </div>
    <div class="columns">
      <div class="column is-full">
        <Subtitle note={subtitleNote} />
      </div>
    </div>
  </section>
</div>

<style>
  #main {
    margin: 0 auto;
    height: 100%;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
  #down-area {
    padding: 1em 288px;
    position: fixed;
    bottom: 11em;
    width: 100%;
    left: 0px;
    height: 25%;
    background-color: black;
    opacity: 80%;
  }

  #chapter-info .has-addons {
    justify-content: center;
  }
</style>
