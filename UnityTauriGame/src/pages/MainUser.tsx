import { createSignal, For, Show } from "solid-js";
import Rating from "./Rating";
import "./Pages.css";

// Тип данных игры
type Game = {
  id: number;
  title: string;
  genre: string;
  developer: string;
  rating: number; // средний рейтинг
};

export default function UserPage() {
  const [games, setGames] = createSignal<Game[]>([
    { id: 1, title: "The Witcher 3", genre: "RPG", developer: "CD Projekt Red", rating: 4.5 },
    { id: 2, title: "Cyberpunk 2077", genre: "RPG", developer: "CD Projekt Red", rating: 4.2 },
    { id: 3, title: "Forza Horizon 5", genre: "Racing", developer: "Playground Games", rating: 4.7 },
    { id: 4, title: "Age of Empires IV", genre: "Strategy", developer: "Relic Entertainment", rating: 4.0 },
  ]);

  const [search, setSearch] = createSignal("");
  const [filterGenre, setFilterGenre] = createSignal("");
  const [sortKey, setSortKey] = createSignal<"title" | "genre" | "rating">("title");

  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);
  const [showRating, setShowRating] = createSignal(false);

  // Фильтрация и сортировка
  const filteredGames = () => {
    return games()
      .filter(g => g.title.toLowerCase().includes(search().toLowerCase()) &&
                   (!filterGenre() || g.genre === filterGenre()))
      .sort((a, b) => {
        if (sortKey() === "rating") return b.rating - a.rating;
        const valA = String(a[sortKey()]);
        const valB = String(b[sortKey()]);
        return valA.localeCompare(valB);
      });
  };

  const handleRate = (game: Game) => {
    setSelectedGame(game);
    setShowRating(true);
  };

  const handleRatingSave = (rating: number) => {
    setGames(prev => prev.map(g => g.id === selectedGame()!.id ? { ...g, rating } : g));
    setShowRating(false);
  };

  const handleRatingSkip = () => {
    setShowRating(false);
  };

  const handleLaunch = (game: Game) => {
    alert(`Запуск игры: ${game.title}`);
  };

  return (
    <div class="auth-root" style={{ "flex-direction": "column", gap: "24px" }}>
      <h1>Каталог игр</h1>

      {/* Фильтры */}
      <div style={{ display: "flex", gap: "12px", "align-items": "center" }}>
        <input
          placeholder="Поиск..."
          value={search()}
          onInput={e => setSearch(e.currentTarget.value)}
          class="auth-input"
        />
        <select
          value={filterGenre()}
          onChange={e => setFilterGenre(e.currentTarget.value)}
          class="auth-input"
        >
          <option value="">Все жанры</option>
          <option value="RPG">RPG</option>
          <option value="Strategy">Strategy</option>
          <option value="Racing">Racing</option>
        </select>
        <select
          value={sortKey()}
          onChange={e => setSortKey(e.currentTarget.value as any)}
          class="auth-input"
        >
          <option value="title">Сортировка по названию</option>
          <option value="genre">Сортировка по жанру</option>
          <option value="rating">Сортировка по рейтингу</option>
        </select>
      </div>

      {/* Сетка карточек */}
      <div class="user-grid" style={{
        display: "grid",
        "grid-template-columns": "repeat(auto-fit, minmax(260px, 1fr))",
        gap: "16px",
        width: "100%"
      }}>
        <For each={filteredGames()}>
          {game => (
            <div class="admin-card" style={{
              padding: "20px",
              "text-align": "left",
              "display": "flex",
              "flex-direction": "column",
              "justify-content": "space-between",
              "min-width": "260px"
            }}>
              <h3 style={{
                margin: "0 0 8px",
                color: "#0f172a",
                "font-weight": "700",
                "font-size": "18px"
              }}>
                {game.title}
              </h3>

              <p style={{ margin: "4px 0", color: "#475569" }}>Жанр: {game.genre}</p>
              <p style={{ margin: "4px 0", color: "#475569" }}>Разработчик: {game.developer}</p>

              <p
                class="rating"
                style="margin: 8px 0; display: flex; align-items: center; gap: 4px; font-size: 24px;"
              >
                {Array.from({ length: 5 }, (_, i) => i + 1).map(i => (
                  <span style={`color: ${i <= Math.round(game.rating) ? "#f59e0b" : "#e2e8f0"};`}>
                    ★
                  </span>
                ))}
                <span style="margin-left: 6px; color: #64748b; font-size: 14px;">
                  ({game.rating.toFixed(1)})
                </span>
              </p>


              <div style={{
                display: "flex",
                "justify-content": "space-between",
                gap: "12px",
                "margin-top": "12px"
              }}>
                <button
                  class="btn btn-primary"
                  style={{ flex: 1 }}
                  onClick={() => handleLaunch(game)}
                >
                  Запустить игру
                </button>
                <button
                  class="btn-icon"
                  style={{ padding: "0 12px", "font-size": "20px", color: "#f59e0b" }}
                  onClick={() => handleRate(game)}
                >
                  ⭐
                </button>
              </div>
            </div>
          )}
        </For>
      </div>

      {/* Оверлей для оценки */}
      <Show when={showRating()}>
        <Rating
          gameTitle={selectedGame()?.title}
          onSave={handleRatingSave}
          onSkip={handleRatingSkip}
        />
      </Show>
    </div>
  );
}
