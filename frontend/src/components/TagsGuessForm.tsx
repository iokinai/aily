import { TagsPost } from "@/models/tagsPost";
import { GuessObject } from "@/models/ui/guessObject";
import { TagsGuessFormHeader } from "./TagsGuessFormHeader";
import { TagsGuessFormInput } from "./TagsGuessFormInput";
import { TagsGuessFormButton } from "./TagsGuessFormButton";
import { GuessedTagIsland } from "./GuessedTagIsland";
import { RoundResultView } from "./RoundResultView";

interface TagsGuessFormOptions {
  correctGuesses: number;
  guessesToWin: number;
  maxGuesses: number;
  guesses: GuessObject[];
  guessTag: () => void;
  tagInput: string;
  tagFinished: boolean;
  tagWon: boolean | null;
  setTagInput: React.Dispatch<React.SetStateAction<string>>;
  startNewRound: () => void;
  tagArtwork: TagsPost | null;
}

export function TagsGuessForm({
  correctGuesses,
  guessesToWin,
  maxGuesses,
  guesses,
  guessTag,
  tagInput,
  tagFinished,
  tagWon,
  setTagInput,
  startNewRound,
  tagArtwork,
}: TagsGuessFormOptions) {
  return (
    <div className="rounded-3xl border border-ink/10 bg-zinc-900/80 p-5 sm:p-6">
      <TagsGuessFormHeader
        correctGuesses={correctGuesses}
        guessesToWin={guessesToWin}
        maxGuesses={maxGuesses}
        guesses={guesses}
      />
      <form
        onSubmit={(event) => {
          event.preventDefault();
          guessTag();
        }}
        className="flex gap-2"
      >
        <TagsGuessFormInput
          tagInput={tagInput}
          setTagInput={setTagInput}
          tagFinished={tagFinished}
        />

        <TagsGuessFormButton tagInput={tagInput} tagFinished={tagFinished} />
      </form>

      {guesses.length > 0 && <GuessedTagIsland guesses={guesses} />}

      {tagFinished && (
        <RoundResultView
          tagWon={tagWon}
          correctGuesses={correctGuesses}
          guessesToWin={guessesToWin}
          tagArtwork={tagArtwork}
          startNewRound={startNewRound}
        />
      )}
    </div>
  );
}
