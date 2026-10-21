import { SessionData } from "@/models/sessionData";
import { PageLayout } from "@/components/PageLayout";
import { AilyHistory } from "../components/AilyHistory";

interface HistoryProps {
  nsfw: boolean;
  setNsfw: (v: boolean) => void;
  session: SessionData | null;
}

export function HistoryPage({ nsfw, setNsfw, session }: HistoryProps) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <AilyHistory session={session} />
    </PageLayout>
  );
}
