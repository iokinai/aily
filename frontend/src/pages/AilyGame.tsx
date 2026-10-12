import { SessionData } from "../api/sessionData";
import DanbooruPost from "../api/danbooruPost";
import { Api } from "../api/api";
import { ArtworkCard } from "../components/ArtworkCard";
import { errorToStringRu, HttpErrorBody } from "../api/apiError";
import { toast } from "sonner";
import { useState } from "react";

export function AilyGame({
  session,
  api,
  nsfw,
  setSession,
}: {
  session: SessionData;
  api: Api;
  nsfw: boolean;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
}) {
  const [currentPost, setCurrentPost] = useState<DanbooruPost>(session.current);
  const [nextPost, setNextPost] = useState<DanbooruPost | null>(null);
  const [answer, setAnswer] = useState<boolean | null>(null);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);

  const handleGuess = async (aiGuess: boolean) => {
    if (answer !== null || !currentPost) return;

    try {
      const result = await api.makeGuess({
        include_nsfw: nsfw,
        is_ai_generated: aiGuess,
      });
      setAnswer(aiGuess);
      setIsCorrect(result.guessed);
      setNextPost(result.post);
      setSession((s) => {
        if (s === null) {
          return s;
        }

        return {
          prev: [...s.prev, [currentPost, result.guessed]],
          current: result.post,
        };
      });
    } catch (err) {
      toast.error(errorToStringRu(err as HttpErrorBody), {
        id: "make-guess-error",
      });
    }
  };

  const nextArtwork = () => {
    if (nextPost) {
      setCurrentPost(nextPost);
      setNextPost(null);
    }
    setAnswer(null);
    setIsCorrect(null);
  };

  return (
    <div>
      <ArtworkCard
        post={currentPost}
        answer={answer}
        isCorrect={isCorrect}
        onGuess={handleGuess}
        onNext={nextArtwork}
      />
    </div>
  );
}
