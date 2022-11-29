<script>
  import { createEventDispatcher } from "svelte";
  export let worksheet;
  let selectedProviders = [];

  const levels = [
    { id: "0", text: "Select option" },
    { id: "A1", text: "A1" },
    { id: "A2", text: "A2" },
    { id: "B1", text: "B1" },
    { id: "B2", text: "B2" },
    { id: "C1", text: "C1" },
  ];
  const providers = [
    { id: 1, name: "HBO Max" },
    { id: 2, name: "Disney+" },
    { id: 3, name: "Amazon Prime" },
    { id: 4, name: "Netflix" },
    { id: 5, name: "Yoututbe" },
  ];
  const dispatch = createEventDispatcher();

  function saveForm() {
    dispatch("save-worksheet-details", {
      ...worksheet,
      compatibility: selectedProviders,
    });
  }
</script>

<fieldset class="box">
  <div class="field">
    <label class="label">Title</label>
    <div class="control">
      <input
        bind:value={worksheet.title}
        class="input"
        type="text"
        placeholder="Text input"
      />
    </div>
  </div>

  <div class="field">
    <label class="label">Compatible with providers</label>
    {#each providers as provider}
      <div class="control">
        <label class="checkbox">
          <input
            bind:group={selectedProviders}
            type="checkbox"
            value={provider}
            checked={worksheet.compatibility.find(
              ({ id }) => id === provider.id
            )}
          />
          {provider.name}
        </label>
      </div>
    {/each}

    <div class="field">
      <label class="label">Level</label>
      <div class="control">
        <div class="select">
          <select bind:value={worksheet.variation}>
            {#each levels as level}
              <option value={level.id}>
                {level.text}
              </option>
            {/each}
          </select>
        </div>
      </div>
    </div>

    <div class="field is-grouped">
      <div class="control">
        <button on:click={saveForm} class="button is-link">Save</button>
      </div>
    </div>
  </div>
</fieldset>
