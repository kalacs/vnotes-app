import { mergeAttributes, Node } from "@tiptap/core";

export default Node.create({
  name: "videoNote",
  group: "block",
  content: "inline*",

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
      const { view } = editor;
      console.log("NOTE", node);
      const dom = document.createElement("div");

      dom.classList.add("video-note");
      dom.classList.add("box");

      const startSpan = document.createElement("span");
      const endSpan = document.createElement("span");
      const content = document.createElement("p");
      content.contentEditable = true;
      content.classList.add("content");
      startSpan.innerText = node.attrs.start;
      endSpan.innerText = node.attrs.end;
      dom.append(startSpan, endSpan, content);

      return {
        dom,
        contentDOM: content,
      };
    };
  },
});
