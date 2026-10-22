import { TagsSession, TagsState } from "@/models/tagsSession";
import { ProxyImage } from "./ProxyImage";
import { ResultBadge } from "./ResultBadge";
import { Link } from "react-router-dom";

interface TagsHistoryItemOptions {
  entry: TagsSession;
  guessesToWin: number;
}

export function TagsHistoryItem({
  entry,
  guessesToWin,
}: TagsHistoryItemOptions) {
  const isWin = (state: TagsState) => {
    return (
      typeof state === "object" &&
      "finished" in state &&
      state.finished.finish === "win"
    );
  };

  const isSurrender = (state: TagsState) => {
    return (
      typeof state === "object" &&
      "finished" in state &&
      typeof state.finished === "object" &&
      typeof state.finished.finish === "object" &&
      state.finished.finish.lose == "surrender"
    );
  };

  const loadTagsFromResult = (state: TagsState) => {
    if (
      typeof state !== "object" ||
      !("finished" in state) ||
      typeof state.finished !== "object"
    ) {
      return [];
    }

    return state.finished.post.tags.map((tag) => tag.tag_name);
  };

  return (
    <article
      key={entry.artwork.path}
      className="grid grid-cols-[5rem_1fr_auto] items-center gap-4 py-4 sm:grid-cols-[7rem_1fr_auto] sm:gap-6 sm:py-5"
    >
      <ProxyImage
        src={entry.artwork.path}
        className="aspect-square w-full rounded-xl object-cover"
      />
      <div className="min-w-0">
        <h2 className="font-display text-base font-semibold text-white sm:text-xl">
          Угадано {entry.rounds.filter((e) => e.guessed_correctly).length} из{" "}
          {guessesToWin} тегов
        </h2>
        {entry.rounds.length > 0 && (
          <p className="mt-1 hidden truncate text-xs text-white sm:block">
            Угадано: {entry.rounds.map((v) => v.tag).join(" · ")}
          </p>
        )}
        {isSurrender(entry.state) && (
          <p className="mt-1 hidden truncate text-xs text-red-400 sm:block">
            Вы сдались
          </p>
        )}
        <p className="mt-1 text-xs text-white">
          Теги: {loadTagsFromResult(entry.state).join(" · ")}
        </p>
        <Link
          to={`https://danbooru.donmai.us/posts/${entry.artwork.danbooru_id}`}
          className="mt-1 hidden text-xs text-zinc-400 sm:block"
          target="_blank"
        >
          Открыть оригинал
        </Link>
      </div>
      <ResultBadge success={isWin(entry.state)} />
    </article>
  );
}
