<script>
  const DEFAULT_VALUE = "Chapter has not been started";
  export let chapters;
  export let currentTime = 0;

  $: chapter = getChapter(chapters, currentTime);
  $: chapterIndex = getChapterIndex(chapters, currentTime);
  $: previousButtonEnabled = chapterIndex > 0;
  $: nextButtonEnabled = chapters && chapterIndex < chapters.length - 1;

  function testIfChapterIsCurrent(currentTime) {
    return ({ startTime, endTime }) =>
      startTime <= currentTime && currentTime <= endTime;
  }

  function getChapter(chapters, currentTime) {
    return chapters ? chapters.find(testIfChapterIsCurrent(currentTime)) : null;
  }

  function getChapterIndex(chapters, currentTime) {
    return chapters
      ? chapters.findIndex(testIfChapterIsCurrent(currentTime))
      : null;
  }
</script>

<div class="columns is-centered is-gapless is-multiline">
  <div class="column is-full">
    <div class="field has-addons">
      {#if previousButtonEnabled}
        <p class="control">
          <button class="button is-rounded">
            <span class="icon is-large">
              <i class="mdi mdi-arrow-left mdi-24px" />
            </span>
          </button>
        </p>
      {/if}
      <p class="control">
        <button class="button is-rounded">
          <span class="is-size-5"
            >{chapter
              ? `Chapter ${chapterIndex + 1}. ${chapter.title}`
              : DEFAULT_VALUE}</span
          >
        </button>
      </p>
      {#if nextButtonEnabled}
        <p class="control">
          <button class="button is-rounded">
            <span class="icon is-large">
              <i class="mdi mdi-arrow-right mdi-24px" />
            </span>
          </button>
        </p>
      {/if}
    </div>
  </div>
  <div class="column is-one-third">
    <progress
      class="progress is-success"
      value={chapter ? currentTime - chapter.startTime : 0}
      max={chapter ? chapter.endTime - chapter.startTime : 100}
    />
  </div>
</div>

<style>
  .has-addons {
    justify-content: center;
  }
</style>
