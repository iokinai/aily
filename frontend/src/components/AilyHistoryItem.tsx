import { Link } from "react-router-dom";
import { AilySession } from "@/models/ailySession";
import { ProxyImage } from "./ProxyImage";

export function AilyHistoryItem({ entry }: { entry: AilySession }) {
  return (
    <article className="grid grid-cols-[5rem_1fr_auto] items-center gap-4 py-4 sm:grid-cols-[7rem_1fr_auto] sm:gap-6 sm:py-5 border-b border-zinc-800/60">
      <ProxyImage
        className="aspect-square w-full rounded-xl object-cover bg-zinc-900 ring-1 ring-white/10"
        src={entry.post.path}
      />
      <div className="min-w-0">
        <h2 className="truncate font-display text-base font-semibold text-white sm:text-xl">
          {entry.post.is_ai_generated == entry.correct
            ? "Вы выбрали ИИ"
            : "Вы выбрали Человек"}
        </h2>
        <p className="mt-1 hidden text-xs text-zinc-400 sm:block">
          На самом деле{" "}
          {entry.post.is_ai_generated
            ? "сгенерировано ИИ"
            : "сделано человеком"}
        </p>
        <Link
          to={`https://danbooru.donmai.us/posts/${entry.post.danbooru_id}`}
          className="mt-1 hidden text-xs text-zinc-400 sm:block"
          target="_blank"
        >
          Открыть оригинал
        </Link>
      </div>
      <span
        className={`rounded-full px-3 py-1.5 text-[0.65rem] font-bold uppercase tracking-[0.12em] ${
          entry.correct
            ? "bg-blue-500/10 text-blue-400 border border-blue-500/20"
            : "bg-red-500/10 text-red-400 border border-red-500/20"
        }`}
      >
        {entry.correct ? "Верно" : "Неверно"}
      </span>
    </article>
  );
}
