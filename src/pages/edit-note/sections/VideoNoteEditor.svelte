<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import { HandleVideoNotes } from "../../../extensions/handle-videonotes";
  import ExtraInformation from "../../../extensions/extra-information";
  import VideoNote from "../../../extensions/video-note";
  let element;
  let editor;

  function importSRTFile() {
    invoke("plugin:videonote|import_srt_file", {
      fileName: "Friends - 1x01.en.srt",
    })
      .then((content) => {
        editor.commands.setContent(content);
      })
      .catch(console.log);
  }

  onMount(() => {
    editor = new Editor({
      element: element,
      extensions: [ExtraInformation, StarterKit, VideoNote],
      content: "<p>Hello World! 🌍️ </p>",
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
            class:active={editor.isActive("extra-information", {
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
            class:active={editor.isActive("heading", { level: 2 })}
            >Add vocabulary</button
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
                  '<extra-information start="00:04.23" end="00:18.234234" type="pronunciation">Content</extra-information>'
                )
                .run()}
            class:active={editor.isActive("heading", { level: 2 })}
            >Add pronunciation</button
          >
        </p>
        <p class="control">
          <button
            class="button is-outlined"
            on:click={() =>
              editor
                .chain()
                .insertContentAt(
                  0,
                  '<video-note start="00:04.23" end="00:18.234234">Content</video-note>'
                )
                .run()}
            class:active={editor.isActive("paragraph")}>Add VideoNote</button
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
