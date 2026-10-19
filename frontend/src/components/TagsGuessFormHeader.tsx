import { GuessObject } from "@/models/ui/guessObject";

interface TagsGuessFormHeaderOptions {
  correctGuesses: number;
  guessesToWin: number;
  maxGuesses: number;
  guesses: GuessObject[];
}

export function TagsGuessFormHeader({
  correctGuesses,
  guessesToWin,
  maxGuesses,
  guesses,
}: TagsGuessFormHeaderOptions) {
  return (
    <>
      <div className="flex items-center justify-between">
        <div>
          <p className="text-[0.65rem] font-bold uppercase tracking-[0.18em]">
            Прогресс
          </p>
          <p className="mt-1 font-display text-lg font-semibold text-white">
            {`${correctGuesses} из ${guessesToWin} верно`}
          </p>
        </div>
        <div
          className="flex gap-1.5"
          aria-label={`${guesses.length} отправлено из ${maxGuesses} попыток`}
        >
          {Array.from({ length: guesses.length }, (_, index) => (
            <span
              key={index}
              className={`h-2.5 w-2.5 rounded-full ${
                guesses[index].correct ? "bg-cobalt" : "bg-vermillion"
              }`}
            />
          ))}
        </div>
      </div>
      <p className="mb-3 text-xs leading-relaxed text-white/50 mt-1">
        Вводите 1 тег за попытку. У вас осталось {maxGuesses - guesses.length}{" "}
        попыток
      </p>
    </>
  );
}
