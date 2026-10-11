export default interface DanbooruPost {
  id: number;
  danbooru_id: number | null;
  path: string;
  is_ai_generated: boolean;
}
