import { AilyHistoryItem } from "@/components/AilyHistoryItem";
import { EmptyHistory } from "@/components/EmptyHistory";
import { SessionData } from "@/models/sessionData";

export function AilyHistory({ session }: { session: SessionData | null }) {
  if (!session) {
    return null;
  }

  const score = session.prev.filter((p) => p.correct).length;
  const accuracy =
    session.prev.length > 0
      ? Math.round((score / session.prev.length) * 100)
      : 0;

  return (
    <section className="mx-auto w-full max-w-4xl flex flex-col flex-1 h-[calc(100vh-25rem)] py-8 px-4 sm:px-0">
      <div>
        {session.prev.length > 0 && (
          <div className="text-right">
            <p className="font-display text-3xl font-semibold tracking-tighter text-white">
              {accuracy}%
            </p>
            <p className="text-xs text-zinc-500">точность</p>
          </div>
        )}
      </div>

      {session.prev.length === 0 ? (
        <EmptyHistory mode="aily" />
      ) : (
        <div className="grow relative w-full mt-6">
          <div className="absolute inset-0 overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-zinc-800 scrollbar-track-transparent">
            <div className="divide-y divide-zinc-800/60">
              {[...session.prev].reverse().map((entry) => (
                <AilyHistoryItem key={entry.post.danbooru_id} entry={entry} />
              ))}
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
