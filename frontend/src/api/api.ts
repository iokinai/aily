import DanbooruPost from "./danbooruPost";
import { SessionData } from "./sessionData";

export class Api {
  private url: string;

  constructor(url: string) {
    this.url = url;
  }

  async getSession(): Promise<SessionData> {
    const response = await fetch(`${this.url}/session`, {
      method: "GET",
      credentials: "include",
    });

    if (!response.ok) {
      throw new Error("Failed to restore session");
    }

    const result = await response.json();
    return result as SessionData;
  }

  async start(includeNsfw: boolean): Promise<DanbooruPost> {
    const response = await fetch(
      `${this.url}/start?include_nsfw=${includeNsfw}`,
      {
        method: "GET",
        credentials: "include",
      },
    );

    if (!response.ok) {
      throw new Error("Failed to start game");
    }

    const result = await response.json();
    return result as DanbooruPost;
  }

  async guess(
    include_nsfw: boolean,
    ai: boolean,
  ): Promise<{ guessed: boolean; post: DanbooruPost }> {
    const response = await fetch(
      `${this.url}/guess?include_nsfw=${include_nsfw}&is_ai_generated=${ai}`,
      {
        method: "GET",
        credentials: "include",
      },
    );

    if (!response.ok) {
      console.log(await response.json());
      throw new Error("Failed to guess");
    }

    const result = await response.json();
    return {
      guessed: result[0],
      post: result[1] as DanbooruPost,
    };
  }
}
