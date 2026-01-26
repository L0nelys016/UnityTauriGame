import { createSignal } from "solid-js";
import { GameViewModel, Game, Genre } from "./GameViewModel";

export class MainUserViewModel {
  private gameViewModel: GameViewModel;
  private games = createSignal<Game[]>([]);
  private genres = createSignal<Genre[]>([]);
  private loading = createSignal(false);
  private error = createSignal<string | null>(null);
  private search = createSignal("");
  private filterGenre = createSignal<number | null>(null);
  private sortKey = createSignal<"title" | "genre" | "rating">("title");

  constructor() {
    this.gameViewModel = new GameViewModel();
  }

  getGames = () => this.games();
  getGenres = () => this.genres();
  getLoading = () => this.loading();
  getError = () => this.error();
  getSearch = () => this.search();
  getFilterGenre = () => this.filterGenre();
  getSortKey = () => this.sortKey();

  setSearchValue = (value: string) => {
    const [, setSearch] = this.search;
    setSearch(value);
  };
  
  setFilterGenreValue = (value: number | null) => {
    const [, setFilterGenre] = this.filterGenre;
    setFilterGenre(value);
  };
  
  setSortKeyValue = (value: "title" | "genre" | "rating") => {
    const [, setSortKey] = this.sortKey;
    setSortKey(value);
  };

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

  async loadGenres() {
    const [, setGenres] = this.genres;
    const [, setError] = this.error;
    
    try {
      const loadedGenres = await this.gameViewModel.getAllGenres();
      setGenres(loadedGenres);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось загрузить жанры");
    }
  }

  getFilteredGames(): Game[] {
    const [games] = this.games;
    const [search] = this.search;
    const [filterGenre] = this.filterGenre;
    const [sortKey] = this.sortKey;
    const [genres] = this.genres;
    
    return games()
      .filter((g) => {
        const matchesSearch = g.title.toLowerCase().includes(search().toLowerCase());
        const matchesGenre = !filterGenre() || g.genre_id === filterGenre();
        return matchesSearch && matchesGenre;
      })
      .sort((a, b) => {
        if (sortKey() === "rating") {
          return b.average_rating - a.average_rating;
        }
        if (sortKey() === "genre") {
          const genreA = genres().find((g) => g.id === a.genre_id)?.name || "";
          const genreB = genres().find((g) => g.id === b.genre_id)?.name || "";
          return genreA.localeCompare(genreB);
        }
        return a.title.localeCompare(b.title);
      });
  }

  getGenreName(genreId: number): string {
    const [genres] = this.genres;
    return genres().find((g) => g.id === genreId)?.name || "Неизвестно";
  }

  async refreshGames() {
    await this.loadGames();
  }
}
