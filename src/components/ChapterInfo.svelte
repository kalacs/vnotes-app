<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");

  const DEFAULT_VALUE = "Chapter has not been started";
  export let chapters;
  export let currentTime = 0;

  $: chapter = getChapter(chapters, currentTime);
  $: chapterIndex = getChapterIndex(chapters, currentTime);
  $: previousButtonEnabled = chapterIndex > 0;
  $: nextButtonEnabled = chapters && chapterIndex < chapters.length - 1;

  function testIfChapterIsCurrent(currentTime) {
    return ({ start, end }) => start <= currentTime && currentTime <= end;
  }

  function getChapter(chapters, currentTime) {
    return chapters ? chapters.find(testIfChapterIsCurrent(currentTime)) : null;
  }

  function getChapterIndex(chapters, currentTime) {
    return chapters
      ? chapters.findIndex(testIfChapterIsCurrent(currentTime))
      : null;
  }

  function seekToPreviousChapter() {
    if (chapterIndex !== null && chapterIndex > 0) {
      invoke("plugin:videonote|seek_content", {
        time: chapters[chapterIndex - 1].start - 1,
      });
    }
  }

  function seekToNextChapter() {
    if (chapterIndex !== null && chapterIndex < chapters.length - 1) {
      invoke("plugin:videonote|seek_content", {
        time: chapters[chapterIndex + 1].start - 1,
      });
    }
  }

  function formatSecond(seconds) {
    const remains = Math.round(seconds) % 60;
    return `${(Math.round(seconds) - remains) / 60}:${
      remains < 10 ? `0${remains}` : remains
    }`;
  }
</script>

<div class="columns is-centered is-gapless is-multiline">
  <div class="column is-full">
    <div class="field has-addons">
      {#if previousButtonEnabled}
        <p class="control">
          <button class="button is-rounded" on:click={seekToPreviousChapter}>
            <span class="icon is-large">
              <i class="mdi mdi-arrow-left mdi-24px" />
            </span>
          </button>
        </p>
      {/if}
      <p class="control">
        <button class="button is-rounded" class:is-loading={!chapter}>
          <span class="is-size-5"
            >{chapter
              ? `Chapter ${chapterIndex + 1}. ${chapter.title}`
              : DEFAULT_VALUE}</span
          >
        </button>
      </p>
      {#if nextButtonEnabled}
        <p class="control">
          <button class="button is-rounded" on:click={seekToNextChapter}>
            <span class="icon is-large">
              <i class="mdi mdi-arrow-right mdi-24px" />
            </span>
          </button>
        </p>
      {/if}
    </div>
  </div>
  <div class="column is-5">
    <div class="columns">
      <div class="column">
        <span
          class="is-uppercase has-text-white is-size-5-desktop has-text-weight-bold"
          >{chapter ? formatSecond(chapter.start) : ""}</span
        >
      </div>
      <div class="column is-9">
        <progress
          class="progress is-success"
          value={chapter ? currentTime - chapter.start : 0}
          max={chapter ? chapter.end - chapter.start : 100}
        />
      </div>
      <div class="column">
        <span
          class="is-uppercase has-text-white  is-size-5-desktop has-text-weight-bold"
          >{chapter ? formatSecond(chapter.end) : ""}</span
        >
      </div>
    </div>
  </div>
</div>

<style>
  .has-addons {
    justify-content: center;
  }
  progress {
    display: inline-block;
  }
</style>
