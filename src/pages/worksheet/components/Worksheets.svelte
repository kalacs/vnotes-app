<script>
  import { createEventDispatcher } from "svelte";
  export let worksheets;

  const dispatch = createEventDispatcher();

  function sayHello(id) {
    dispatch(
      "select-worksheet",
      worksheets.find(({ id: worksheetId }) => id === worksheetId)
    );
  }
</script>

<div class="box">
  <nav class="panel">
    <p class="panel-heading">Your available worksheets</p>
    {#each worksheets as worksheet}
      <a class="panel-block" href="#" on:click={sayHello(worksheet.id)}>
        <span class="panel-icon">
          <i class="fas fa-book" aria-hidden="true" />
        </span>
        {worksheet.title} ({worksheet.variation}) - v{worksheet.version}
      </a>
    {/each}
  </nav>
</div>
