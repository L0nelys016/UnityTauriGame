import { invoke } from "@tauri-apps/api/core";

export interface Game {
  id: number;
  title: string;
  description: string | null;
  genre_id: number;
  release_date: string;
  average_rating: number;
  total_ratings: number;
}

export interface Genre {
  id: number;
  name: string;
}

export class GameViewModel {
  async getAllGames(): Promise<Game[]> {
    try {
      const games = await invoke<Game[]>("get_all_games");
      return games;
    } catch (error) {
      throw new Error(`Не удалось загрузить игры: ${error}`);
    }
  }

  async getGame(id: number): Promise<Game | null> {
    try {
      const game = await invoke<Game | null>("get_game", { id });
      return game;
    } catch (error) {
      throw new Error(`Не удалось загрузить игру: ${error}`);
    }
  }

  async createGame(
    title: string,
    description: string | null,
    genre_id: number,
    release_date: string
  ): Promise<Game> {
    try {
      const game = await invoke<Game>("create_game", {
        title,
        description,
        genreId: genre_id,
        releaseDate: release_date,
      });
      return game;
    } catch (error) {
      throw new Error(`Не удалось создать игру: ${error}`);
    }
  }

  async deleteGame(id: number): Promise<void> {
    try {
      await invoke("delete_game", { id });
    } catch (error) {
      throw new Error(`Не удалось удалить игру: ${error}`);
    }
  }

  async searchGames(query: string): Promise<Game[]> {
    try {
      const games = await invoke<Game[]>("search_games", { query });
      return games;
    } catch (error) {
      throw new Error(`Не удалось выполнить поиск: ${error}`);
    }
  }

  async getGamesByGenre(genre_id: number): Promise<Game[]> {
    try {
      const games = await invoke<Game[]>("get_games_by_genre", { genreId: genre_id });
      return games;
    } catch (error) {
      throw new Error(`Не удалось загрузить игры по жанру: ${error}`);
    }
  }

  async getAllGenres(): Promise<Genre[]> {
    try {
      const genres = await invoke<Genre[]>("get_all_genres");
      return genres;
    } catch (error) {
      throw new Error(`Не удалось загрузить жанры: ${error}`);
    }
  }
}
