<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import VideoNoteEditor from "./components/VideoNoteEditor.svelte";
  import Worksheets from "./components/Worksheets.svelte";

  /*
  {
    "id":1,
    "title": "Friends S01 E01",
    "variation": "B2",
    "version": "0.1",
    "compability": [
      {
        "id": 1,
        "name": "HBO Max"
      }
    ]
  }
  */

  export let worksheet;
  export let worksheets = [];

  onMount(() => {
    invoke("plugin:videonote|get_all_worksheets")
      .then((result) => {
        worksheets = result;
      })
      .catch(console.log);
  });
</script>

<div id="main" class="container is-fullhd">
  <section id="video-note-info" class="section">
    <h1 class="is-size-3">Edit notes for Friends S01 E01</h1>
  </section>
  <section id="worksheets" class="section">
    <Worksheets
      on:select-worksheet={(selectedWorksheet) => {
        worksheet = selectedWorksheet;
      }}
      {worksheets}
    />
  </section>
  {#if worksheet}
    <section id="video-note-editor" class="section">
      <VideoNoteEditor />
    </section>
  {/if}
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
