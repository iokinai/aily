import { Link } from "react-router-dom";

export function Header() {
  return (
    <div className="mx-auto w-full max-w-5xl px-4 sm:px-6">
      <header className="flex items-center justify-between border-b border-zinc-800 pb-5">
        <div>
          <Link
            to="/"
            className="mr-15 font-display text-[2rem] font-semibold leading-none tracking-[-0.08em] text-white"
          >
            Aily<span className="text-blue-500">.</span>
          </Link>

          <Link
            to="/aily"
            className="mr-5 text-xs font-semibold uppercase tracking-[0.14em] text-zinc-400 hover:text-white transition-colors"
          >
            Классический
          </Link>

          <Link
            to="/tags"
            className="mr-5 text-xs font-semibold uppercase tracking-[0.14em] text-zinc-400 hover:text-white transition-colors"
          >
            Теги
          </Link>

          <Link
            to="/tag-list"
            className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-400 hover:text-white transition-colors"
          >
            Список тегов
          </Link>
        </div>

        <Link
          to="/history"
          className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-400 hover:text-white transition-colors"
        >
          История
        </Link>
      </header>
    </div>
  );
}
