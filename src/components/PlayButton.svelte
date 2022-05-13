<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow } from "@tauri-apps/api/window";

  const webview = new WebviewWindow("main");
  let buttonId = 1; // 0: open, 1:switch
  function handleClick(event) {
    if (buttonId === 0) {
      invoke("plugin:videonote|play_content");
      buttonId = 1;
    } else {
      invoke("plugin:videonote|pause_content");
      buttonId = 0;
    }
  }
</script>

<button
  on:click={handleClick}
  id={buttonId === 0 ? "play-button" : "pause-button"}
>
  <span>{buttonId === 0 ? "Play" : "Pause"}</span>
</button>

<style>
  button {
    position: fixed;
    bottom: 20px;
    left: 60px;
    border-radius: 45px;
    border: 1px solid transparent;
    padding: 22px 0;
  }
</style>
