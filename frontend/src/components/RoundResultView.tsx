import { TagsPost } from "@/models/tagsPost";

interface RoundResultViewOptions {
  tagWon: boolean | null;
  correctGuesses: number;
  guessesToWin: number;
  tagArtwork: TagsPost | null;
  startNewRound: () => void;
}

export function RoundResultView({
  tagWon,
  correctGuesses,
  guessesToWin,
  tagArtwork,
  startNewRound,
}: RoundResultViewOptions) {
  return (
    <div className="mt-5 border-t border-ink/10 pt-5">
      <p
        className={`text-xs font-bold uppercase tracking-[0.18em] ${tagWon ? "text-cobalt" : "text-vermillion"}`}
      >
        {tagWon ? "Три из трех" : "Попытки закончились"}
      </p>
      <p className="mt-1 font-display text-2xl font-semibold tracking-[-0.03em] text-white">
        {tagWon
          ? "Идеально"
          : `Вы угадали ${correctGuesses} из ${guessesToWin}.`}
      </p>
      <p className="mt-2 text-xs text-white/45">
        Tags: {tagArtwork?.tags.map((tag) => tag.tag_name).join(" · ")}
      </p>
      <button
        type="button"
        onClick={() => startNewRound()}
        className="mt-4 w-full bg-black rounded-full px-5 py-3.5 text-xs font-bold uppercase tracking-[0.12em] text-white transition hover:-translate-y-0.5 hover:bg-cobalt focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ink"
      >
        Далее
      </button>
    </div>
  );
}
