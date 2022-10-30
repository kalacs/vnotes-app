import { Extension } from "@tiptap/core";

import { BubbleMenuPlugin } from "./bubble-menu-plugin";

export const BubbleMenu = Extension.create({
  name: "bubbleMenu",

  addOptions() {
    return {
      element: null,
      tippyOptions: {},
      pluginKey: "bubbleMenu",
      shouldShow: null,
    };
  },

  addProseMirrorPlugins() {
    if (!this.options.element) {
      return [];
    }

    return [
      BubbleMenuPlugin({
        pluginKey: this.options.pluginKey,
        editor: this.editor,
        element: this.options.element,
        tippyOptions: this.options.tippyOptions,
        shouldShow: this.options.shouldShow,
      }),
    ];
  },
});
