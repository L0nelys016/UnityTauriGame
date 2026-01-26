import { createSignal, For, Show } from "solid-js";
import "./Pages.css";
import ManagementGame from "./ManagementGame";

type Game = {
  id: number;
  title: string;
  genre: string;
  rating: number;
  ratingsCount: number;
};

const [showOverlay, setShowOverlay] = createSignal(false);

export default function MainAdmin() {  
  const [games, setGames] = createSignal<Game[]>([
    {
      id: 1,
      title: "The Witcher 3",
      genre: "RPG",
      rating: 4.5,
      ratingsCount: 15,
    },
    {
      id: 2,
      title: "Cyberpunk 2077",
      genre: "Action RPG",
      rating: 3.8,
      ratingsCount: 42,
    },
  ]);

  const deleteGame = (id: number) => {
    setGames(games().filter((g) => g.id !== id));
  };

  const editGame = (id: number) => {
    console.log("Редактировать игру с id:", id);
    // тут позже будет роутинг или модалка
  };

  return (
    <div class="auth-root">
      <div class="auth-background" />

      <div class="admin-card">
        <div class="admin-header">
          <h1 class="admin-title">Управление играми</h1>
          <button class="btn btn-primary btn--sm" onClick={() => setShowOverlay(true)}>
            Добавить игру
          </button>
        </div>

        <Show when={showOverlay()}>
          <ManagementGame
            onClose={() => setShowOverlay(false)}
          />
        </Show>

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
            <For each={games()}>
              {(game) => (
                <tr>
                  <td>{game.title}</td>
                  <td>{game.genre}</td>
                  <td>
                    <span class="rating">
                      {"★".repeat(Math.round(game.rating))}
                      {"☆".repeat(5 - Math.round(game.rating))}
                    </span>
                  </td>
                  <td>{game.ratingsCount}</td>
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
      </div>
    </div>
  );
}
