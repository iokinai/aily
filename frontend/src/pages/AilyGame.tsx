import { useEffect, useState } from "react";
import { SessionData } from "../api/sessionData";
import DanbooruPost from "../api/danbooruPost";
import { Api } from "../api/api";
import { ArtworkCard } from "../components/ArtworkCard";

export function AilyGame({
  loading,
  session,
  api,
  nsfw,
  setSession,
}: {
  loading: boolean;
  session: SessionData | null;
  api: Api;
  nsfw: boolean;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
}) {
  const [currentPost, setCurrentPost] = useState<DanbooruPost | null>(null);
  const [nextPost, setNextPost] = useState<DanbooruPost | null>(null);
  const [answer, setAnswer] = useState<boolean | null>(null);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);

  useEffect(() => {
    if (!session || currentPost !== null) {
      return;
    }

    setCurrentPost(session.current);
  }, [session, currentPost]);

  const handleGuess = async (aiGuess: boolean) => {
    if (answer !== null || !currentPost) return;

    try {
      const result = await api.guess(nsfw, aiGuess);
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
      console.error(err);
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

  if (loading || !currentPost) {
    return (
      <main className="min-h-screen flex items-center justify-center bg-zinc-950 text-white">
        <p className="text-xs font-bold uppercase tracking-[0.2em] animate-pulse text-blue-500">
          Загрузка сессии...
        </p>
      </main>
    );
  }

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
