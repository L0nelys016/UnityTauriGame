import { createSignal, For, Show, onMount } from "solid-js";
import Rating from "./Rating";
import "./Pages.css";
import { MainUserViewModel } from "../viewmodels/MainUserViewModel";
import { Game } from "../viewmodels/GameViewModel";
import { useAuth } from "../contexts/AuthContext";
import { useNotifications } from "../services/NotificationService";

export default function UserPage() {
  const viewModel = new MainUserViewModel();
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);
  const [showRating, setShowRating] = createSignal(false);
  const { user, logout } = useAuth();
  const { success, error: showError } = useNotifications();

  onMount(async () => {
    try {
      await viewModel.loadGames();
      await viewModel.loadGenres();
    } catch (err) {
      showError("Не удалось загрузить данные");
    }
  });

  const handleRate = (game: Game) => {
    setSelectedGame(game);
    setShowRating(true);
  };

  const handleRatingSave = async (rating: number) => {
    if (selectedGame()) {
      try {
        await viewModel.refreshGames();
        success("Оценка успешно сохранена");
        setShowRating(false);
      } catch (err) {
        showError("Не удалось обновить данные");
      }
    }
  };

  const handleRatingSkip = () => {
    setShowRating(false);
  };

  const handleLaunch = (game: Game) => {
    success(`Запуск игры: ${game.title}`);
  };

  return (
    <div class="auth-root" style={{ "flex-direction": "column", gap: "24px" }}>
      <div style={{ display: "flex", "justify-content": "space-between", "align-items": "center" }}>
        <div>
          <h1>Каталог игр</h1>
          {user() && (
            <p style={{ margin: "4px 0", color: "#64748b", "font-size": "14px" }}>
              Пользователь: {user()!.username}
            </p>
          )}
        </div>
        <button 
          class="btn btn--sm" 
          onClick={() => {
            logout();
            success("Вы вышли из системы");
          }}
          style={{ "background-color": "#ef4444", color: "white" }}
        >
          Выйти
        </button>
      </div>

      {/* Фильтры */}
      <div style={{ display: "flex", gap: "12px", "align-items": "center" }}>
        <input
          placeholder="Поиск..."
          value={viewModel.getSearch()}
          onInput={e => viewModel.setSearchValue(e.currentTarget.value)}
          class="auth-input"
        />
        <select
          value={viewModel.getFilterGenre() || ""}
          onChange={e => viewModel.setFilterGenreValue(
            e.currentTarget.value ? parseInt(e.currentTarget.value) : null
          )}
          class="auth-input"
        >
          <option value="">Все жанры</option>
          <For each={viewModel.getGenres()}>
            {(genre) => (
              <option value={genre.id}>{genre.name}</option>
            )}
          </For>
        </select>
        <select
          value={viewModel.getSortKey()}
          onChange={e => viewModel.setSortKeyValue(e.currentTarget.value as any)}
          class="auth-input"
        >
          <option value="title">Сортировка по названию</option>
          <option value="genre">Сортировка по жанру</option>
          <option value="rating">Сортировка по рейтингу</option>
        </select>
      </div>

      {/* Сетка карточек */}
      {viewModel.getLoading() ? (
        <div style={{ "text-align": "center", padding: "20px" }}>Загрузка...</div>
      ) : (
        <div class="user-grid" style={{
          display: "grid",
          "grid-template-columns": "repeat(auto-fit, minmax(260px, 1fr))",
          gap: "16px",
          width: "100%"
        }}>
          <For each={viewModel.getFilteredGames()}>
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

                <p style={{ margin: "4px 0", color: "#475569" }}>
                  Жанр: {viewModel.getGenreName(game.genre_id)}
                </p>
                <p style={{ margin: "4px 0", color: "#475569" }}>
                  Дата выхода: {game.release_date}
                </p>

                <p
                  class="rating"
                  style="margin: 8px 0; display: flex; align-items: center; gap: 4px; font-size: 24px;"
                >
                  {Array.from({ length: 5 }, (_, i) => i + 1).map(i => (
                    <span style={`color: ${i <= Math.round(game.average_rating) ? "#f59e0b" : "#e2e8f0"};`}>
                      ★
                    </span>
                  ))}
                  <span style="margin-left: 6px; color: #64748b; font-size: 14px;">
                    ({game.average_rating.toFixed(1)})
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
      )}

      {/* Оверлей для оценки */}
      <Show when={showRating() && selectedGame()}>
        <Rating
          gameId={selectedGame()!.id}
          userId={user()?.id || 0}
          gameTitle={selectedGame()?.title}
          onSave={handleRatingSave}
          onSkip={handleRatingSkip}
        />
      </Show>
    </div>
  );
}
