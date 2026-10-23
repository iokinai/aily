import { GuessObject } from "@/models/ui/guessObject";

interface GuessedTagIslandOptions {
  guesses: GuessObject[];
}

export function GuessedTagIsland({ guesses }: GuessedTagIslandOptions) {
  return (
    <div className="mt-4 flex flex-wrap gap-2" aria-label="Ваши попытки">
      {guesses.map(({ tag, correct }) => (
        <span
          key={tag}
          className={`rounded-full px-3 py-2 text-[0.65rem] font-bold uppercase tracking-[0.08em] text-white ${
            correct ? "bg-cobalt" : "bg-vermillion"
          }`}
        >
          {tag} · {correct ? "Верно" : "Неверно"}
        </span>
      ))}
    </div>
  );
}
