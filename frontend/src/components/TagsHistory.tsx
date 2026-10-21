import { EmptyHistory } from "./EmptyHistory";
import { TagsHistoryItem } from "./TagsHistoryItem";
import { SessionData } from "@/models/sessionData";

interface TagsHistoryOptions {
  session: SessionData;
  guessesToWin: number;
}

export function TagsHistory({ session, guessesToWin }: TagsHistoryOptions) {
  if (session.tags_prev.length === 0) return <EmptyHistory />;

  const endedRounds = session.tags_prev.filter(
    (o) => typeof o.state === "object" && "finished" in o.state,
  );

  const wins = endedRounds.filter(
    (entry) =>
      typeof entry.state === "object" &&
      "finished" in entry.state &&
      entry.state.finished == "win",
  ).length;

  return (
    <>
      <div className="flex items-center justify-between border-b border-ink/10 py-5">
        <p className="text-xs text-white">Rounds won</p>
        <p className="font-display text-2xl font-semibold tracking-[-0.04em] text-white">
          {wins}/{endedRounds.length}
        </p>
      </div>
      <div className="divide-y divide-ink/10">
        {[...endedRounds].reverse().map((entry) => (
          <TagsHistoryItem entry={entry} guessesToWin={guessesToWin} />
        ))}
      </div>
    </>
  );
}
