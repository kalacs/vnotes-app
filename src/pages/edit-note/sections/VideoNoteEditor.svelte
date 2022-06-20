<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Span from "../../../extensions/span";
  import VideoNote from "../../../extensions/video-note";
  import VideoNoteReference from "../../../extensions/video-note-reference";

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

  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [VideoNoteReference, Span, StarterKit, VideoNote],
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
    <button class="button" on:click={importSRTFile}>Import .srt</button>
    <button class="button" on:click={loadVideoNote}>Open VideoNote</button>
    <div class="field has-addons">
      {#if editor}
        <p class="control">
          <button
            class="button is-primary is-outlined"
            on:click={() =>
              editor
                .chain()
                .insertContentAt(
                  0,
                  '<extra-information start="00:04.23" end="00:18.234234" type="references">Content</extra-information>'
                )
                .run()}
            class:is-hovered={editor.isActive("extraInformation", {
              type: "references",
            })}>Add references</button
          >
        </p>
        <p class="control">
          <button
            class="button is-info is-outlined"
            on:click={() =>
              editor
                .chain()
                .insertContentAt(
                  0,
                  '<extra-information start="00:04.23" end="00:18.234234" type="vocabulary">Content</extra-information>'
                )
                .run()}
            class:is-hovered={editor.isActive("extraInformation", {
              type: "vocabulary",
            })}>Add vocabulary</button
          >
        </p>
        <p class="control">
          <button
            class="button is-danger is-outlined"
            on:click={() =>
              editor
                .chain()
                .insertContentAt(
                  0,
                  "<video-note><video-note-reference>Content</video-note-reference></video-note>"
                )
                .run()}
            class:is-hovered={editor.isActive("extraInformation", {
              type: "pronunciation",
            })}>Add pronunciation</button
          >
        </p>
        <p class="control">
          <button
            class="button is-dark is-outlined"
            on:click={() =>
              editor
                .chain()
                .insertContentAt(
                  0,
                  '<video-note start="00:04.23" end="00:18.234234">Content</video-note>'
                )
                .run()}
            class:is-hovered={editor.isActive("videoNote")}
            >Add VideoNote</button
          >
        </p>{/if}
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
