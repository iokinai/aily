import { Artwork } from "./artwork";

export interface TagsRound {
  tag: string;
  guessed_correctly: boolean;
}

export type TagsLoseReason = "not_guessed" | "surrender";

export type TagsFinish = "win" | { lose: TagsLoseReason };

export type TagsState =
  | "going"
  | {
      finished: TagsFinish;
    };

export interface TagsSession {
  artwork: Artwork;
  rounds: TagsRound[];
  state: TagsState;
}
