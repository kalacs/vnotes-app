<script>
  import VideoNoteEditor from "./sections/VideoNoteEditor.svelte";
</script>

<div id="main" class="container is-fullhd">
  <section id="video-note-info" class="section">
    <h1 class="is-size-3">VideoNote editor</h1>
  </section>
  <section id="video-note-editor" class="section">
    <VideoNoteEditor />
  </section>
</div>

<style>
  #main {
    margin: 0 auto;
    background-color: white;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
