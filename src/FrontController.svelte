<script>
  import WorksheetPage from "./pages/worksheet/Index.svelte";
  import ErrorNotFound from "./pages/ErrorNotFound.svelte";
  import RenderPage from "./pages/render/Index.svelte";

  const Routes = {
    WorksheetPage: "worksheet-page",
    RenderPage: "render-page",
  };

  const DEFAULT_ROUTE = Routes.WorksheetPage;

  export let route = DEFAULT_ROUTE;
</script>

{#if route === Routes.RenderPage}
  <RenderPage />
{:else if route === Routes.WorksheetPage}
  <WorksheetPage />
{:else}
  <ErrorNotFound />
{/if}
