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

  getGames = () => {
    const [games] = this.games;
    return games();
  };
  getGenres = () => {
    const [genres] = this.genres;
    return genres();
  };
  getLoading = () => {
    const [loading] = this.loading;
    return loading();
  };
  getError = () => {
    const [error] = this.error;
    return error();
  };
  getSearch = () => {
    const [search] = this.search;
    return search();
  };
  getFilterGenre = () => {
    const [filterGenre] = this.filterGenre;
    return filterGenre();
  };
  getSortKey = () => {
    const [sortKey] = this.sortKey;
    return sortKey();
  };

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
    
    const gamesValue = games();
    const searchValue = search();
    const filterGenreValue = filterGenre();
    const sortKeyValue = sortKey();
    const genresValue = genres();
    
    return gamesValue
      .filter((g: Game) => {
        const matchesSearch = g.title.toLowerCase().includes(searchValue.toLowerCase());
        const matchesGenre = !filterGenreValue || g.genre_id === filterGenreValue;
        return matchesSearch && matchesGenre;
      })
      .sort((a: Game, b: Game) => {
        if (sortKeyValue === "rating") {
          return b.average_rating - a.average_rating;
        }
        if (sortKeyValue === "genre") {
          const genreA = genresValue.find((g: Genre) => g.id === a.genre_id)?.name || "";
          const genreB = genresValue.find((g: Genre) => g.id === b.genre_id)?.name || "";
          return genreA.localeCompare(genreB);
        }
        return a.title.localeCompare(b.title);
      });
  }

  getGenreName(genreId: number): string {
    const [genres] = this.genres;
    const genresValue = genres();
    return genresValue.find((g: Genre) => g.id === genreId)?.name || "Неизвестно";
  }

  async refreshGames() {
    await this.loadGames();
  }
}
