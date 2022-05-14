<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow } from "@tauri-apps/api/window";

  const webview = new WebviewWindow("main");
  function handleClick(event) {
    invoke("plugin:videonote|load_notes");
  }
</script>

<button on:click={handleClick}>
  <span>Load notes</span>
</button>

<style>
  button {
    position: fixed;
    bottom: 20px;
    left: 100px;
    border-radius: 45px;
    border: 1px solid transparent;
    padding: 22px 0;
  }
</style>
