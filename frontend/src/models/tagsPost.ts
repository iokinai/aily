import { Tag } from "./tag";

export interface TagsPost {
  id: number;
  danbooru_id: number;
  path: string;
  is_nsfw: boolean;
  tags: Tag[];
}
