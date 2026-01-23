import { createSignal } from "solid-js";
import "./Pages.css";

export default function MainAdmin() {
  const [title] = createSignal("Панель администратора");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Здесь будут административные функции: управление пользователями и доступами.</p>
    </div>
  );
}

