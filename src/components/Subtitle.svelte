<script lang="js">
  export let note;
</script>

<div id="subtitle" class="columns is-centered">
  <div class="column is-half">
    <p
      data-note-id={note ? note.id : ""}
      class="has-text-white is-size-3 has-text-weight-semibold"
    >
      {@html note ? note.payload.content : ""}
    </p>
  </div>
</div>

<style>
  div {
    position: absolute;
    text-align: center;
    width: 100%;
  }
</style>
