import { createSignal } from "solid-js";
import "./Pages.css";

export default function MainUser() {
  const [title] = createSignal("Профиль пользователя");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Здесь будет интерфейс пользователя: список игр, личный рейтинг и рекомендации.</p>
    </div>
  );
}

