interface FooterProps {
  nsfw: boolean;
  setNsfw: (value: boolean) => void;
}

export function Footer({ nsfw, setNsfw }: FooterProps) {
  return (
    <footer className="flex items-center justify-between border-t border-zinc-800 pt-5">
      <label className="flex cursor-pointer items-center gap-3 text-sm font-medium text-zinc-300">
        <input
          type="checkbox"
          checked={nsfw}
          onChange={(event) => setNsfw(event.target.checked)}
          className="peer sr-only"
        />
        <span className="relative h-6 w-11 rounded-full bg-zinc-800 transition peer-checked:bg-red-600 peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-red-600 after:absolute after:left-1 after:top-1 after:h-4 after:w-4 after:rounded-full after:bg-white after:shadow-sm after:transition-transform peer-checked:after:translate-x-5" />
        Добавлять NSFW арты
      </label>
    </footer>
  );
}
