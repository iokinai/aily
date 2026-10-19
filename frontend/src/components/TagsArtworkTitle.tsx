import { Artwork } from "@/models/artwork";

interface TagsArtworkTitleOptions {
  artwork: Artwork;
}

export function TagsArtworkTitle({ artwork }: TagsArtworkTitleOptions) {
  return (
    <div className="mb-4 flex items-end justify-between gap-4">
      <div>
        <p className="mb-1.5 text-[0.65rem] font-bold uppercase tracking-[0.2em] text-cobalt">
          Арт #{artwork.id}
        </p>
        <h1 className="font-display text-2xl font-medium tracking-[-0.03em] text-white sm:text-3xl">
          Какие теги подходят под это изображение?
        </h1>
      </div>
    </div>
  );
}
