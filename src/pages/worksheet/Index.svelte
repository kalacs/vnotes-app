<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import VideoNoteEditor from "./components/VideoNoteEditor.svelte";
  import Worksheets from "./components/Worksheets.svelte";
  import Details from "./components/Form.svelte";

  /*
  {
    "id":1,
    "title": "Friends S01 E01",
    "variation": "B2",
    "version": 1,
    "compatibility": [
      {
        "id": 1,
        "name": "HBO Max"
      }
    ]
  }
  */

  let worksheet = {
    id: "",
    title: "",
    variation: "0",
    version: 1,
    compatibility: [],
  };
  let worksheets = [];
  let editorContent = "";

  $: {
    if (worksheet?.id) {
      invoke("plugin:videonote|load_notes", {
        id: worksheet.id,
      })
        .then(() => invoke("plugin:videonote|open_video_notes"))
        .then((result) => {
          editorContent = result;
        })
        .catch(console.error);
    }
  }

  function importSRTFile() {
    invoke("plugin:videonote|import_srt_file", {
      fileName: "Friends - 1x01.en.srt",
    })
      .then((content) => {
        editor.commands.setContent(content, true);
      })
      .catch(console.log);
  }

  const createNotes = () => {
    console.log(editor.getJSON());
    invoke("plugin:videonote|save_notes", {
      editorJson: editor.getJSON(),
    })
      .then((result) => {
        window.alert(result);
      })
      .catch(console.log);
  };

  const updateNotes = () => {
    console.log(editor.getJSON());
    invoke("plugin:videonote|save_notes", {
      editorJson: editor.getJSON(),
    })
      .then((result) => {
        window.alert(result);
      })
      .catch(console.log);
  };

  const saveForm = ({ detail }) => {
    invoke("plugin:videonote|save_worksheet", { payload: detail })
      .then(({ error, message }) => {
        console.error(error);
        window.alert(message);
      })
      .catch(console.log);
  };

  onMount(() => {
    invoke("plugin:videonote|get_all_worksheets")
      .then((result) => {
        worksheets = result;
      })
      .catch(console.error);
  });
</script>

<div id="main" class="container is-fullhd">
  <section id="video-note-info" class="section">
    <h1 class="is-size-3">Worksheets</h1>
  </section>
  <section id="worksheets" class="section">
    <Worksheets
      on:select-worksheet={({ detail }) => {
        worksheet = detail;
      }}
      {worksheets}
    />
  </section>
  <section id="worksheet-detail" class="section">
    <Details on:save-worksheet-details={saveForm} {worksheet} />
  </section>
  {#if worksheet}
    <button class="button" on:click={importSRTFile}
      >Import from subtitles (.srt file)</button
    >
    <section id="video-note-editor" class="section">
      <VideoNoteEditor {editorContent} />
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
