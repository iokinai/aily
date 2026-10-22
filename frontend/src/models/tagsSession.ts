import { Artwork } from "./artwork";
import { TagsPost } from "./tagsPost";

export type TagsState =
  | "going"
  | {
      finished: {
        post: TagsPost;
        finish: TagsFinish;
      };
    };

export interface TagsRound {
  tag: string;
  guessed_correctly: boolean;
}

export type TagsLoseReason = "not_guessed" | "surrender";

export type TagsFinish = "win" | { lose: TagsLoseReason };

export interface TagsSession {
  artwork: Artwork;
  rounds: TagsRound[];
  state: TagsState;
}
