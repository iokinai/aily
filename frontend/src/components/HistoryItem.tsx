import { Link } from "react-router-dom";
import { HistoryObject } from "../api/historyObject";
import { ProxyImage } from "./ProxyImage";

export function HistoryItem({ entry }: { entry: HistoryObject }) {
  return (
    <article className="grid grid-cols-[5rem_1fr_auto] items-center gap-4 py-4 sm:grid-cols-[7rem_1fr_auto] sm:gap-6 sm:py-5 border-b border-zinc-800/60">
      <ProxyImage
        className="aspect-square w-full rounded-xl object-cover bg-zinc-900 ring-1 ring-white/10"
        src={entry[0].path}
      />
      <div className="min-w-0">
        <h2 className="truncate font-display text-base font-semibold text-white sm:text-xl">
          {entry[0].is_ai_generated == entry[1]
            ? "Вы выбрали ИИ"
            : "Вы выбрали Человек"}
        </h2>
        <p className="mt-1 hidden text-xs text-zinc-400 sm:block">
          На самом деле{" "}
          {entry[0].is_ai_generated ? "сгенерировано ИИ" : "сделано человеком"}
        </p>
        <Link
          to={`https://danbooru.donmai.us/posts/${entry[0].danbooru_id}`}
          className="mt-1 hidden text-xs text-zinc-400 sm:block"
          target="_blank"
        >
          Открыть оригинал
        </Link>
      </div>
      <span
        className={`rounded-full px-3 py-1.5 text-[0.65rem] font-bold uppercase tracking-[0.12em] ${
          entry[1]
            ? "bg-blue-500/10 text-blue-400 border border-blue-500/20"
            : "bg-red-500/10 text-red-400 border border-red-500/20"
        }`}
      >
        {entry[1] ? "Верно" : "Неверно"}
      </span>
    </article>
  );
}
