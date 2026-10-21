import { AilySession } from "./ailySession";
import { TagsSession } from "./tagsSession";
import { Artwork } from "./artwork";

export interface SessionData {
  prev: AilySession[];
  tags_prev: TagsSession[];
  current: Artwork | null;
  tags_current: Artwork | null;
}
