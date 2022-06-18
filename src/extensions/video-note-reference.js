import { mergeAttributes, Node, Mark } from "@tiptap/core";
import { Plugin, PluginKey } from "prosemirror-state";
export default Mark.create({
  name: "kalacs",
  priority: 1000,
  selectable: true,
  addProseMirrorPlugins() {
    const editor = this.editor;
    return [
      new Plugin({
        key: new PluginKey("eventHandler"),
        props: {
          handleClick(view, pos, event) {
            console.log({ view, pos, event });
            editor.commands.setNodeSelection(pos);
          },
        },
      }),
    ];
  },
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
      case "vocabulary":
        spanClass = "has-text-info";
        break;
      case "references":
        spanClass = "has-text-primary";
        break;
      case "pronunciation":
        spanClass = "has-text-danger";
        break;

      default:
        break;
    }
    return [
      "span",
      mergeAttributes(HTMLAttributes, {
        class: "has-text-weight-bold is-clickable " + spanClass,
        ["contenteditable"]: false,
      }),
      0,
    ];
  },
});
