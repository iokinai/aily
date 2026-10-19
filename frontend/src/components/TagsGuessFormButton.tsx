interface TagsGuessFormButtonOptions {
  tagInput: string;
  tagFinished: boolean;
}

export function TagsGuessFormButton({
  tagInput,
  tagFinished,
}: TagsGuessFormButtonOptions) {
  return (
    <button
      type="submit"
      disabled={!tagInput.trim() || tagFinished}
      className="shrink-0 rounded-full bg-cobalt px-5 py-3 text-xs font-bold uppercase tracking-widest text-white transition hover:-translate-y-0.5 hover:bg-cobalt-dark focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cobalt disabled:cursor-default disabled:opacity-35 disabled:hover:translate-y-0"
    >
      Отправить
    </button>
  );
}
