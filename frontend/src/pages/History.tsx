import { HistoryItem } from "../components/HistoryItem";
import { EmptyHistory } from "../components/EmptyHistory";
import { SessionData } from "../api/sessionData";

export function History({ session }: { session: SessionData | null }) {
  if (!session) {
    return null;
  }

  const score = session.prev.filter((p) => p[1]).length;
  const accuracy =
    session.prev.length > 0
      ? Math.round((score / session.prev.length) * 100)
      : 0;

  return (
    <section className="mx-auto w-full max-w-4xl flex flex-col flex-1 h-[calc(100vh-6rem)] py-8 px-4 sm:px-0">
      <div className="flex items-end justify-between gap-6 border-b border-zinc-800 pb-6 shrink-0">
        <div>
          <h1 className="font-display text-4xl font-semibold tracking-tighter text-white sm:text-5xl">
            История ответов
          </h1>
        </div>
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
        <EmptyHistory />
      ) : (
        <div className="grow relative w-full mt-6">
          <div className="absolute inset-0 overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-zinc-800 scrollbar-track-transparent">
            <div className="divide-y divide-zinc-800/60">
              {[...session.prev].reverse().map((entry) => (
                <HistoryItem key={entry[0].danbooru_id} entry={entry} />
              ))}
            </div>
          </div>
        </div>
      )}
    </section>
  );
}
