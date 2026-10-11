import { Link } from "react-router-dom";

export function EmptyHistory() {
  return (
    <div className="flex min-h-96 flex-col items-center justify-center text-center">
      <div className="mb-5 flex h-16 w-16 items-center justify-center rounded-full border border-white/10 bg-zinc-900 font-display text-2xl text-zinc-400">
        0
      </div>
      <h2 className="font-display text-2xl font-semibold tracking-[-0.03em] text-white">
        Пока нет выборов.
      </h2>
      <p className="mt-2 max-w-sm text-sm leading-relaxed text-zinc-400">
        Сыграйте и ваш ответ появится тут.
      </p>
      <Link
        to="/"
        className="mt-6 rounded-full bg-blue-600 px-6 py-3 text-xs font-bold uppercase tracking-[0.12em] text-white transition hover:-translate-y-0.5 hover:bg-blue-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
      >
        Начать игру
      </Link>
    </div>
  );
}
