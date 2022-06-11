<script>
  import EditNote from "./pages/edit-note/EditNote.svelte";
  import ErrorNotFound from "./pages/ErrorNotFound.svelte";
  import RenderNote from "./pages/RenderNote.svelte";
  const DEFAULT_ROUTE = "edit-note";

  export let route = DEFAULT_ROUTE;
</script>

{#if route === "render-note"}
  <RenderNote />
{:else if route === "edit-note"}
  <EditNote />
{:else}
  <ErrorNotFound />
{/if}
