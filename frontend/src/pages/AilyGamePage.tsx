import { PageLayout } from "../components/PageLayout";
import { AilyGame } from "./AilyGame";
import { Api } from "../api/api";
import { SessionData } from "../api/sessionData";

interface GamePageProps {
  session: SessionData;
  api: Api;
  nsfw: boolean;
  setNsfw: (value: boolean) => void;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
}

export function AilyGamePage({
  session,
  api,
  nsfw,
  setNsfw,
  setSession,
}: GamePageProps) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <AilyGame
        session={session}
        api={api}
        nsfw={nsfw}
        setSession={setSession}
      />
    </PageLayout>
  );
}
