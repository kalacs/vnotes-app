<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Span from "../../../extensions/span";
  import VideoNote from "../../../extensions/video-note/video-note";
  import VideoNoteReference from "../../../extensions/video-note-reference";
  import BubbleMenu from "../../../extensions/bubble-menu";
  import Chapter from "../../../extensions/chapter";

  let element;
  let editor;

  function importSRTFile() {
    invoke("plugin:videonote|import_srt_file", {
      fileName: "Friends - 1x01.en.srt",
    })
      .then((content) => {
        editor.commands.setContent(content, true);
      })
      .catch(console.log);
  }

  function loadVideoNote() {
    invoke("plugin:videonote|load_notes")
      .then(() =>
        invoke("plugin:videonote|open_video_notes", {
          fileName: "Friends - 1x01.en.srt",
        })
      )
      .then((content) => {
        editor.commands.setContent(content, true);
        editor.commands.markReferences();
      })
      .catch(console.log);
  }

  const saveNotes = () => {
    console.log(editor.getJSON());
    invoke("plugin:videonote|save_notes", {
      editorJson: editor.getJSON(),
    })
      .then((result) => {
        window.alert(result);
      })
      .catch(console.log);
  };

  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [
        VideoNoteReference,
        Span,
        StarterKit,
        VideoNote,
        BubbleMenu.configure({
          element: document.querySelector(".menu"),
        }),
        Chapter,
      ],
      content: "",
      onTransaction: () => {
        // force re-render so `editor.isActive` works as expected
        editor = editor;
      },
    });
  });

  onDestroy(() => {
    if (editor) {
      editor.destroy();
    }
  });
</script>

<div class="columns is-multiline">
  <div class="column is-full">
    <button class="button" on:click={importSRTFile}
      >Import From Subtitles</button
    >
    <button class="button" on:click={loadVideoNote}>Load Notes</button>
    <button class="button" on:click={saveNotes}>Save</button>
    <div class="buttons has-addons menu bubble-menu are-small is-rounded">
      {#if editor}
        <button
          class="button is-primary is-light is-rounded"
          on:click={() => editor.chain().toggleAnnotation("references").run()}
          class:is-light={!editor.isActive("videoNoteReference", {
            type: "references",
          })}>Reference</button
        >
        <button
          class="button is-info is-light is-rounded"
          on:click={() => editor.chain().toggleAnnotation("vocabulary").run()}
          class:is-light={!editor.isActive("videoNoteReference", {
            type: "vocabulary",
          })}>Vocabulary</button
        >
        <button
          class="button is-danger is-light is-rounded"
          on:click={() =>
            editor.chain().toggleAnnotation("pronunciation").run()}
          class:is-light={!editor.isActive("videoNoteReference", {
            type: "pronunciation",
          })}>Pronunciation</button
        >
      {/if}
    </div>
  </div>
  <div class="column is-half editor-container">
    <div class="content" bind:this={element} />
  </div>
</div>

<style>
  button.active {
    background: black;
    color: white;
  }
  .editor-container {
    border: 1px solid black;
    border-radius: 5px;
  }
  .editor-container > div > div {
    outline: 0px solid transparent;
  }
</style>
