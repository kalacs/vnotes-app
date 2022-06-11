import { mergeAttributes, Node } from "@tiptap/core";

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
      headerContent.innerHTML = node.attrs.type;
      header.classList.add("message-header");
      header.appendChild(headerContent);

      body.classList.add("message-body");

      dom.append(header, body);

      return {
        dom,
        contentDOM: body,
      };
    };
  },
});
