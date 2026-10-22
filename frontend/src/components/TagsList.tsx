import { Tag } from "@/models/tag";

interface TagsListOptions {
  tags: Tag[];
}

export function TagsList({ tags }: TagsListOptions) {
  return (
    <section className="mx-auto w-full max-w-4xl flex-1 py-10 sm:py-14">
      <h1 className="font-display text-4xl font-semibold tracking-tighter text-white sm:text-5xl">
        Полный список тегов
      </h1>
      <ul className="list-none mt-5 columns-4 gap-x-8 pl-5">
        {tags.map((v) => {
          return (
            <li className="break-inside-avoid-column" key={v.id}>
              {v.tag_name}
            </li>
          );
        })}
      </ul>
    </section>
  );
}
