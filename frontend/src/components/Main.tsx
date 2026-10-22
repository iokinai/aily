import { Link } from "react-router-dom";

export function Main() {
  return (
    <section className="flex flex-1 flex-col justify-center py-10 sm:py-14">
      <div className="mb-8 max-w-2xl sm:mb-10">
        <h1 className="font-display text-4xl font-semibold leading-[1.05] tracking-[-0.06em] text-white sm:text-6xl">
          Выбери режим
        </h1>
      </div>

      <div className="grid gap-4 md:grid-cols-2">
        <Link
          to="/aily"
          className="mode-card group relative min-h-80 overflow-hidden rounded-3xl bg-ink text-left text-white focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-cobalt"
        >
          <img
            src="/assets/img/aily_placeholder.jpeg"
            alt=""
            className="absolute inset-0 h-full w-full object-cover opacity-65 transition duration-500 group-hover:scale-[1.03] group-hover:opacity-75"
          />
          <span className="absolute inset-0 bg-linear-to-t from-black/90 via-black/20 to-black/10" />
          <span className="absolute inset-x-0 bottom-0 block p-6 sm:p-7">
            <span className="mb-3 block text-[0.65rem] font-bold uppercase tracking-[0.2em] text-white/65">
              Aily · Классический режим
            </span>
            <span className="font-display block text-3xl font-semibold tracking-[-0.04em]">
              Человек или ИИ?
            </span>
            <span className="mt-2 block max-w-sm text-sm leading-relaxed text-white/65">
              Угадай кто нарисовал арт
            </span>
          </span>
        </Link>

        <Link
          to="/tags"
          className="mode-card group relative min-h-80 overflow-hidden rounded-3xl bg-cobalt text-left text-white focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-cobalt"
        >
          <img
            src="/assets/img/tags_placeholder.jpeg"
            alt=""
            className="absolute inset-0 h-full w-full object-cover opacity-70 transition duration-500 group-hover:scale-[1.03] group-hover:opacity-80"
          />
          <span className="absolute inset-0 bg-linear-to-t from-cobalt-dark via-cobalt/20 to-transparent" />
          <span className="absolute inset-x-0 bottom-0 block p-6 sm:p-7">
            <span className="mb-3 block text-[0.65rem] font-bold uppercase tracking-[0.2em] text-white/70">
              Tags
            </span>
            <span className="font-display block text-3xl font-semibold tracking-[-0.04em]">
              Угадай теги
            </span>
            <span className="mt-2 block max-w-sm text-sm leading-relaxed text-white/70">
              Угадай 3 тега, которые есть у арта
            </span>
          </span>
        </Link>
      </div>
    </section>
  );
}
