import { Extension } from "@tiptap/core";
import Paragraph from "@tiptap/extension-paragraph";

function formatSeconds(totalSeconds) {
  const [mainPart, fragmentPart] = (totalSeconds + "").split(".");
  const minutes = Math.floor(mainPart / 60);
  const seconds = mainPart % 60;

  function padTo2Digits(num) {
    return num.toString().padStart(2, "0");
  }
  return `${minutes < 10 ? `0${minutes}` : minutes}:${
    seconds < 10 ? `0${seconds}` : seconds
  }.${fragmentPart}`;
}

export const HandleVideoNotes = Paragraph.extend({
  name: "handleVideoNotes",

  addAttributes() {
    return {
      class: {
        default: "box",
      },
      ["data-start"]: {
        default: null,
      },
      ["data-end"]: {
        default: null,
      },
    };
  },
  parseHTML() {
    return [
      {
        tag: "p",
        getAttrs: (element) => {
          element.getAttribute("data-start");
          element.getAttribute("data-end");
        },
      },
    ];
  },
  renderHTML({ node, HTMLAttributes }) {
    console.log(node, HTMLAttributes);
    return [
      "div",
      HTMLAttributes,
      [
        "span",
        { class: "start-time tag is-dark" },
        HTMLAttributes["data-start"]
          ? formatSeconds(HTMLAttributes["data-start"])
          : "",
      ],
      [
        "span",
        { class: "start-time tag is-dark" },
        HTMLAttributes["data-end"]
          ? formatSeconds(HTMLAttributes["data-end"])
          : "",
      ],
      ["p", 0],
    ];
  },
  addCommands() {
    return {
      insertArticle:
        () =>
        ({ tr, dispatch, editor }) => {
          console.log(tr, dispatch, editor);

          if (dispatch) {
          }

          return true;
        },
    };
  },
});
