<script>
  import { onMount, onDestroy, afterUpdate } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Span from "../../../extensions/span";
  import VideoNote from "../../../extensions/video-note/video-note";
  import VideoNoteReference from "../../../extensions/video-note-reference";
  import BubbleMenu from "../../../extensions/bubble-menu";
  import Chapter from "../../../extensions/chapter";

  export let editorContent;
  let element;
  let editor;

  $: {
    if (editorContent) {
      editor.commands.setContent(editorContent, true);
      editor.commands.markReferences();
    }
  }

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
