interface ResultBadgeOptions {
  success: boolean;
}

export function ResultBadge({ success }: ResultBadgeOptions) {
  return (
    <span
      className={`rounded-full px-3 py-2 text-[0.65rem] font-bold uppercase tracking-[0.12em] ${
        success
          ? "bg-cobalt/10 text-cobalt"
          : "bg-vermillion/10 text-vermillion"
      }`}
    >
      {success ? "Победа" : "Поражение"}
    </span>
  );
}
