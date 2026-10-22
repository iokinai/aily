import { EmptyHistory } from "./EmptyHistory";
import { TagsHistoryItem } from "./TagsHistoryItem";
import { SessionData } from "@/models/sessionData";

interface TagsHistoryOptions {
  session: SessionData;
  guessesToWin: number;
}

export function TagsHistory({ session, guessesToWin }: TagsHistoryOptions) {
  const endedRounds = session.tags_prev.filter(
    (o) => typeof o.state === "object" && "finished" in o.state,
  );

  const wins = endedRounds.filter(
    (entry) =>
      typeof entry.state === "object" &&
      "finished" in entry.state &&
      entry.state.finished.finish == "win",
  ).length;

  return (
    <section className="mx-auto w-full max-w-4xl flex flex-col flex-1 h-[calc(100vh-25rem)] py-8 px-4 sm:px-0">
      {session.tags_prev.length !== 0 && (
        <div>
          <p className="text-xs text-white">Выиграно</p>
          <p className="font-display text-2xl font-semibold tracking-[-0.04em] text-white">
            {wins}/{endedRounds.length}
          </p>
        </div>
      )}

      {session.tags_prev.length === 0 ? (
        <EmptyHistory mode="tags" />
      ) : (
        <div className="grow relative w-full mt-6">
          <div className="absolute inset-0 overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-zinc-800 scrollbar-track-transparent">
            <div className="divide-y divide-zinc-800/60">
              {[...endedRounds].reverse().map((entry) => (
                <TagsHistoryItem
                  key={entry.artwork.id}
                  entry={entry}
                  guessesToWin={guessesToWin}
                />
              ))}
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
