<script>
  import { WebviewWindow } from "@tauri-apps/api/window";
  import PlayButton from "../../components/PlayButton.svelte";
  import SwitchButton from "../../components/SwitchButton.svelte";
  import LoadButton from "../../components/LoadButton.svelte";
  import Subtitle from "../../components/Subtitle.svelte";
  import LeftPanel from "../../components/LeftPanel.svelte";
  import ChapterInfo from "../../components/ChapterInfo.svelte";

  const webview = new WebviewWindow("main");
  let subtitleNote = null;
  let vocabularyNote = null;
  let pronunciationNote = null;
  let referencesNote = null;
  let currentTime = 0;
  let chapters = null;
  let leftPanelIsOpened = false;
  let leftPanelSelectedSection = "";
  let html = "";

  webview.listen("videonotes://notes-loaded", () => {
    console.log("NOTES LOADED");
  });
  webview.listen(
    "videonotes://video-player-event",
    ({ payload: { name, currentTime: time } }) => {
      if (name === "timeupdate") {
        currentTime = time;
      }
    }
  );
  webview.listen("videonotes://chapters-loaded", ({ payload }) => {
    chapters = payload;
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
                const replaceString = `<span class="has-text-info is-clickable" data-chunk-type="vocabulary">${fragment}</span>`;
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
                const replaceString = `<span class="has-text-danger is-clickable" data-chunk-type="pronunciation">${fragment}</span>`;
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
                const replaceString = `<span class="has-text-primary is-clickable" data-chunk-type="references">${fragment}</span>`;
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

  function handleSubtitleClick({ target }) {
    if (target.dataset["chunkType"]) {
      if (target.dataset["chunkType"] === leftPanelSelectedSection) {
        leftPanelIsOpened = false;
        leftPanelSelectedSection = "";
        return;
      }
      leftPanelIsOpened = true;
      leftPanelSelectedSection = target.dataset["chunkType"];
    }
  }
</script>

<div id="main" class="container is-fullhd">
  <section id="chapter-info" class="section">
    <ChapterInfo {currentTime} {chapters} />
  </section>
  <section id="left-area" class="section">
    <LeftPanel
      {vocabularyNote}
      {pronunciationNote}
      {referencesNote}
      isOpened={leftPanelIsOpened}
      selectedSection={leftPanelSelectedSection}
    />
  </section>
  <section class="section">{@html html}</section>
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
        <Subtitle note={subtitleNote} on:click={handleSubtitleClick} />
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
    bottom: 8em;
    width: 100%;
    left: 0px;
    height: 25%;
    background-color: black;
    opacity: 90%;
  }
  #down-area > .columns:first-child {
    margin-bottom: 0;
  }
</style>
