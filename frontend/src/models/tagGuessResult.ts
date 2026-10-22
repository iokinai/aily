import { Artwork } from "./artwork";

export type SessionEnded =
  | "no"
  | {
      yes: {
        won: boolean;
        next: Artwork;
      };
    };

export interface TagGuessResult {
  is_current_guess_correct: boolean;
  ended: SessionEnded;
}
