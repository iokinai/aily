import React from "react";

interface TagsGuessFormButtonOptions extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  children: React.ReactNode;
}

export function TagsGuessFormButton({
  children,
  className = "",
  ...props
}: TagsGuessFormButtonOptions) {
  return (
    <button
      {...props}
      className={`shrink-0 rounded-full px-5 py-3 text-xs font-bold uppercase tracking-widest transition hover:-translate-y-0.5 focus-visible:outline-2 focus-visible:outline-offset-2 disabled:cursor-default disabled:opacity-35 disabled:hover:translate-y-0 ${className}`}
    >
      {children}
    </button>
  );
}
