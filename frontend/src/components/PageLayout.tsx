import { JSX } from "react";
import { Header } from "./Header";
import { Footer } from "./Footer";

interface PageLayoutProps {
  children?: JSX.Element[] | JSX.Element | undefined | null;
  nsfw: boolean;
  setNsfw: (v: boolean) => void;
}

export function PageLayout({ children, nsfw, setNsfw }: PageLayoutProps) {
  return (
    <main className="min-h-screen px-5 py-6 sm:px-8 sm:py-8 bg-zinc-950 text-zinc-100 flex flex-col justify-between">
      <Header />
      <div className="mx-auto flex w-full max-w-5xl flex-col flex-1">
        {children}
      </div>
      <Footer nsfw={nsfw} setNsfw={setNsfw} />
    </main>
  );
}
