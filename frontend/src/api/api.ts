import { SessionData } from "./sessionData";
import { ErrorType, HttpErrorBody } from "./apiError";
import { GuessRequest } from "./guessRequest";
import DanbooruPost from "./danbooruPost";

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
  ): Promise<{ guessed: boolean; post: DanbooruPost }> {
    const result = await this.request<[boolean, DanbooruPost]>("/aily", {
      method: "POST",
      body: JSON.stringify(req),
    });

    return {
      guessed: result[0],
      post: result[1],
    };
  }
}
