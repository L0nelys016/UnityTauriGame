import { createSignal, onMount, For, Show } from "solid-js";
import "./Pages.css";
import { ManagementGameViewModel } from "../viewmodels/ManagementGameViewModel";
import { GameViewModel, Game } from "../viewmodels/GameViewModel";
import { useNotifications } from "../services/NotificationService";

type Props = {
  game?: Game | null;
  onClose: () => void;
  onSave?: () => void;
};

export default function ManagementGame(props: Props) {
  const viewModel = new ManagementGameViewModel(props.game || undefined);
  const gameViewModel = new GameViewModel();
  const [genres, setGenres] = createSignal<Array<{ id: number; name: string }>>([]);
  const { error: showError } = useNotifications();
  
  const isEditMode = () => !!props.game;

  onMount(async () => {
    try {
      const loadedGenres = await gameViewModel.getAllGenres();
      setGenres(loadedGenres);
    } catch (err) {
      showError("Не удалось загрузить жанры");
    }
  });

  const save = async () => {
    const game = await viewModel.save();
    if (game) {
      props.onSave?.();
      props.onClose();
    } else {
      const error = viewModel.getError();
      if (error) {
        showError(error);
      }
    }
  };

  return (
    <div class="overlay-root">
      <div class="overlay-backdrop" onClick={props.onClose} />

      <div class="overlay-card">
        <h2 class="overlay-title">
          {isEditMode() ? "✏️ Редактировать игру" : "➕ Добавить игру"}
        </h2>

        <div class="overlay-form">
          <label class="overlay-field">
            <span>Название *</span>
            <input
              class="overlay-input"
              value={viewModel.getTitle()}
              onInput={(e) => viewModel.setTitleValue(e.currentTarget.value)}
              disabled={viewModel.getLoading()}
              placeholder="Введите название игры"
            />
          </label>

          <label class="overlay-field">
            <span>Описание</span>
            <textarea
              class="overlay-textarea"
              value={viewModel.getDescription()}
              onInput={(e) => viewModel.setDescriptionValue(e.currentTarget.value)}
              disabled={viewModel.getLoading()}
              placeholder="Введите описание игры (необязательно)"
              rows={4}
            />
          </label>

          <label class="overlay-field">
            <span>Жанр *</span>
            <select
              class="overlay-input"
              value={viewModel.getGenreId() || ""}
              onChange={(e) => viewModel.setGenreIdValue(
                e.currentTarget.value ? parseInt(e.currentTarget.value) : null
              )}
              disabled={viewModel.getLoading()}
            >
              <option value="">Выберите жанр</option>
              <For each={genres()}>
                {(genre) => (
                  <option value={genre.id}>{genre.name}</option>
                )}
              </For>
            </select>
          </label>

          <label class="overlay-field">
            <span>Дата выпуска</span>
            <input
              type="date"
              class="overlay-input"
              value={viewModel.getReleaseDate()}
              onInput={(e) => viewModel.setReleaseDateValue(e.currentTarget.value)}
              disabled={viewModel.getLoading()}
            />
          </label>
        </div>
s
        <Show when={viewModel.getError()}>
          <div style={{ 
            padding: "12px", 
            "margin-bottom": "16px", 
            background: "rgba(239, 68, 68, 0.1)", 
            color: "#ef4444", 
            "border-radius": "8px",
            "font-size": "14px"
          }}>
            {viewModel.getError()}
          </div>
        </Show>

        <div class="overlay-actions">
          <button 
            class="overlay-save" 
            onClick={save}
            disabled={viewModel.getLoading()}
          >
            {viewModel.getLoading() 
              ? (isEditMode() ? "Обновление..." : "Сохранение...") 
              : (isEditMode() ? "Обновить" : "Сохранить")}
          </button>
          <button 
            class="overlay-cancel" 
            onClick={props.onClose}
            disabled={viewModel.getLoading()}
          >
            Отмена
          </button>
        </div>
      </div>
    </div>
  );
}
