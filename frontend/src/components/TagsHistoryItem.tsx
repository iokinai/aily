import { TagsSession } from "@/models/tagsSession";
import { ProxyImage } from "./ProxyImage";
import { ResultBadge } from "./ResultBadge";

interface TagsHistoryItemOptions {
  entry: TagsSession;
  guessesToWin: number;
}

export function TagsHistoryItem({
  entry,
  guessesToWin,
}: TagsHistoryItemOptions) {
  return (
    <article
      key={entry.artwork.path}
      className="grid grid-cols-[5rem_1fr_auto] items-center gap-4 py-4 sm:grid-cols-[7rem_1fr_auto] sm:gap-6 sm:py-5"
    >
      <ProxyImage
        src={entry.artwork.path}
        className="aspect-square w-full rounded-xl object-cover"
      />
      <div className="min-w-0">
        <h2 className="font-display text-base font-semibold text-white sm:text-xl">
          Угадано {entry.rounds.filter((e) => e.guessed_correctly).length} из{" "}
          {guessesToWin} тегов
        </h2>
        {entry.rounds.length > 0 && (
          <p className="mt-1 hidden truncate text-xs text-white sm:block">
            Угадано: {entry.rounds.map((v) => v.tag).join(" · ")}
          </p>
        )}
      </div>
      <ResultBadge
        success={
          typeof entry.state === "object" &&
          "finished" in entry.state &&
          entry.state.finished === "win"
        }
      />
    </article>
  );
}
