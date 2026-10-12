import { useState } from "react";
import DanbooruPost from "../api/danbooruPost";
import { ProxyImage } from "./ProxyImage";

interface ArtworkCardProps {
  post: DanbooruPost;
  answer: boolean | null;
  isCorrect: boolean | null;
  onGuess: (ai: boolean) => void;
  onNext: () => void;
}

export function ArtworkCard({
  post,
  answer,
  isCorrect,
  onGuess,
  onNext,
}: ArtworkCardProps) {
  const [canGuess, setCanGuess] = useState(false);
  const guess = (ai: boolean) => {
    setCanGuess(false);
    onGuess(ai);
  };

  return (
    <section className="mx-auto flex w-full max-w-3xl flex-1 flex-col justify-center py-8 sm:py-10">
      <div className="mb-4 flex items-end justify-between gap-4">
        <div>
          <p className="mb-1.5 text-[0.65rem] font-bold uppercase tracking-[0.2em] text-blue-500">
            Арт #{post.id}
          </p>
          <p className="font-display text-2xl font-medium tracking-[-0.03em] text-white sm:text-3xl">
            Человек или ИИ?
          </p>
        </div>
        <p className="hidden max-w-44 text-right text-xs leading-relaxed text-zinc-500 sm:block">
          Смотри внимательно...
        </p>
      </div>

      <div className="art-frame relative overflow-hidden bg-zinc-900 rounded-2xl flex items-center justify-center min-h-87.5 max-h-[60vh]">
        <ProxyImage
          key={post.id}
          className="max-h-[60vh] w-full object-contain transition-opacity duration-300"
          src={post.path}
          onLoadedWithoutError={() => setCanGuess(true)}
        />
        <div className="pointer-events-none absolute inset-0 ring-1 ring-inset ring-white/10 rounded-2xl" />

        {answer !== null && (
          <div className="absolute inset-x-0 bottom-0 flex items-end justify-between gap-4 bg-linear-to-t from-black/90 via-black/50 to-transparent px-5 pb-5 pt-20 text-white sm:px-7 sm:pb-7 z-20">
            <div>
              <p className="text-xs font-bold uppercase tracking-[0.18em] text-zinc-400">
                {isCorrect ? "Вы угадали" : "Неверно"}
              </p>
              <p className="mt-1 font-display text-2xl font-semibold">
                {post.is_ai_generated
                  ? "Сгенерировано ИИ."
                  : "Сделано человеком."}
              </p>
              <a
                href={`https://danbooru.donmai.us/posts/${post.danbooru_id}`}
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center px-3 py-1.5 bg-black/60 backdrop-blur-md border border-white/15 rounded-lg text-xs font-medium text-gray-200 hover:bg-black/80 hover:border-white/30 transition-all mt-2"
              >
                Открыть оригинал ↗
              </a>
            </div>
            <button
              type="button"
              onClick={onNext}
              className="shrink-0 rounded-full bg-white px-5 py-3 text-xs font-bold uppercase tracking-[0.12em] text-zinc-950 transition hover:-translate-y-0.5 hover:bg-zinc-200 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white"
            >
              Далее
            </button>
          </div>
        )}
      </div>

      <div className="mt-4 grid grid-cols-2 gap-3 sm:gap-4">
        <button
          type="button"
          onClick={() => guess(true)}
          disabled={answer !== null || !canGuess}
          className="group flex min-h-16 items-center justify-center rounded-full border-2 border-blue-600 bg-blue-600 px-4 text-center text-xs font-bold uppercase tracking-widest text-white transition hover:-translate-y-0.5 hover:bg-blue-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:cursor-default disabled:opacity-45 disabled:hover:translate-y-0 sm:text-sm"
        >
          Сгенерировано ИИ
        </button>
        <button
          type="button"
          onClick={() => guess(false)}
          disabled={answer !== null || !canGuess}
          className="flex min-h-16 items-center justify-center rounded-full border-2 border-zinc-700 bg-transparent px-4 text-center text-xs font-bold uppercase tracking-widest text-zinc-100 transition hover:-translate-y-0.5 hover:bg-zinc-800 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-zinc-500 disabled:cursor-default disabled:opacity-45 disabled:hover:translate-y-0 sm:text-sm"
        >
          Сделано человеком
        </button>
      </div>
    </section>
  );
}
