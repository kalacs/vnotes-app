<script lang="js">
  import { invoke } from "@tauri-apps/api/tauri";
  export let content;
</script>

<div id="subtitle" class="columns is-centered">
  <div class="column is-half">
    <p>{content}</p>
  </div>
</div>

<style>
  div {
    position: absolute;
    bottom: -6em;
    font-weight: 700;
    font-size: 2em;
    text-align: center;
    width: 100%;
  }
</style>
