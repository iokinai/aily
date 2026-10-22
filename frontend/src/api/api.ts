import { SessionData } from "../models/sessionData";
import { ErrorType, HttpErrorBody } from "./apiError";
import { GuessRequest } from "./guessRequest";
import DanbooruPost from "../models/danbooruPost";
import { Artwork } from "../models/artwork";
import { TagGuessResult } from "../models/tagGuessResult";
import { TagsPost } from "../models/tagsPost";
import { SurrenderResult } from "@/models/surrenderResult";
import { Tag } from "@/models/tag";

export class Api {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options?: RequestInit,
  ): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        ...options?.headers,
      },
    });

    const contentType = response.headers.get("content-type");
    const isJson = contentType && contentType.includes("application/json");
    const data = isJson ? await response.json() : await response.text();

    if (!response.ok) {
      throw isJson
        ? (data as HttpErrorBody)
        : {
            error_type: ErrorType.ServerConnection,
            inner: null,
          };
    }

    return data as T;
  }

  async getSession(): Promise<SessionData> {
    return this.request<SessionData>("/session", { method: "GET" });
  }

  async startNewSession(): Promise<SessionData> {
    return this.request<SessionData>("/session", { method: "POST" });
  }

  async makeGuess(
    req: GuessRequest,
  ): Promise<{ guessed: boolean; post: Artwork; session: SessionData }> {
    const result = await this.request<[boolean, Artwork, SessionData]>(
      "/aily",
      {
        method: "POST",
        body: JSON.stringify(req),
      },
    );

    return {
      guessed: result[0],
      post: result[1],
      session: result[2],
    };
  }

  async artworkToPost(artwork: Artwork): Promise<DanbooruPost> {
    return await this.request<DanbooruPost>(
      `/aily/artwork_to_post?id=${artwork.id}`,
      {
        method: "GET",
      },
    );
  }

  async loadAilyArtwork(include_nsfw: boolean): Promise<Artwork> {
    return await this.request<Artwork>(`/aily?include_nsfw=${include_nsfw}`, {
      method: "GET",
      credentials: "include",
    });
  }

  async loadTagsArtwork(include_nsfw: boolean): Promise<Artwork> {
    return await this.request<Artwork>(`/tags?include_nsfw=${include_nsfw}`, {
      method: "GET",
      credentials: "include",
    });
  }

  async makeTagsGuess(request: {
    include_nsfw: boolean;
    tag: string;
  }): Promise<{ result: TagGuessResult; session: SessionData }> {
    const result = await this.request<[TagGuessResult, SessionData]>("/tags", {
      method: "POST",
      credentials: "include",
      body: JSON.stringify(request),
    });

    return {
      result: result[0],
      session: result[1],
    };
  }

  async artworkToTagsPost(artwork: Artwork): Promise<TagsPost> {
    return await this.request<TagsPost>(
      `/tags/artwork_to_post?id=${artwork.id}`,
      {
        method: "GET",
      },
    );
  }

  async tagsSurrender(
    include_nsfw: boolean,
  ): Promise<[SurrenderResult, SessionData]> {
    return await this.request<[SurrenderResult, SessionData]>(
      "/tags/surrender",
      {
        method: "POST",
        credentials: "include",
        body: JSON.stringify({
          include_nsfw: include_nsfw,
        }),
      },
    );
  }

  async getTagsList(): Promise<Tag[]> {
    return await this.request<Tag[]>("/tags/list", {
      method: "GET",
    });
  }
}
