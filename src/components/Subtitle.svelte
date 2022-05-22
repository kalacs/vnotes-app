<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  export let content;
</script>

<div>
  <span>{content}</span>
</div>

<style>
  div {
    position: absolute;
    bottom: 3em;
    color: yellow;
    font-weight: 700;
    font-size: 4em;
    text-align: center;
    width: 100%;
  }
</style>
