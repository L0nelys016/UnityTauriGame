import { createSignal } from "solid-js";
import "./Pages.css";

type Props = {
  onClose: () => void;
  onSave?: (game: {
    title: string;
    description: string;
    genre: string;
    developer: string;
  }) => void;
};

export default function ManagementGame(props: Props) {
  const [title, setTitle] = createSignal("");
  const [description, setDescription] = createSignal("");
  const [genre, setGenre] = createSignal("RPG");
  const [developer, setDeveloper] = createSignal("");

  const save = () => {
    if (!title().trim()) {
      alert("Название обязательно");
      return;
    }

    props.onSave?.({
      title: title(),
      description: description(),
      genre: genre(),
      developer: developer(),
    });

    props.onClose();
  };

  return (
    <div class="overlay-root">
      <div class="overlay-backdrop" onClick={props.onClose} />

      <div class="overlay-card">
        <h2 class="overlay-title">Добавить игру</h2>

        <div class="overlay-form">
          <label class="overlay-field">
            <span>Название *</span>
            <input
              class="overlay-input"
              value={title()}
              onInput={(e) => setTitle(e.currentTarget.value)}
            />
          </label>

          <label class="overlay-field">
            <span>Описание</span>
            <textarea
              class="overlay-textarea"
              value={description()}
              onInput={(e) => setDescription(e.currentTarget.value)}
            />
          </label>

          <label class="overlay-field">
            <span>Жанр</span>
            <select
              class="overlay-input"
              value={genre()}
              onChange={(e) => setGenre(e.currentTarget.value)}
            >
              <option value="RPG">RPG</option>
              <option value="Action">Action</option>
              <option value="Strategy">Strategy</option>
              <option value="Simulator">Simulator</option>
            </select>
          </label>

          <label class="overlay-field">
            <span>Разработчик</span>
            <input
              class="overlay-input"
              value={developer()}
              onInput={(e) => setDeveloper(e.currentTarget.value)}
            />
          </label>
        </div>

        <div class="overlay-actions">
          <button class="overlay-save" onClick={save}>
            Сохранить
          </button>
          <button class="overlay-cancel" onClick={props.onClose}>
            Отмена
          </button>
        </div>
      </div>
    </div>
  );
}
