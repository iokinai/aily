import { TagsPost } from "@/models/tagsPost";
import { EndGameReason } from "@/models/ui/endGameReason";

interface RoundResultViewOptions {
  endGameReason: EndGameReason | null;
  correctGuesses: number;
  guessesToWin: number;
  tagArtwork: TagsPost | null;
  startNewRound: () => void;
}

export function RoundResultView({
  endGameReason,
  correctGuesses,
  guessesToWin,
  tagArtwork,
  startNewRound,
}: RoundResultViewOptions) {
  let textColorClass;
  let afterGameStatus;
  let afterGameComment;

  switch (endGameReason) {
    case "won":
      textColorClass = "text-cobalt";
      afterGameStatus = "Три из трех";
      afterGameComment = "Идеально";
      break;
    case "noGuessesLeft":
      textColorClass = "text-vermillion";
      afterGameStatus = "Попытки закончились";
      afterGameComment = `Вы угадали ${correctGuesses} из ${guessesToWin}.`;
      break;
    case "surrender":
      textColorClass = "text-vermillion";
      afterGameStatus = "Вы сдались";
      afterGameComment = `Вы угадали ${correctGuesses} из ${guessesToWin}.`;
      break;
  }

  return (
    <div className="mt-5 border-t border-ink/10 pt-5">
      <p
        className={`text-xs font-bold uppercase tracking-[0.18em] ${textColorClass}`}
      >
        {afterGameStatus}
      </p>
      <p className="mt-1 font-display text-2xl font-semibold tracking-[-0.03em] text-white">
        {afterGameComment}
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
