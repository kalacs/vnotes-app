<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow } from "@tauri-apps/api/window";

  const webview = new WebviewWindow("main");
  function handleClick(event) {
    invoke("plugin:videonote|load_notes", {
      id: 1,
    });
  }
</script>

<button on:click={handleClick}>
  <span>Load notes</span>
</button>

<style>
  button {
  }
</style>
