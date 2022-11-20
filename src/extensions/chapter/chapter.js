import { mergeAttributes, Node } from "@tiptap/core";
import { REFERENCE_TYPES } from "../video-note-reference";

function formatSeconds(totalSeconds) {
  const [mainPart, fragmentPart] = (totalSeconds + "").split(".");
  const minutes = Math.floor(mainPart / 60);
  const seconds = mainPart % 60;

  return `${minutes < 10 ? `0${minutes}` : minutes}:${
    seconds < 10 ? `0${seconds}` : seconds
  }.${fragmentPart}`;
}

function createCardHeaderButton({ document, icon, type }) {
  const cardHeaderButton = document.createElement("button");
  const cardHeaderButtonIconContainer = document.createElement("span");
  const cardHeaderButtonIcon = document.createElement("i");

  cardHeaderButtonIcon.classList.add("mdi", `mdi-${icon}`);
  cardHeaderButtonIconContainer.classList.add("icon");
  cardHeaderButton.classList.add(
    "button",
    `is-${type}`,
    "is-rounded",
    "is-outlined"
  );

  cardHeaderButton.append(cardHeaderButtonIconContainer);
  cardHeaderButtonIconContainer.append(cardHeaderButtonIcon);

  return cardHeaderButton;
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
    return ({ node, getPos, editor }) => {
      const { view } = editor;
      const dom = document.createElement("div");

      dom.classList.add("card");

      const cardHeader = document.createElement("header");
      const cardHeaderTitle = document.createElement("p");
      const cardHeaderButtonVocabulary = createCardHeaderButton({
        document,
        icon: "book-open",
        type: "info",
      });
      const cardHeaderButtonPronunciation = createCardHeaderButton({
        document,
        icon: "account-voice",
        type: "danger",
      });
      const cardHeaderButtonReference = createCardHeaderButton({
        document,
        icon: "account-question",
        type: "primary",
      });

      cardHeaderButtonVocabulary.addEventListener("click", () => {
        if (typeof getPos === "function") {
          editor.commands.insertVideoNote({
            chapter: node,
            position: getPos() + 1,
            type: REFERENCE_TYPES.vocabulary,
          });
        }
      });

      cardHeaderButtonPronunciation.addEventListener("click", () => {
        if (typeof getPos === "function") {
          editor.commands.insertVideoNote({
            chapter: node,
            position: getPos() + 1,
            type: REFERENCE_TYPES.pronunciation,
          });
        }
      });

      cardHeaderButtonReference.addEventListener("click", () => {
        if (typeof getPos === "function") {
          editor.commands.insertVideoNote({
            chapter: node,
            position: getPos() + 1,
            type: REFERENCE_TYPES.references,
          });
        }
      });

      cardHeader.classList.add("card-header");
      cardHeaderTitle.classList.add("card-header-title");
      cardHeaderTitle.innerText =
        node.attrs.title +
        formatSeconds(node.attrs.start) +
        formatSeconds(node.attrs.end);
      cardHeader.append(
        cardHeaderTitle,
        cardHeaderButtonVocabulary,
        cardHeaderButtonPronunciation,
        cardHeaderButtonReference
      );

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
  addCommands() {
    return {
      insertVideoNote:
        ({
          chapter: {
            attrs: { start, end },
          },
          position,
          type,
        }) =>
        ({ commands }) => {
          const array = new Uint32Array(1);
          const [calculatedId] = window.crypto.getRandomValues(array);
          commands.insertContentAt(
            position,
            `<video-note type="${type}" start="${start}" end="${end}" references="" id="${calculatedId}"></video-note>`
          );
        },
    };
  },
});
