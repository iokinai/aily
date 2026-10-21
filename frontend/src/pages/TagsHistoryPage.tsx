import { PageLayout } from "@/components/PageLayout";
import { TagsHistory } from "@/components/TagsHistory";
import { SessionData } from "@/models/sessionData";
import { SetStateAction } from "react";

interface TagsHistoryPageOptions {
  nsfw: boolean;
  setNsfw: React.Dispatch<SetStateAction<boolean>>;
  session: SessionData;
  guessesToWin: number;
}

export function TagsHistoryPage({
  nsfw,
  setNsfw,
  session,
  guessesToWin,
}: TagsHistoryPageOptions) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <TagsHistory session={session} guessesToWin={guessesToWin} />
    </PageLayout>
  );
}
