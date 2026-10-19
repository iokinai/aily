import { SessionData } from "@/models/sessionData";
import { Api } from "@/api/api";
import { ArtworkCard } from "@/components/ArtworkCard";
import { errorToStringRu, HttpErrorBody } from "@/api/apiError";
import { toast } from "sonner";
import { useState } from "react";
import { Artwork } from "../models/artwork";

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
  const [currentPost, setCurrentPost] = useState<Artwork | null>(
    session.current,
  );
  const [nextPost, setNextPost] = useState<Artwork | null>(null);
  const [answer, setAnswer] = useState<boolean | null>(null);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);
  const [isAiGenerated, setIsAiGenerated] = useState<boolean | null>(null);

  const loadArtwork = () => {
    api
      .loadAilyArtwork(nsfw)
      .then((art) => setCurrentPost(art))
      .catch((e) =>
        toast(errorToStringRu(e), {
          id: "get-current-aily-art",
        }),
      );
  };

  if (currentPost === null) {
    loadArtwork();
    return;
  }

  const handleGuess = async (aiGuess: boolean) => {
    if (answer !== null || !currentPost) return;

    try {
      const currentArtworkAsPost = await api.artworkToPost(currentPost);
      setIsAiGenerated(currentArtworkAsPost.is_ai_generated);

      const result = await api.makeGuess({
        include_nsfw: nsfw,
        is_ai_generated: aiGuess,
      });
      setAnswer(aiGuess);
      setIsCorrect(result.guessed);
      setNextPost(result.post);
      setSession(result.session);
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
        isAiGenerated={isAiGenerated}
      />
    </div>
  );
}
