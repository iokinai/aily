import { useState } from "react";
import { toast } from "sonner";

interface ProxyImageProps {
  src: string;
  className?: string;
  alt?: string;
  onLoadedWithoutError?: () => void;
}

export const ProxyImage = ({
  src,
  className,
  alt = "Artwork",
  onLoadedWithoutError: onReadyToGuess,
}: ProxyImageProps) => {
  const [isLoading, setIsLoading] = useState(true);
  const [hasError, setHasError] = useState(false);

  const proxyUrl = `${import.meta.env.VITE_API_URL}/api/proxy?url=${encodeURIComponent(src)}`;
  const fallbackImage = "/assets/img/fallback.jpeg";

  return (
    <div className="relative w-full h-full flex items-center justify-center overflow-hidden">
      {isLoading && !hasError && (
        <div className="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-zinc-900/90 z-10">
          <div className="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
          <p className="text-xs font-semibold uppercase tracking-[0.15em] text-zinc-400">
            Загрузка работы...
          </p>
        </div>
      )}

      <img
        src={hasError ? fallbackImage : proxyUrl}
        alt={alt}
        className={`${className} transition-opacity duration-300 w-full h-full ${hasError ? "object-cover" : "object-contain"} ${
          isLoading ? "opacity-0" : "opacity-100"
        }`}
        onLoad={() => {
          setIsLoading(false);
          onReadyToGuess?.();
        }}
        onError={() => {
          setIsLoading(false);
          setHasError(true);
          toast.error("Не удалось загрузить изображение через прокси", {
            id: "proxy-error",
          });
        }}
      />
    </div>
  );
};
