import DanbooruPost from "./danbooruPost";
import { HistoryObject } from "./historyObject";

export interface SessionData {
  prev: HistoryObject[];
  current: DanbooruPost;
}
