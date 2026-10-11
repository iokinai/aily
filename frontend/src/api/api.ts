import DanbooruPost from "./danbooruPost";

export class Api {
  private url: string;
  private session_id: string | null;

  constructor(url: string) {
    this.url = url;
    this.session_id = null;
  }

  async start(includeNsfw: boolean): Promise<DanbooruPost> {
    const response = await fetch(
      `${this.url}/start?include_nsfw=${includeNsfw}`,
      {
        method: "GET",
      },
    );

    if (!response.ok) {
      throw new Error("Failed to start game");
    }

    const result = await response.json();
    this.session_id = result[0];
    return result[1] as DanbooruPost;
  }

  async guess(
    include_nsfw: boolean,
    ai: boolean,
  ): Promise<{ guessed: boolean; post: DanbooruPost }> {
    const response = await fetch(
      `${this.url}/guess?include_nsfw=${include_nsfw}&is_ai_generated=${ai}`,
      {
        method: "GET",
        headers: {
          "X-Session-ID": this.session_id ?? "",
        },
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
