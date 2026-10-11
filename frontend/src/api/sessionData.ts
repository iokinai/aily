import DanbooruPost from "./danbooruPost";

export interface SessionData {
  prev: [DanbooruPost, boolean][];
  current: DanbooruPost;
}
