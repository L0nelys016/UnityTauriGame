import { invoke } from "@tauri-apps/api/core";

export interface Rating {
  id: number;
  user_id: number;
  game_id: number;
  score: number;
  created_at: string;
}

export class RatingViewModel {
  async rateGame(
    user_id: number,
    game_id: number,
    score: number
  ): Promise<Rating> {
    try {
      const rating = await invoke<Rating>("rate_game", {
        userId: user_id,
        gameId: game_id,
        score,
      });
      return rating;
    } catch (error) {
      throw new Error(`Не удалось оценить игру: ${error}`);
    }
  }

  async getGameRatings(game_id: number): Promise<Rating[]> {
    try {
      const ratings = await invoke<Rating[]>("get_game_ratings", { gameId: game_id });
      return ratings;
    } catch (error) {
      throw new Error(`Не удалось загрузить оценки игры: ${error}`);
    }
  }

  async getUserRatingForGame(
    user_id: number,
    game_id: number
  ): Promise<Rating | null> {
    try {
      const rating = await invoke<Rating | null>("get_user_rating_for_game", {
        userId: user_id,
        gameId: game_id,
      });
      return rating;
    } catch (error) {
      throw new Error(`Не удалось получить оценку пользователя: ${error}`);
    }
  }

  async getAverageRating(game_id: number): Promise<number> {
    try {
      const rating = await invoke<number>("get_average_rating", { gameId: game_id });
      return rating;
    } catch (error) {
      throw new Error(`Не удалось получить среднюю оценку: ${error}`);
    }
  }
}
