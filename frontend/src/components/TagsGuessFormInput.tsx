interface TagsGuessFormInputOptions {
  tagInput: string;
  setTagInput: React.Dispatch<React.SetStateAction<string>>;
  tagFinished: boolean;
}

export function TagsGuessFormInput({
  tagInput,
  setTagInput,
  tagFinished,
}: TagsGuessFormInputOptions) {
  return (
    <input
      type="text"
      value={tagInput}
      onChange={(event) => setTagInput(event.target.value)}
      disabled={tagFinished}
      aria-label="Введите тег"
      placeholder="Например: white_hair"
      autoComplete="off"
      className="min-w-0 flex-1 rounded-full border bg-ink/15 border-paper/60 px-5 py-3.5 text-sm text-white outline-none placeholder:text-white/30 focus:border-cobalt focus:ring-2 focus:ring-cobalt/15 disabled:cursor-default disabled:opacity-50"
    />
  );
}
