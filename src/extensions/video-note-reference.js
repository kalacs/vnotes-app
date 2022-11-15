import { mergeAttributes, Node, Mark } from "@tiptap/core";

export const REFERENCE_TYPES = {
  vocabulary: "vocabulary",
  references: "references",
  pronunciation: "pronunciation",
};

export default Mark.create({
  name: "videoNoteReference",
  priority: 1000,
  selectable: true,
  onCreate: () => {
    window.selectAnnotation = (element) => {
      const role = element.attributes.getNamedItem("role")?.value;

      if (role !== "annotation") {
        return true;
      }
      let range = new Range();

      range.setStart(element, 0);
      range.setEnd(element, 1);

      document.getSelection().removeAllRanges();
      document.getSelection().addRange(range);
    };
  },
  addAttributes() {
    return {
      type: {
        default: "",
      },
      sectionId: {
        default: 0,
      },
      selected: {
        default: false,
      },
      role: {
        default: "annotation",
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
        onclick: "selectAnnotation(this)",
      }),
      0,
    ];
  },
  addKeyboardShortcuts() {
    return {
      Backspace: () => true,
    };
  },
});
