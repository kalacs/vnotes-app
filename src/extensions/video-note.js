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
  addKeyboardShortcuts() {
    return {
      // ↓ your new keyboard shortcut
      Enter: () => this.editor.commands.insertContent("<br />"),
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
      ["video-note-id"]: {
        default: 0,
      },
    };
  },
  parseHTML() {
    return [
      {
        tag: "video-note",
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return ["video-note", mergeAttributes(HTMLAttributes), 0];
  },
  addNodeView() {
    return ({ editor, node, getPos }) => {
      const dom = document.createElement("div");

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
      console.log(
        ["pronunciation", "references", "vocabulary"].indexOf(node.attrs.type) >
          -1
      );
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
});
