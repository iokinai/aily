import { useEffect, useState } from "react";
import { Api } from "./api/api";
import { SessionData } from "./models/sessionData";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AilyGamePage } from "./pages/AilyGamePage";
import { HistoryPage } from "./pages/HistoryPage";
import { Toaster } from "sonner";
import { toast } from "sonner";
import { errorToStringRu, HttpErrorBody } from "./api/apiError";
import { TagsGamePage } from "./pages/TagsGamePage";
import { TagsListPage } from "./pages/TagsListPage";
import { Tag } from "./models/tag";
import { MainPage } from "./pages/MainPage";

const api = new Api(`${import.meta.env.VITE_API_URL}/api`);

export default function App() {
  const [loading, setLoading] = useState(true);
  const [session, setSession] = useState<SessionData | null>(null);
  const [nsfw, setNsfw] = useState(false);
  const [tags, setTags] = useState<Tag[] | null>(null);

  useEffect(() => {
    const loadEverything = async () => {
      let session;

      try {
        session = await api.getSession();
      } catch {
        session = await api.startNewSession();
      }

      const tags = await api.getTagsList();

      return {
        session,
        tags,
      };
    };

    loadEverything()
      .then(({ session, tags }) => {
        setSession(session);
        setTags(tags);
        setLoading(false);
      })
      .catch((err) => {
        toast.error(errorToStringRu(err as HttpErrorBody), {
          id: "loading-error",
        });
        setLoading(false);
      });
  }, []);

  if (loading || session === null || tags === null) {
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
      <BrowserRouter>
        <Routes>
          <Route
            path="/"
            element={<MainPage nsfw={nsfw} setNsfw={setNsfw} />}
          />
          <Route
            path="/aily"
            element={
              <AilyGamePage
                session={session}
                api={api}
                nsfw={nsfw}
                setNsfw={setNsfw}
                setSession={setSession}
              />
            }
          />
          <Route
            path="/tags"
            element={
              <TagsGamePage
                session={session}
                api={api}
                nsfw={nsfw}
                setNsfw={setNsfw}
                setSession={setSession}
                tags={tags.map((t) => t.tag_name)}
              />
            }
          />
          <Route
            path="/history"
            element={
              <HistoryPage
                nsfw={nsfw}
                setNsfw={setNsfw}
                session={session}
                guessesToWin={3}
              />
            }
          />
          <Route
            path="/tag-list"
            element={<TagsListPage tags={tags} nsfw={nsfw} setNsfw={setNsfw} />}
          />
        </Routes>
      </BrowserRouter>
      <Toaster position="bottom-right" richColors theme="dark" />
    </div>
  );
}
