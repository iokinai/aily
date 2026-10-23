import { Api } from "@/api/api";
import { PageLayout } from "@/components/PageLayout";
import { TagsGame } from "./TagsGame";
import { SessionData } from "@/models/sessionData";

interface GamePageProps {
  session: SessionData;
  api: Api;
  nsfw: boolean;
  setNsfw: (value: boolean) => void;
  setSession: React.Dispatch<React.SetStateAction<SessionData | null>>;
  tags: string[];
}

export function TagsGamePage({
  session,
  api,
  nsfw,
  setNsfw,
  setSession,
  tags,
}: GamePageProps) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <TagsGame
        api={api}
        nsfw={nsfw}
        session={session}
        setSession={setSession}
        tags={tags}
      />
    </PageLayout>
  );
}
