import { mergeAttributes, Node } from "@tiptap/core";

const capitalize = (s) => {
  if (typeof s !== "string") return "";
  return s.charAt(0).toUpperCase() + s.slice(1);
};

function formatSeconds(totalSeconds) {
  const [mainPart, fragmentPart] = (totalSeconds + "").split(".");
  const minutes = Math.floor(mainPart / 60);
  const seconds = mainPart % 60;

  return `${minutes < 10 ? `0${minutes}` : minutes}:${
    seconds < 10 ? `0${seconds}` : seconds
  }.${fragmentPart}`;
}
export default Node.create({
  name: "videoNote",
  group: "block",
  content: "inline*",
  marks: "videoNoteReference bold",

  addStorage() {
    return {
      sections: new Map(),
      pointers: new Map(),
      hasAnnotation: new Map(),
    };
  },
  addKeyboardShortcuts() {
    return {
      Enter: () => this.editor.commands.insertContent("<br />"),
      "Shift-Cmd-v": () => this.editor.commands.addReference("vocabulary"),
      "Shift-Cmd-r": () => this.editor.commands.addReference("references"),
      "Shift-Cmd-p": () => this.editor.commands.addReference("pronunciation"),
    };
  },
  addAttributes() {
    return {
      type: {
        default: "default",
      },
      start: {
        default: null,
      },
      end: {
        default: null,
      },
      id: {
        default: 0,
      },
      references: {
        default: null,
        parseHTML: (node) => {
          const references = node.attributes.getNamedItem("references");
          if (references) {
            return references.value
              .split(";")
              .reduce((map = new Map(), reference) => {
                const [rawId, phrase] = reference.split("::");
                const id = parseInt(rawId);
                if (id) {
                  map.set(id, phrase);
                }
                return map;
              }, new Map());
          }
          return null;
        },
      },
    };
  },
  parseHTML: () => {
    return [
      {
        tag: "video-note",
      },
    ];
  },

  renderHTML: ({ HTMLAttributes }) => {
    return ["video-note", mergeAttributes(HTMLAttributes), 0];
  },
  addNodeView() {
    return (tools) => {
      const dom = document.createElement("div");
      const { node } = tools;

      if (node.attrs.references) {
        const references = [...node.attrs.references].reduce(
          (map, [id, phrase]) => {
            if (id) {
              this.storage.hasAnnotation.set(id, node.attrs.id);
              this.storage.pointers.set(phrase, {
                sectionType: node.attrs.type,
                sectionId: node.attrs.id,
              });
              map.push({
                id,
                phrase,
                type: node.attrs.type,
                parentId: node.attrs.id,
              });
            }
            return map;
          },
          []
        );
        this.storage.sections.set(node.attrs.id, references);
      }

      if (this.storage.hasAnnotation.has(node.attrs.id)) {
        dom.dataset.hasAnnotation = true;
      }
      dom.dataset.id = node.attrs.id;
      dom.classList.add("video-note");
      dom.classList.add("box");

      const startSpan = document.createElement("span");
      const endSpan = document.createElement("span");
      const iconSpan = document.createElement("span");
      const icon = document.createElement("icon");
      const content = document.createElement("p");
      const columnContainer = document.createElement("div");
      columnContainer.classList.add("columns", "is-centered");

      iconSpan.classList.add("icon", "column", "is-1");
      startSpan.classList.add(
        "column",
        "is-3",
        "has-text-right",
        "has-text-weight-bold"
      );
      endSpan.classList.add("column", "is-3", "has-text-weight-bold");
      icon.classList.add("mdi", "mdi-arrow-right");
      iconSpan.append(icon);
      content.contentEditable = true;
      content.classList.add("content");
      startSpan.innerText = formatSeconds(node.attrs.start);
      endSpan.innerText = formatSeconds(node.attrs.end);
      columnContainer.append(startSpan, iconSpan, endSpan);

      if (
        ["pronunciation", "references", "vocabulary"].indexOf(node.attrs.type) >
        -1
      ) {
        const header = document.createElement("div");
        const headerContent = document.createElement("p");
        const body = document.createElement("div");

        dom.classList.add("message");
        dom.style.padding = 0;

        switch (node.attrs.type) {
          case "vocabulary":
            dom.classList.add("is-info");
            break;
          case "references":
            dom.classList.add("is-primary");
            break;
          case "pronunciation":
            dom.classList.add("is-danger");
            break;

          default:
            break;
        }
        headerContent.innerHTML = capitalize(node.attrs.type);
        header.classList.add("message-header");
        header.appendChild(headerContent);

        body.classList.add("message-body");
        body.append(columnContainer, content);
        dom.append(header, body);
      } else {
        dom.append(columnContainer, content);
      }

      return {
        dom,
        contentDOM: content,
      };
    };
  },
  addCommands() {
    return {
      addAnnotation:
        (type) =>
        ({ editor, commands }) => {
          const { state } = editor;
          const transaction = state.tr;
          const videoNoteNodePos =
            state.selection.$anchor.pos -
            state.selection.$anchor.parentOffset -
            1;
          const videoNoteNode = state.doc.nodeAt(videoNoteNodePos);
          const phrase = window.getSelection().toString();
          let relatedSection = null;
          let relatedSectionPosition = 0;
          let exitCondition = false;

          state.doc.descendants((node, pos) => {
            if (exitCondition) return false;
            if (node.attrs.type === type) {
              relatedSection = node;
              relatedSectionPosition = pos;
            }
            if (videoNoteNode.attrs.id === node.attrs.id) {
              exitCondition = true;
              return false;
            }
          });
          const endPosition =
            relatedSectionPosition + relatedSection.nodeSize - 1;
          // wrap the selected phrase
          commands.setMark("videoNoteReference", {
            type,
          });
          // insert phrase
          commands.insertContentAt(
            endPosition,
            `<br /><strong>${phrase}</strong>`,
            {
              updateSelection: true,
            }
          );
          commands.scrollIntoView();
          // update references attribute
          transaction.setNodeMarkup(relatedSectionPosition, undefined, {
            ...relatedSection.attrs,
          });
          editor.view.dispatch(transaction);
        },
      toggleAnnotation:
        (type) =>
        ({ editor, commands }) => {
          if (editor.isActive("videoNoteReference")) {
            commands.removeAnnotation(type);
          } else {
            commands.addAnnotation(type);
          }
          return true;
        },
      markReferences: () => () => {
        // TODO: fix annotation attributes
        document
          .querySelectorAll("[data-has-annotation] p")
          .forEach((element) => {
            for (const [phrase, pointer] of this.storage.pointers) {
              const { sectionType: type, sectionId } = pointer;
              const phraseStartIndex = element.innerHTML.indexOf(phrase);
              if (phraseStartIndex > -1) {
                const phraseEndIndex = phraseStartIndex + phrase.length;
                const phraseFound = element.innerHTML.substring(
                  phraseStartIndex,
                  phraseEndIndex
                );
                const replaceString = `<video-note-reference type="${type}" sectionId="${sectionId}">${phraseFound}</video-note-reference>`;
                element.innerHTML = element.innerHTML.replace(
                  phrase,
                  replaceString
                );
              }
            }
          });
      },
      removeAnnotation:
        (type) =>
        ({ editor, commands }) => {
          const { state } = editor;
          const videoNoteNodePos =
            state.selection.$anchor.pos -
            state.selection.$anchor.parentOffset -
            1;
          const videoNoteNode = state.doc.nodeAt(videoNoteNodePos);

          const phrase = window.getSelection().toString();
          const pointer = this.storage.pointers.get(phrase);
          const videoNoteId = videoNoteNode.attrs.id;
          const sectionId = this.storage.hasAnnotation.get(videoNoteId);
          // update pointers
          this.storage.pointers.delete(phrase);
          // update sections
          this.storage.sections.set(
            sectionId,
            this.storage.sections
              .get(sectionId)
              .filter(({ id }) => id !== videoNoteId)
          );
          // update hasAnnotation
          this.storage.hasAnnotation.delete(videoNoteId);

          commands.toggleMark("videoNoteReference", { type });
          window.getSelection().removeAllRanges();
          window.alert(
            `Don't forget to update "${pointer.sectionType}" section!`
          );
        },
    };
  },
});
