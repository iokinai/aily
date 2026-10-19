import { HistoryObject as AilyHistoryObject } from "./ailyHistoryObject";
import { TagsHistoryObject } from "./tagsHistoryObject";
import { Artwork } from "./artwork";

export interface SessionData {
  prev: AilyHistoryObject[];
  tags_prev: TagsHistoryObject[];
  current: Artwork | null;
  tags_current: Artwork | null;
}
