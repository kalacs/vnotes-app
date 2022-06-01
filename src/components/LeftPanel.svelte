<script>
  export let isOpened = false;
  export let selectedSection = "";
  export let vocabularyNote = null;
  export let pronunciationNote = null;
  export let referencesNote = null;

  const handleClick = (sectionName) => () => {
    if (sectionName === selectedSection) {
      isOpened = false;
      selectedSection = "";
    } else {
      selectedSection = sectionName;
      isOpened = true;
    }
  };
</script>

<div id="left-panel" class="columns is-gapless" class:opened={isOpened}>
  <div class="column is-one-fifth tab-contents">
    <article
      class="message is-info tab-content"
      class:is-active={selectedSection === "vocabulary"}
    >
      <div class="message-header">
        <p>Vocabulary</p>
      </div>
      <div class="message-body">
        {@html vocabularyNote ? vocabularyNote.payload.content : ""}
      </div>
    </article>
    <article
      class="message is-danger tab-content"
      class:is-active={selectedSection === "pronunciation"}
    >
      <div class="message-header">
        <p>Pronunciation</p>
      </div>
      <div class="message-body">
        {@html pronunciationNote ? pronunciationNote.payload.content : ""}
      </div>
    </article>
    <article
      class="message is-primary tab-content"
      class:is-active={selectedSection === "references"}
    >
      <div class="message-header">
        <p>References</p>
      </div>
      <div class="message-body">
        {@html referencesNote ? referencesNote.payload.content : ""}
      </div>
    </article>
  </div>
  <div class="column">
    <div class="field has-addons vertical-aligned">
      <p class="control">
        <button
          data-button-role="vocabulary"
          on:click={handleClick("vocabulary")}
          class="button is-info is-medium"
          class:is-hovered={selectedSection === "vocabulary"}
          disabled={vocabularyNote ? false : true}
        >
          <span class="icon is-large">
            <i class="mdi mdi-book-open-variant mdi-24px" />
          </span>
        </button>
      </p>
      <p class="control">
        <button
          data-button-role="pronunciation"
          on:click={handleClick("pronunciation")}
          class="button is-danger is-medium"
          class:is-hovered={selectedSection === "pronunciation"}
          disabled={pronunciationNote ? false : true}
        >
          <span class="icon is-large">
            <i class="mdi mdi-account-voice mdi-24px" />
          </span>
        </button>
      </p>
      <p class="control">
        <button
          data-button-role="references"
          on:click={handleClick("references")}
          class="button is-primary is-medium"
          class:is-hovered={selectedSection === "references"}
          disabled={referencesNote ? false : true}
        >
          <span class="icon is-large">
            <i class="mdi mdi-lightbulb-on-outline mdi-24px" />
          </span>
        </button>
      </p>
    </div>
  </div>
</div>

<style>
  #left-panel {
    position: absolute;
    left: -3em;
    transform: translateX(-20%);
    width: 100%;
  }
  #left-panel.opened {
    transform: translateX(0);
  }
  #left-panel .message-header {
    border-radius: 0;
  }
  #left-panel .message-body {
    border-bottom-left-radius: 0;
  }

  .tabs > ul {
    display: block;
    border: none;
  }
  .tabs a {
    display: inbuttonne;
  }
  article {
    display: none;
  }
  article.is-active {
    display: block;
  }
  .vertical-aligned.has-addons {
    flex-direction: column;
  }
  .vertical-aligned.has-addons .button {
    margin-bottom: 0;
  }
  .vertical-aligned.has-addons .control:first-child:not(:only-child) .button {
    border-bottom-right-radius: 0;
    border-bottom-left-radius: 0;
    border-top-right-radius: 4px;
    border-top-left-radius: 0;
  }
  .vertical-aligned.has-addons .control:last-child:not(:only-child) .button {
    border-bottom-right-radius: 4px;
    border-bottom-left-radius: 0;
    border-top-right-radius: 0;
    border-top-left-radius: 0;
  }
</style>
