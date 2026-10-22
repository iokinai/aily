import { TagsPost } from "@/models/tagsPost";
import { GuessObject } from "@/models/ui/guessObject";
import { TagsGuessFormHeader } from "./TagsGuessFormHeader";
import { TagsGuessFormButton } from "./TagsGuessFormButton";
import { GuessedTagIsland } from "./GuessedTagIsland";
import { RoundResultView } from "./RoundResultView";
import { EndGameReason } from "@/models/ui/endGameReason";
import { TagInput } from "./TagInput";

interface TagsGuessFormOptions {
  correctGuesses: number;
  guessesToWin: number;
  maxGuesses: number;
  guesses: GuessObject[];
  guessTag: () => void;
  tagInput: string;
  tagFinished: boolean;
  endGameReason: EndGameReason | null;
  setTagInput: React.Dispatch<React.SetStateAction<string>>;
  startNewRound: () => void;
  tagArtwork: TagsPost | null;
  surrender: () => void;
  tags: string[];
}

export function TagsGuessForm({
  correctGuesses,
  guessesToWin,
  maxGuesses,
  guesses,
  guessTag,
  tagInput,
  tagFinished,
  endGameReason,
  setTagInput,
  startNewRound,
  tagArtwork,
  surrender,
  tags,
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
        className="flex gap-2 flex-col"
      >
        {/*<TagsGuessFormInput
          tagInput={tagInput}
          setTagInput={setTagInput}
          tagFinished={tagFinished}
        />*/}

        <TagInput
          tags={tags}
          onSelectTag={(tag) => {
            setTagInput(tag);
          }}
          tagFinished={tagFinished}
        />

        <TagsGuessFormButton
          type="submit"
          disabled={!tagInput.trim() || tagFinished}
          className="bg-cobalt text-white hover:bg-cobalt-dark focus-visible:outline-cobalt"
        >
          Отправить
        </TagsGuessFormButton>
        <TagsGuessFormButton
          disabled={tagFinished}
          className="bg-red-500 text-white hover:bg-red-600 focus-visible:outline-red-600"
          onClick={(e) => {
            e.preventDefault();
            surrender();
          }}
        >
          Сдаться
        </TagsGuessFormButton>
      </form>

      {guesses.length > 0 && <GuessedTagIsland guesses={guesses} />}

      {tagFinished && (
        <RoundResultView
          endGameReason={endGameReason}
          correctGuesses={correctGuesses}
          guessesToWin={guessesToWin}
          tagArtwork={tagArtwork}
          startNewRound={startNewRound}
        />
      )}
    </div>
  );
}
