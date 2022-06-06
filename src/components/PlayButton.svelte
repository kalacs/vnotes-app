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
  class="button is-rounded"
>
  <span class="icon is-large">
    <i
      class="mdi mdi-24px"
      class:mdi-play={buttonId === 0}
      class:mdi-pause={buttonId === 1}
    />
  </span>
</button>

<style>
  button {
  }
</style>
