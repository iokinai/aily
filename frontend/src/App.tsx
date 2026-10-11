import { useState, useEffect, useRef } from "react";
import { Api } from "./api/api";
import DanbooruPost from "./api/danbooruPost";
import { Header } from "./components/Header";
import { Footer } from "./components/Footer";
import { ArtworkCard } from "./components/ArtworkCard";

const api = new Api(window.APP_PATH);

export default function App() {
  const [currentPost, setCurrentPost] = useState<DanbooruPost | null>(null);
  const [nextPost, setNextPost] = useState<DanbooruPost | null>(null);
  const [answer, setAnswer] = useState<boolean | null>(null);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);
  const [score, setScore] = useState(0);
  const [total, setTotal] = useState(0);
  const [nsfw, setNsfw] = useState(false);
  const [loading, setLoading] = useState(true);

  const initialized = useRef(false);

  useEffect(() => {
    if (initialized.current) return;
    initialized.current = true;

    setLoading(true);
    api
      .getSession()
      .then((session) => {
        setCurrentPost(session.current);

        const prevList = session.prev || [];
        const restoredTotal = prevList.length;
        const restoredScore = prevList.filter(([_, guessed]) => guessed).length;

        setTotal(restoredTotal);
        setScore(restoredScore);

        setLoading(false);
      })
      .catch(() => {
        api.start(nsfw).then((post) => {
          setCurrentPost(post);
          setScore(0);
          setTotal(0);
          setLoading(false);
        });
      });
  }, []);

  const handleGuess = async (aiGuess: boolean) => {
    if (answer !== null || !currentPost) return;

    try {
      const result = await api.guess(nsfw, aiGuess);
      setAnswer(aiGuess);
      setIsCorrect(result.guessed);
      if (result.guessed) {
        setScore((s) => s + 1);
      }
      setTotal((t) => t + 1);
      setNextPost(result.post);
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
    <main className="min-h-screen px-5 py-6 sm:px-8 sm:py-8 bg-zinc-950 text-zinc-100 flex flex-col justify-between">
      <div className="mx-auto flex w-full max-w-5xl flex-col flex-1 justify-between">
        <Header score={score} total={total} />

        <ArtworkCard
          post={currentPost}
          answer={answer}
          isCorrect={isCorrect}
          onGuess={handleGuess}
          onNext={nextArtwork}
        />

        <Footer nsfw={nsfw} setNsfw={setNsfw} />
      </div>
    </main>
  );
}
