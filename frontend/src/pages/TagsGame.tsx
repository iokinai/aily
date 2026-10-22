import { Api } from "@/api/api";
import { errorToStringRu, HttpErrorBody } from "@/api/apiError";
import { Artwork } from "@/models/artwork";
import { SessionData } from "@/models/sessionData";
import { TagsPost } from "@/models/tagsPost";
import { ProxyImage } from "@/components/ProxyImage";
import { toast } from "sonner";
import { GuessObject } from "@/models/ui/guessObject";
import { TagsGuessForm } from "@/components/TagsGuessForm";
import { TagsArtworkTitle } from "@/components/TagsArtworkTitle";
import { useState } from "react";
import { EndGameReason } from "@/models/ui/endGameReason";

const GUESSES_TO_WIN = 3;
const MAX_GUESSES = 5;

interface TagsGameProps {
  api: Api;
  session: SessionData;
  nsfw: boolean;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
  tags: string[];
}

export function TagsGame({
  api,
  session,
  nsfw,
  setSession,
  tags,
}: TagsGameProps) {
  let initCorrectGuesses = 0;
  let initGuesses = [] as { tag: string; correct: boolean }[];

  if (session.tags_prev.length !== 0) {
    const lastRound = session.tags_prev[session.tags_prev.length - 1];

    if (!lastRound !== null) {
      initCorrectGuesses = session.tags_prev[
        session.tags_prev.length - 1
      ].rounds.filter((g) => g.guessed_correctly).length;

      initGuesses = session.tags_prev[session.tags_prev.length - 1].rounds.map(
        (v) => {
          return {
            tag: v.tag,
            correct: v.guessed_correctly,
          };
        },
      );
    }
  }

  const [artwork, setArtwork] = useState<Artwork | null>(session.tags_current);
  const [correctGuesses, setCorrectGuesses] = useState(initCorrectGuesses);
  const [guesses, setGuesses] = useState<GuessObject[]>(initGuesses);
  const [tagInput, setTagInput] = useState("");
  const [tagFinished, setTagFinished] = useState(false);
  const [endGameReason, setEndGameReason] = useState<EndGameReason | null>(
    null,
  );
  const [nextArtwork, setNextArtwork] = useState<Artwork | null>(null);
  const [tagArtwork, setTagArtwork] = useState<TagsPost | null>(null);

  const loadArtwork = () => {
    api
      .loadTagsArtwork(nsfw)
      .then((artwork) => setArtwork(artwork))
      .catch((e) =>
        toast(errorToStringRu(e), {
          id: "get-current-tags-art",
        }),
      );
  };

  if (artwork === null) {
    loadArtwork();
    return;
  }

  const endRound = (won: boolean) => {
    api
      .artworkToTagsPost(artwork)
      .then((ta) => {
        setTagArtwork(ta);
        setEndGameReason(won ? "won" : "noGuessesLeft");
        setTagFinished(true);
      })
      .catch((e) => {
        toast.error(errorToStringRu(e), {
          id: "could-not-load-tags-artwork",
        });
      });
  };

  const startNewRound = (next: Artwork | null) => {
    if (next === null) {
      toast.error("Не удалось загрузить следующий арт", {
        id: "tags-could-not-load-next-artwork",
      });

      return;
    }

    setTagFinished(false);
    setEndGameReason(null);
    setArtwork(next);
    setCorrectGuesses(0);
    setGuesses([]);
    setNextArtwork(null);
  };

  const surrender = async () => {
    try {
      const [surrenderResult, session] = await api.tagsSurrender(nsfw);
      setTagArtwork(surrenderResult.post);
      setEndGameReason("surrender");
      setTagFinished(true);
      setSession(session);
      setNextArtwork(surrenderResult.next);
    } catch (e) {
      toast.error(errorToStringRu(e as HttpErrorBody), {
        id: "surrender-error",
      });
    }
  };

  const guessTag = async () => {
    const tag = tagInput;
    setTagInput("");

    if (artwork === null) {
      return;
    }

    try {
      const result = await api.makeTagsGuess({
        include_nsfw: nsfw,
        tag: tag,
      });

      setSession(result.session);

      setGuesses((p) => [
        ...p,
        { tag: tag, correct: result.result.is_current_guess_correct },
      ]);

      if (result.result.is_current_guess_correct) {
        setCorrectGuesses((p) => p + 1);
      }

      if (
        typeof result.result.ended === "object" &&
        "yes" in result.result.ended
      ) {
        const { won, next } = result.result.ended.yes;
        setNextArtwork(next);
        endRound(won);
        return;
      }
    } catch (e) {
      toast.error(errorToStringRu(e as HttpErrorBody), {
        id: "tags-guess-error",
      });
    }
  };

  return (
    <section className="flex flex-1 items-center py-8 sm:py-10">
      <div className="grid w-full gap-6 lg:grid-cols-[minmax(0,1.15fr)_minmax(20rem,0.85fr)] lg:items-center lg:gap-9">
        <div>
          <TagsArtworkTitle artwork={artwork} />

          <div className="tag-art-frame relative overflow-hidden bg-ink">
            <ProxyImage
              key={artwork.id}
              src={artwork.path}
              className="max-h-[60vh] w-full object-contain transition-opacity duration-300"
            />
          </div>
        </div>

        <TagsGuessForm
          correctGuesses={correctGuesses}
          guessesToWin={GUESSES_TO_WIN}
          maxGuesses={MAX_GUESSES}
          guesses={guesses}
          guessTag={guessTag}
          tagInput={tagInput}
          tagFinished={tagFinished}
          endGameReason={endGameReason}
          setTagInput={setTagInput}
          startNewRound={() => startNewRound(nextArtwork)}
          tagArtwork={tagArtwork}
          surrender={surrender}
          tags={tags}
        />
      </div>
    </section>
  );
}
