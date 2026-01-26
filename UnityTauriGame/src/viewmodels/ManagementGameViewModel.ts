import { createSignal } from "solid-js";
import { GameViewModel, Game } from "./GameViewModel";

export interface CreateGameData {
  title: string;
  description: string;
  genre_id: number;
  release_date: string;
}

export class ManagementGameViewModel {
  private gameViewModel: GameViewModel;
  private gameId = createSignal<number | null>(null);
  private title = createSignal("");
  private description = createSignal("");
  private genreId = createSignal<number | null>(null);
  private releaseDate = createSignal("");
  private developer = createSignal("");
  private loading = createSignal(false);
  private error = createSignal<string | null>(null);

  constructor(game?: Game) {
    this.gameViewModel = new GameViewModel();
    if (game) {
      this.loadGame(game);
    }
  }

  // Загружаем данные игры в сигналы
  loadGame(game: Game) {
    this.gameId[1](game.id);
    this.title[1](game.title);
    this.description[1](game.description || "");
    this.genreId[1](game.genre_id);
    this.releaseDate[1](game.release_date);
  }

  // Геттеры
  getGameId = () => this.gameId[0]();
  getTitle = () => this.title[0]();
  getDescription = () => this.description[0]();
  getGenreId = () => this.genreId[0]();
  getDeveloper = () => this.developer[0]();
  getReleaseDate = () => this.releaseDate[0]();
  getLoading = () => this.loading[0]();
  getError = () => this.error[0]();

  // Сеттеры
  setTitleValue = (value: string) => this.title[1](value);
  setDescriptionValue = (value: string) => this.description[1](value);
  setGenreIdValue = (value: number | null) => this.genreId[1](value);
  setDeveloperValue = (value: string) => this.developer[1](value);
  setReleaseDateValue = (value: string) => this.releaseDate[1](value);

  // Сохраняем игру (создание или обновление)
  async save(): Promise<Game | null> {
    const title = this.title[0]();
    const genreId = this.genreId[0]();
    const description = this.description[0]();
    const releaseDate = this.releaseDate[0]();
    const gameId = this.gameId[0]();

    this.loading[1](true);
    this.error[1](null);

    if (!title.trim()) {
      this.error[1]("Название обязательно");
      this.loading[1](false);
      return null;
    }

    if (!genreId) {
      this.error[1]("Жанр обязателен");
      this.loading[1](false);
      return null;
    }

    try {
      const releaseDateValue = releaseDate || new Date().toISOString().split("T")[0];
      let game: Game;

      if (gameId) {
        // Редактирование существующей игры
        game = await this.gameViewModel.updateGame(
          gameId,
          title,
          description || null,
          genreId,
          releaseDateValue
        );
      } else {
        // Создание новой игры
        game = await this.gameViewModel.createGame(
          title,
          description || null,
          genreId,
          releaseDateValue
        );
      }

      // Сброс формы
      this.reset();

      return game;
    } catch (err) {
      this.error[1](
        err instanceof Error
          ? err.message
          : gameId
          ? "Не удалось обновить игру"
          : "Не удалось создать игру"
      );
      return null;
    } finally {
      this.loading[1](false);
    }
  }

  // Сброс всех сигналов
  reset() {
    this.gameId[1](null);
    this.title[1]("");
    this.description[1]("");
    this.genreId[1](null);
    this.releaseDate[1]("");
    this.developer[1]("");
    this.error[1](null);
  }
}
