import { useState, useEffect } from "react";
import { Api } from "./api/api";
import { SessionData } from "./api/sessionData";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AilyGamePage } from "./pages/AilyGamePage";
import { HistoryPage } from "./pages/HistoryPage";
import { Toaster } from "sonner";
import { toast } from "sonner";
import { errorToStringRu, HttpErrorBody } from "./api/apiError";

const api = new Api(`${window.APP_PATH}/api`);

export default function App() {
  const [loading, setLoading] = useState(true);
  const [session, setSession] = useState<SessionData | null>(null);
  const [nsfw, setNsfw] = useState(false);

  useEffect(() => {
    api
      .getSession()
      .then((session) => {
        setSession(session);
        setLoading(false);
      })
      .catch(() => {
        api
          .startNewSession()
          .then((session) => {
            setSession(session);
            setLoading(false);
          })
          .catch((err) => {
            toast.error(errorToStringRu(err as HttpErrorBody), {
              id: "session-start-error",
            });
          });
      });
  }, []);

  if (loading || session === null) {
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
            path="/history"
            element={
              <HistoryPage nsfw={nsfw} setNsfw={setNsfw} session={session} />
            }
          />
        </Routes>
      </BrowserRouter>
      <Toaster position="bottom-right" richColors theme="dark" />
    </div>
  );
}
