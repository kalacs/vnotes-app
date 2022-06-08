<script lang="js">
  export let note;
  //const DEFAULT_VALUE = `Lorem ipsssssum dolor sit amet, <span class="has-text-info is-clickable" data-chunk-type="vocabulary">consectetur adipiscing</span> elit. Donec odio eros, vulputate varius <span class="has-text-primary is-clickable" data-chunk-type="references">tortor non, bibendum</span> semper libero. Curabitur porttitor ligula in <span class="has-text-danger is-clickable" data-chunk-type="pronunciation">lobortis</span> sagittis.`;
  const DEFAULT_VALUE = "";
</script>

<div id="subtitle" class="columns is-centered" on:click>
  <div class="column">
    <p
      data-note-id={note ? note.id : ""}
      class="has-text-white is-size-3 has-text-weight-semibold"
    >
      {@html note ? note.payload.content : DEFAULT_VALUE}
    </p>
  </div>
</div>

<style>
  div {
    text-align: center;
    width: 100%;
  }
</style>
