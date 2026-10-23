import { PageLayout } from "@/components/PageLayout";
import { TagsList } from "@/components/TagsList";
import { Tag } from "@/models/tag";

interface TagsListPageOptions {
  tags: Tag[];
  nsfw: boolean;
  setNsfw: React.Dispatch<React.SetStateAction<boolean>>;
}

export function TagsListPage({ tags, nsfw, setNsfw }: TagsListPageOptions) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <TagsList tags={tags} />
    </PageLayout>
  );
}
