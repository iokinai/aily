import { SessionData } from "@/models/sessionData";
import { PageLayout } from "@/components/PageLayout";
import { AilyHistory } from "../components/AilyHistory";
import { useState } from "react";
import { TagsHistory } from "@/components/TagsHistory";
import { GameMode } from "@/models/ui/gameMode";

interface HistoryProps {
  nsfw: boolean;
  setNsfw: (v: boolean) => void;
  session: SessionData;
  guessesToWin: number;
}

export function HistoryPage({
  nsfw,
  setNsfw,
  session,
  guessesToWin,
}: HistoryProps) {
  const [view, setView] = useState<GameMode>("aily");

  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <section className="mx-auto w-full max-w-4xl flex-1 py-10 sm:py-14">
        <div className="flex flex-col gap-6 border-b border-ink/10 pb-6 sm:flex-row sm:items-end sm:justify-between">
          <div>
            <h1 className="font-display text-4xl font-semibold tracking-tighter text-white sm:text-5xl">
              История
            </h1>
          </div>
          <div className="flex rounded-full bg-ink/5 p-1">
            <button
              type="button"
              onClick={() => setView("aily")}
              aria-pressed={view === "aily"}
              className={`rounded-full px-4 py-2 text-xs font-bold uppercase tracking-widest transition ${
                view === "aily"
                  ? "bg-white text-ink"
                  : "text-white hover:text-ink hover:bg-white"
              }`}
            >
              Aily
            </button>
            <button
              type="button"
              onClick={() => setView("tags")}
              aria-pressed={view === "tags"}
              className={`rounded-full px-4 py-2 text-xs font-bold uppercase tracking-widest transition ${
                view === "tags"
                  ? "bg-cobalt text-white"
                  : "text-white hover:text-white hover:bg-cobalt"
              }`}
            >
              Tags
            </button>
          </div>
        </div>

        {view === "aily" ? (
          <AilyHistory session={session} />
        ) : (
          <TagsHistory session={session} guessesToWin={guessesToWin} />
        )}
      </section>
    </PageLayout>
  );
}
