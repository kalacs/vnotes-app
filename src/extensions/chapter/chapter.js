import { mergeAttributes, Node } from "@tiptap/core";

function formatSeconds(totalSeconds) {
  const [mainPart, fragmentPart] = (totalSeconds + "").split(".");
  const minutes = Math.floor(mainPart / 60);
  const seconds = mainPart % 60;

  return `${minutes < 10 ? `0${minutes}` : minutes}:${
    seconds < 10 ? `0${seconds}` : seconds
  }.${fragmentPart}`;
}
export const Chapter = Node.create({
  name: "chapter",
  group: "block",
  content: "videoNote*",
  marks: "videoNoteReference bold",
  addAttributes() {
    return {
      title: {
        default: "",
      },
      start: {
        default: null,
      },
      end: {
        default: null,
      },
    };
  },
  parseHTML: () => {
    return [
      {
        tag: "chapter",
      },
    ];
  },

  renderHTML: ({ HTMLAttributes }) => {
    return ["chapter", mergeAttributes(HTMLAttributes), 0];
  },
  addNodeView() {
    return ({ node }) => {
      /*
<div class="card">
  <header class="card-header">
    <p class="card-header-title">
      Component
    </p>
    <button class="card-header-icon" aria-label="more options">
      <span class="icon">
        <i class="fas fa-angle-down" aria-hidden="true"></i>
      </span>
    </button>
  </header>
  <div class="card-content">
    <div class="content">
    </div>
  </div>
  <footer class="card-footer">
    <a href="#" class="card-footer-item">Save</a>
    <a href="#" class="card-footer-item">Edit</a>
    <a href="#" class="card-footer-item">Delete</a>
  </footer>
</div>
*/

      const dom = document.createElement("div");

      dom.classList.add("card");

      const cardHeader = document.createElement("header");
      const cardHeaderTitle = document.createElement("p");
      cardHeader.classList.add("card-header");
      cardHeaderTitle.classList.add("card-header-title");
      cardHeaderTitle.innerText =
        node.attrs.title +
        formatSeconds(node.attrs.start) +
        formatSeconds(node.attrs.end);
      cardHeader.append(cardHeaderTitle);

      const cardContent = document.createElement("div");
      const cardContentContent = document.createElement("div");
      cardContent.classList.add("card-content");
      cardContentContent.classList.add("content");
      cardContent.append(cardContentContent);

      const cardFooter = document.createElement("footer");
      cardFooter.classList.add("card-footer");

      cardContent.contentEditable = true;
      dom.append(cardHeader, cardContent, cardFooter);

      return {
        dom,
        contentDOM: cardContent,
      };
    };
  },
});
