import { createSignal } from "solid-js";
import { GameViewModel, Game } from "./GameViewModel";

export class MainAdminViewModel {
  private gameViewModel: GameViewModel;
  private games = createSignal<Game[]>([]);
  private loading = createSignal(false);
  private error = createSignal<string | null>(null);

  constructor() {
    this.gameViewModel = new GameViewModel();
  }

  getGames = () => this.games();
  getLoading = () => this.loading();
  getError = () => this.error();

  async loadGames() {
    const [, setLoading] = this.loading;
    const [, setError] = this.error;
    const [, setGames] = this.games;
    
    setLoading(true);
    setError(null);
    try {
      const loadedGames = await this.gameViewModel.getAllGames();
      setGames(loadedGames);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось загрузить игры");
    } finally {
      setLoading(false);
    }
  }

  async deleteGame(id: number) {
    const [, setLoading] = this.loading;
    const [, setError] = this.error;
    const [games, setGames] = this.games;
    
    setLoading(true);
    setError(null);
    try {
      await this.gameViewModel.deleteGame(id);
      setGames(games().filter((g) => g.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось удалить игру");
    } finally {
      setLoading(false);
    }
  }

  async refreshGames() {
    await this.loadGames();
  }
}
