<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  import { WebviewWindow } from "@tauri-apps/api/window";
  const webview = new WebviewWindow("main");
  webview.listen("videonotes://video-player-found", () => {
    console.log("VIDEO PLAYER IS READY");
  });
  let buttonId = 0; // 0: open, 1:switch
  function handleClick(event) {
    if (buttonId === 0) {
      invoke("plugin:videonote|open_window");
      buttonId = 1;
    } else {
      invoke("plugin:videonote|switch_to_provider");
    }
  }
</script>

<button
  on:click={handleClick}
  id={buttonId === 0 ? "open-button" : "switch-button"}
>
  <span>{buttonId === 0 ? "Open" : "Back"}</span>
</button>

<style>
  button {
  }
</style>
