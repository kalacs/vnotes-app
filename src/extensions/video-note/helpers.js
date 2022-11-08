export function getSelectedMark(state) {
  const { ranges } = state.selection;
  let selectionRange = 0;
  const markRanges = [];
  ranges.forEach(({ $from, $to }) => {
    const from = $from.pos;
    const to = $to.pos;

    state.doc.nodesBetween(from, to, (node, pos) => {
      if (!node.isText && !node.marks.length) {
        return;
      }

      const relativeFrom = Math.max(from, pos);
      const relativeTo = Math.min(to, pos + node.nodeSize);
      const range = relativeTo - relativeFrom;

      selectionRange += range;

      markRanges.push(
        ...node.marks.map((mark) => ({
          mark,
          from: relativeFrom,
          to: relativeTo,
        }))
      );
    });
  });
  const [{ mark }] = markRanges;
  return mark;
}

export function removeFirstOccurenceInArray(array, prediction) {
  const position = array.findIndex(prediction);
  if (position === -1) return array;
  const array2 = [...array];
  delete array2[position];
  return [...array2.filter((id) => !!id)];
}
