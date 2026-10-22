import { useState, useRef, useEffect } from "react";

interface TagSearchProps {
  tags: string[];
  onSelectTag: (tag: string) => void;
  tagFinished: boolean;
}

export function TagInput({ tags, onSelectTag, tagFinished }: TagSearchProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const filteredTags = tags.filter((tag) =>
    tag.toLowerCase().includes(searchQuery.toLowerCase().trim()),
  );

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        containerRef.current &&
        !containerRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <div ref={containerRef} className="relative w-full">
      <input
        type="text"
        placeholder="Например, white_hair"
        disabled={tagFinished}
        value={searchQuery}
        onChange={(e) => {
          setSearchQuery(e.target.value);
          setIsOpen(true);
        }}
        onFocus={() => setIsOpen(true)}
        className="w-full flex-1 rounded-full border bg-ink/15 border-white px-5 py-3.5 text-sm text-white outline-none placeholder:text-white/30 focus:border-cobalt focus:ring-2 focus:ring-cobalt/15 disabled:cursor-default disabled:opacity-50"
      />

      {isOpen && searchQuery.trim().length > 0 && (
        <div className="absolute left-0 right-0 mt-2 z-20 rounded-xl bg-zinc-900 border border-zinc-800 shadow-xl max-h-60 overflow-y-auto p-2 flex flex-col gap-1">
          {filteredTags.length === 0 ? (
            <p className="text-xs text-zinc-500 py-2 px-3">Ничего не найдено</p>
          ) : (
            filteredTags.map((tag) => (
              <button
                key={tag}
                type="button"
                onMouseDown={(e) => e.preventDefault()}
                onClick={() => {
                  onSelectTag(tag);
                  setSearchQuery(tag);
                  setIsOpen(false);
                }}
                className="text-left px-3 py-2 rounded-lg hover:bg-zinc-800 text-sm text-zinc-200 transition"
              >
                {tag}
              </button>
            ))
          )}
        </div>
      )}
    </div>
  );
}
