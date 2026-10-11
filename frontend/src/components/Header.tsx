interface HeaderProps {
  score: number;
  total: number;
}

export function Header({ score, total }: HeaderProps) {
  return (
    <header className="flex items-center justify-between border-b border-zinc-800 pb-5">
      <div className="font-display text-[2rem] font-semibold leading-none tracking-[-0.08em] text-white">
        Aily<span className="text-blue-500">.</span>
      </div>
      <div className="text-xs font-semibold uppercase tracking-[0.14em] text-zinc-400">
        Счет {score}/{total}
      </div>
    </header>
  );
}
