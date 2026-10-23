import { Artwork } from "./artwork";
import { TagsPost } from "./tagsPost";

export interface SurrenderResult {
  post: TagsPost;
  next: Artwork;
}
