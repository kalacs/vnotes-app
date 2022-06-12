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
  name: "extraInformation",
  group: "block",
  content: "inline*",

  addAttributes() {
    return {
      type: {
        default: "vocabulary",
      },
      start: {
        default: null,
      },
      end: {
        default: null,
      },
      ["note-id"]: {
        default: 0,
      },
    };
  },
  parseHTML() {
    return [
      {
        tag: "extra-information",
      },
    ];
  },

  renderHTML({ HTMLAttributes }) {
    return ["extra-information", mergeAttributes(HTMLAttributes)];
  },

  addNodeView() {
    return ({ editor, node, getPos }) => {
      const { view } = editor;

      const dom = document.createElement("article");
      const header = document.createElement("div");
      const headerContent = document.createElement("p");
      const body = document.createElement("div");

      dom.classList.add("extra-information");
      dom.classList.add("message");
      dom.classList.add("box");
      dom.style.padding = 0;

      const startSpan = document.createElement("span");
      const endSpan = document.createElement("span");
      const iconSpan = document.createElement("span");
      const icon = document.createElement("icon");
      const columnContainer = document.createElement("div");
      const content = document.createElement("p");
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
      startSpan.innerText = formatSeconds(node.attrs.start);
      endSpan.innerText = formatSeconds(node.attrs.end);
      columnContainer.append(startSpan, iconSpan, endSpan);

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

      return {
        dom,
        contentDOM: content,
      };
    };
  },
});
