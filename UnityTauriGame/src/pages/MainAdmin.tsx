import { createSignal, For, Show, onMount } from "solid-js";
import "./Pages.css";
import ManagementGame from "./ManagementGame";
import { MainAdminViewModel } from "../viewmodels/MainAdminViewModel";
import { GameViewModel, Game } from "../viewmodels/GameViewModel";
import { useNotifications } from "../services/NotificationService";

type Props = {
  onGameLaunch?: (game: Game) => void;
};

export default function MainAdmin(props: Props) {
  const handleLaunch = (game: Game) => {
    if (props.onGameLaunch) {
      props.onGameLaunch(game);
    }
  };
  const viewModel = new MainAdminViewModel();
  const gameViewModel = new GameViewModel();
  const [genres, setGenres] = createSignal<Array<{ id: number; name: string }>>([]);
  const { success, error: showError } = useNotifications();

  const [showOverlay, setShowOverlay] = createSignal(false);
  const [editingGame, setEditingGame] = createSignal<Game | null>(null);

  onMount(async () => {
    try {
      await viewModel.loadGames();
      const loadedGenres = await gameViewModel.getAllGenres();
      setGenres(loadedGenres);
    } catch (err) {
      showError("Не удалось загрузить данные");
    }
  });

  const deleteGame = async (id: number) => {
    try {
      await viewModel.deleteGame(id);
      success("Игра успешно удалена");
    } catch (err) {
      showError("Не удалось удалить игру");
    }
  };

  const editGame = async (id: number) => {
    try {
      const game = await gameViewModel.getGame(id);
      if (game) {
        setEditingGame(game);
        setShowOverlay(true);
      } else {
        showError("Игра не найдена");
      }
    } catch (err) {
      showError("Не удалось загрузить игру для редактирования");
    }
  };

  const handleGameSaved = async () => {
    try {
      await viewModel.refreshGames();
      const isEdit = editingGame() !== null;
      success(isEdit ? "Игра успешно обновлена" : "Игра успешно добавлена");
      setShowOverlay(false);
      setEditingGame(null);
    } catch (err) {
      showError("Не удалось обновить список игр");
    }
  };

  const handleAddGame = () => {
    setEditingGame(null);
    setShowOverlay(true);
  };

  const getGenreName = (genreId: number): string => {
    return genres().find((g) => g.id === genreId)?.name || "Неизвестно";
  };

  return (
    <>
      {/* Основная страница */}
      <div class="user-page">
        <div class="auth-background" />

        <div class="admin-card">
          <div class="admin-header">
            <h1 class="admin-title">Управление играми</h1>
            <button class="btn btn-primary btn--sm" onClick={handleAddGame}>
              ➕ Добавить игру
            </button>
          </div>

          {viewModel.getLoading() ? (
            <div class="loading-container">
              <div class="loading-spinner"></div>
              <p>Загрузка игр...</p>
            </div>
          ) : (
            <div class="table-container">
              <table class="admin-table">
                <thead>
                  <tr>
                    <th>Название</th>
                    <th>Жанр</th>
                    <th>Рейтинг</th>
                    <th>Оценок</th>
                    <th>Действия</th>
                  </tr>
                </thead>

                <tbody>
                  <Show
                    when={viewModel.getGames().length > 0}
                    fallback={
                      <tr>
                        <td
                          colspan={5}
                          style={{
                            "text-align": "center",
                            padding: "3rem",
                            color: "#64748b",
                          }}
                        >
                          Игры не найдены. Добавьте первую игру!
                        </td>
                      </tr>
                    }
                  >
                    <For each={viewModel.getGames()}>
                      {(game) => (
                        <tr>
                          <td class="game-title-cell">{game.title}</td>
                          <td>{getGenreName(game.genre_id)}</td>
                          <td>
                            <span class="rating">
                              {"★".repeat(Math.round(game.average_rating))}
                              {"☆".repeat(5 - Math.round(game.average_rating))}
                            </span>
                          </td>
                          <td>{game.total_ratings}</td>
                          <td>
                            <div class="admin-actions">
                              <button
                                class="btn-icon play"
                                onClick={() => handleLaunch(game)}
                                title="Запустить"
                                style={{
                                  background: "rgba(16, 185, 129, 0.1)",
                                  color: "#10b981",
                                  transition: "all 0.2s ease",
                                }}
                                onMouseEnter={(e) =>
                                  (e.currentTarget.style.background = "rgba(16, 185, 129, 0.2)")
                                }
                                onMouseLeave={(e) =>
                                  (e.currentTarget.style.background = "rgba(16, 185, 129, 0.1)")
                                }
                              >
                                ▶️
                              </button>
                              <button
                                class="btn-icon edit"
                                onClick={() => editGame(game.id)}
                                title="Редактировать"
                                style={{
                                  background: "rgba(59, 130, 246, 0.1)",
                                  color: "#3b82f6",
                                  transition: "all 0.2s ease",
                                }}
                                onMouseEnter={(e) =>
                                  (e.currentTarget.style.background = "rgba(59, 130, 246, 0.2)")
                                }
                                onMouseLeave={(e) =>
                                  (e.currentTarget.style.background = "rgba(59, 130, 246, 0.1)")
                                }
                              >
                                ✏️
                              </button>
                              <button
                                class="btn-icon delete"
                                onClick={() => deleteGame(game.id)}
                                title="Удалить"
                                style={{
                                  background: "rgba(239, 68, 68, 0.1)",
                                  color: "#ef4444",
                                  transition: "all 0.2s ease",
                                }}
                                onMouseEnter={(e) =>
                                  (e.currentTarget.style.background = "rgba(239, 68, 68, 0.2)")
                                }
                                onMouseLeave={(e) =>
                                  (e.currentTarget.style.background = "rgba(239, 68, 68, 0.1)")
                                }
                              >
                                🗑️
                              </button>
                            </div>
                          </td>
                        </tr>
                      )}
                    </For>
                  </Show>
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>

      {/* Overlay / модалка */}
      <Show when={showOverlay()}>
        <ManagementGame
          game={editingGame()}
          onClose={() => setShowOverlay(false)}
          onSave={handleGameSaved}
        />
      </Show>
    </>
  );
}
