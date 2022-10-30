import { mergeAttributes, Node, Mark } from "@tiptap/core";
import { Plugin, PluginKey } from "prosemirror-state";
const REFERENCE_TYPES = {
  vocabulary: "vocabulary",
  references: "references",
  pronunciation: "pronunciation",
};

export default Mark.create({
  name: "videoNoteReference",
  priority: 1000,
  selectable: true,
  addAttributes() {
    return {
      type: {
        default: "",
      },
      id: {
        default: 0,
      },
      referenceId: {
        default: 0,
      },
      selected: {
        default: false,
      },
    };
  },
  parseHTML: () => {
    return [
      {
        tag: "video-note-reference",
      },
    ];
  },

  renderHTML: ({ HTMLAttributes }) => {
    let spanClass = "";
    switch (HTMLAttributes.type) {
      case REFERENCE_TYPES.vocabulary:
        spanClass = "is-info";
        break;
      case REFERENCE_TYPES.references:
        spanClass = "is-primary";
        break;
      case REFERENCE_TYPES.pronunciation:
        spanClass = "is-danger";
        break;

      default:
        break;
    }

    if (!HTMLAttributes.selected) {
      spanClass += " is-light";
    }

    return [
      "span",
      mergeAttributes(HTMLAttributes, {
        class:
          "tag is-medium is-rounded has-text-weight-bold is-clickable " +
          spanClass,
        ["contenteditable"]: true,
      }),
      0,
    ];
  },
  addKeyboardShortcuts() {
    return {
      Backspace: ({ editor }) => {
        return true;
      },
      "Shift-Backspace": ({ editor }) => {
        const pos = editor.state.selection.$anchor.pos;
        const selection = getSelection();
        const parentElement = selection.anchorNode.parentElement;
        const type = parentElement.attributes.getNamedItem("type")?.value;

        if (Object.values(REFERENCE_TYPES).includes(type)) {
          const from = pos - selection.anchorOffset;
          const to = pos;
          console.log("Reference", { pos, selection, from, to });
          editor.commands.setNodeSelection(from);
          const selectionOfVideoNoteReference = editor.state.selection;
          console.log(editor.state.selection);
          // TODO: remove reference and clean up
          editor.commands.updateAttributes("videoNoteReference", {
            selected: true,
          });
        }

        return true;
      },
    };
  },
});
