import { createSignal, For, Show, onMount } from "solid-js";
import Rating from "./Rating";
import "./Pages.css";
import { MainUserViewModel } from "../viewmodels/MainUserViewModel";
import { Game } from "../viewmodels/GameViewModel";
import { useAuth } from "../contexts/AuthContext";
import { useNotifications } from "../services/NotificationService";
import { webglApi, type WebglStatus } from "../webgl.api";

export default function UserPage() {
  const viewModel = new MainUserViewModel();
  const [selectedGame, setSelectedGame] = createSignal<Game | null>(null);
  const [showRating, setShowRating] = createSignal(false);
  const { user } = useAuth();
  const { success, error: showError } = useNotifications();

  const [launchGame, setLaunchGame] = createSignal<Game | null>(null);
  const [webglUrl, setWebglUrl] = createSignal<string>("");
  const [isGameBusy, setIsGameBusy] = createSignal(false);
  const [gameError, setGameError] = createSignal<string | null>(null);

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

  const handleRatingSave = async () => {
    if (selectedGame()) {
      try {
        await viewModel.refreshGames();
        success("Оценка успешно сохранена");
        setShowRating(false);
      } catch {
        showError("Не удалось обновить данные");
      }
    }
  };

  const handleRatingSkip = () => setShowRating(false);

  const handleLaunch = async (game: Game) => {
    setIsGameBusy(true);
    setGameError(null);
    try {
      // используем webglApi
      const status: WebglStatus = await webglApi.start();

      if (!status.running || !status.url) {
        throw new Error("Не удалось запустить WebGL сервер");
      }

      setWebglUrl(status.url);
      setLaunchGame(game);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setGameError(msg);
    } finally {
      setIsGameBusy(false);
    }
  };

  return (
    <div class="user-page">
      <div class="auth-background" />

      {/* Фильтры */}
      <div class="filters-container">
        <input
          placeholder="🔍 Поиск по названию..."
          value={viewModel.getSearch()}
          onInput={(e) => viewModel.setSearchValue(e.currentTarget.value)}
          class="filter-input"
        />
        <select
          value={viewModel.getFilterGenre() || ""}
          onChange={(e) =>
            viewModel.setFilterGenreValue(
              e.currentTarget.value ? parseInt(e.currentTarget.value) : null
            )
          }
          class="filter-select"
        >
          <option value="">Все жанры</option>
          <For each={viewModel.getGenres()}>
            {(genre) => <option value={genre.id}>{genre.name}</option>}
          </For>
        </select>
        <select
          value={viewModel.getSortKey()}
          onChange={(e) =>
            viewModel.setSortKeyValue(e.currentTarget.value as any)
          }
          class="filter-select"
        >
          <option value="title">📝 По названию</option>
          <option value="genre">🎭 По жанру</option>
          <option value="rating">⭐ По рейтингу</option>
        </select>
      </div>

      {/* Сетка карточек */}
      {viewModel.getLoading() ? (
        <div class="loading-container">
          <div class="loading-spinner"></div>
          <p>Загрузка игр...</p>
        </div>
      ) : (
        <Show
          when={viewModel.getFilteredGames().length > 0}
          fallback={
            <div class="empty-state">
              <div class="empty-state-icon">🎮</div>
              <h3>Игры не найдены</h3>
              <p>Попробуйте изменить параметры поиска или фильтры</p>
            </div>
          }
        >
          <div class="user-grid">
            <For each={viewModel.getFilteredGames()}>
              {(game) => (
                <div class="game-card">
                  <div class="game-card-header">
                    <h3 class="game-title">{game.title}</h3>
                  </div>
                  <div class="game-card-body">
                    <div class="game-info">
                      <span class="game-info-item">
                        🎭 {viewModel.getGenreName(game.genre_id)}
                      </span>
                      <span class="game-info-item">📅 {game.release_date}</span>
                    </div>
                    <div class="game-rating">
                      <div class="stars">
                        {Array.from({ length: 5 }, (_, i) => i + 1).map((i) => (
                          <span
                            class={`star ${
                              i <= Math.round(game.average_rating) ? "filled" : ""
                            }`}
                          >
                            ★
                          </span>
                        ))}
                      </div>
                      <span class="rating-value">
                        {game.average_rating.toFixed(1)} (
                        {game.total_ratings} оценок)
                      </span>
                    </div>
                  </div>
                  <div class="game-card-actions">
                    <button
                      class="btn btn-primary"
                      onClick={() => handleLaunch(game)}
                      disabled={isGameBusy()}
                    >
                      {isGameBusy() ? "Запуск..." : "🎮 Запустить"}
                    </button>
                    <button
                      class="btn btn-rating"
                      onClick={() => handleRate(game)}
                      title="Оценить игру"
                    >
                      ⭐
                    </button>
                  </div>
                </div>
              )}
            </For>
          </div>
        </Show>
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

      {/* Оверлей для WebGL */}
      <Show when={launchGame() && webglUrl()}>
        <div class="game-launch-overlay">
          <div class="overlay-header">
            <h2>{launchGame()!.title}</h2>
            <button class="btn-close" onClick={() => setLaunchGame(null)}>
              ✖
            </button>
          </div>
          <iframe
            src={webglUrl()}
            style={{
              width: "1024px",
              height: "768px",
              border: "none",
              "background-color": "#000",
            }}
            allowfullscreen
          />
          {gameError() && <p class="game-error">{gameError()}</p>}
        </div>
      </Show>
    </div>
  );
}
