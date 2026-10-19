import { SessionData } from "@/models/sessionData";
import { PageLayout } from "@/components/PageLayout";
import { History } from "./History";

interface HistoryProps {
  nsfw: boolean;
  setNsfw: (v: boolean) => void;
  session: SessionData | null;
}

export function HistoryPage({ nsfw, setNsfw, session }: HistoryProps) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <History session={session} />
    </PageLayout>
  );
}
