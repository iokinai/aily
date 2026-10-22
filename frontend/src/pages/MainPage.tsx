import { Main } from "@/components/Main";
import { PageLayout } from "@/components/PageLayout";

interface MainPageLayout {
  nsfw: boolean;
  setNsfw: React.Dispatch<React.SetStateAction<boolean>>;
}

export function MainPage({ nsfw, setNsfw }: MainPageLayout) {
  return (
    <PageLayout nsfw={nsfw} setNsfw={setNsfw}>
      <Main />
    </PageLayout>
  );
}
