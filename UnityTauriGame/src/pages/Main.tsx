import { createSignal } from "solid-js";
import "./Pages.css";

export default function Main() {
  const [title] = createSignal("Вас приветствует, UnityTauriGame 👋");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Приложение успешно запущено.</p>
      <p>Здесь позже будет логика работы с играми и рейтингами.</p>
    </div>
  );
}