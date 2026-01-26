import { createSignal, For, Show, onMount } from "solid-js";
import "./Pages.css";
import ManagementGame from "./ManagementGame";
import { MainAdminViewModel } from "../viewmodels/MainAdminViewModel";
import { GameViewModel } from "../viewmodels/GameViewModel";
import { useNotifications } from "../services/NotificationService";
import { useAuth } from "../contexts/AuthContext";

const [showOverlay, setShowOverlay] = createSignal(false);

export default function MainAdmin() {
  const viewModel = new MainAdminViewModel();
  const gameViewModel = new GameViewModel();
  const [genres, setGenres] = createSignal<Array<{ id: number; name: string }>>([]);
  const { success, error: showError } = useNotifications();
  const { logout, user } = useAuth();

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

  const editGame = (id: number) => {
    console.log("Редактировать игру с id:", id);
    // тут позже будет роутинг или модалка
  };

  const handleGameSaved = async () => {
    try {
      await viewModel.refreshGames();
      success("Игра успешно добавлена");
      setShowOverlay(false);
    } catch (err) {
      showError("Не удалось обновить список игр");
    }
  };

  const getGenreName = (genreId: number): string => {
    return genres().find((g) => g.id === genreId)?.name || "Неизвестно";
  };

  return (
    <div class="auth-root">
      <div class="auth-background" />

      <div class="admin-card">
        <div class="admin-header">
          <div>
            <h1 class="admin-title">Управление играми</h1>
            {user() && (
              <p style={{ margin: "4px 0", color: "#64748b", "font-size": "14px" }}>
                Пользователь: {user()!.username}
              </p>
            )}
          </div>
          <div style={{ display: "flex", gap: "8px" }}>
            <button class="btn btn-primary btn--sm" onClick={() => setShowOverlay(true)}>
              Добавить игру
            </button>
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
        </div>

        <Show when={showOverlay()}>
          <ManagementGame
            onClose={() => setShowOverlay(false)}
            onSave={handleGameSaved}
          />
        </Show>

        {viewModel.getLoading() ? (
          <div style={{ "text-align": "center", padding: "20px" }}>Загрузка...</div>
        ) : (
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
              <For each={viewModel.getGames()}>
                {(game) => (
                  <tr>
                    <td>{game.title}</td>
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
                          class="btn-icon edit"
                          onClick={() => editGame(game.id)}
                          title="Редактировать"
                        >
                          ✏️
                        </button>
                        <button
                          class="btn-icon delete"
                          onClick={() => deleteGame(game.id)}
                          title="Удалить"
                        >
                          🗑️
                        </button>
                      </div>
                    </td>
                  </tr>
                )}
              </For>
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}
