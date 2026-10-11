import { useState, useEffect } from "react";
import { Api } from "./api/api";
import { SessionData } from "./api/sessionData";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { AilyGamePage } from "./pages/AilyGamePage";
import { HistoryPage } from "./pages/HistoryPage";

const api = new Api(window.APP_PATH);

export default function App() {
  const [loading, setLoading] = useState(true);
  const [session, setSession] = useState<SessionData | null>(null);
  const [nsfw, setNsfw] = useState(false);

  useEffect(() => {
    setLoading(true);
    api
      .getSession()
      .then((session) => {
        setSession(session);
        setLoading(false);
      })
      .catch(() => {
        api.start(nsfw).then((session) => {
          setSession(session);
          setLoading(false);
        });
      });
  }, []);

  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={
            <AilyGamePage
              loading={loading}
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
  );
}
