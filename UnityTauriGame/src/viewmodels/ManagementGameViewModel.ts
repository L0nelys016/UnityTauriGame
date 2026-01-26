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
  private title = createSignal("");
  private description = createSignal("");
  private genreId = createSignal<number | null>(null);
  private developer = createSignal("");
  private loading = createSignal(false);
  private error = createSignal<string | null>(null);

  constructor() {
    this.gameViewModel = new GameViewModel();
  }

  getTitle = () => this.title();
  getDescription = () => this.description();
  getGenreId = () => this.genreId();
  getDeveloper = () => this.developer();
  getLoading = () => this.loading();
  getError = () => this.error();

  setTitleValue = (value: string) => {
    const [, setTitle] = this.title;
    setTitle(value);
  };
  
  setDescriptionValue = (value: string) => {
    const [, setDescription] = this.description;
    setDescription(value);
  };
  
  setGenreIdValue = (value: number | null) => {
    const [, setGenreId] = this.genreId;
    setGenreId(value);
  };
  
  setDeveloperValue = (value: string) => {
    const [, setDeveloper] = this.developer;
    setDeveloper(value);
  };

  async save(): Promise<Game | null> {
    const [title] = this.title;
    const [genreId] = this.genreId;
    const [description] = this.description;
    const [, setTitle] = this.title;
    const [, setDescription] = this.description;
    const [, setGenreId] = this.genreId;
    const [, setDeveloper] = this.developer;
    const [, setLoading] = this.loading;
    const [, setError] = this.error;
    
    if (!title().trim()) {
      setError("Название обязательно");
      return null;
    }

    if (!genreId()) {
      setError("Жанр обязателен");
      return null;
    }

    setLoading(true);
    setError(null);

    try {
      const game = await this.gameViewModel.createGame(
        title(),
        description() || null,
        genreId()!,
        new Date().toISOString().split("T")[0] // Use current date as release_date
      );

      // Reset form
      setTitle("");
      setDescription("");
      setGenreId(null);
      setDeveloper("");

      return game;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось создать игру");
      return null;
    } finally {
      setLoading(false);
    }
  }

  reset() {
    const [, setTitle] = this.title;
    const [, setDescription] = this.description;
    const [, setGenreId] = this.genreId;
    const [, setDeveloper] = this.developer;
    const [, setError] = this.error;
    
    setTitle("");
    setDescription("");
    setGenreId(null);
    setDeveloper("");
    setError(null);
  }
}
