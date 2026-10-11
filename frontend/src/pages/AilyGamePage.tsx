import { PageLayout } from "../components/PageLayout";
import { AilyGame } from "./AilyGame";
import { Api } from "../api/api";
import { SessionData } from "../api/sessionData";

interface GamePageProps {
  loading: boolean;
  session: SessionData | null;
  api: Api;
  nsfw: boolean;
  setNsfw: (value: boolean) => void;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
}

export function AilyGamePage({
  loading,
  session,
  api,
  nsfw,
  setNsfw,
  setSession,
}: GamePageProps) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <AilyGame
        loading={loading}
        session={session}
        api={api}
        nsfw={nsfw}
        setSession={setSession}
      />
    </PageLayout>
  );
}
