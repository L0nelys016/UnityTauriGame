import { createSignal } from "solid-js";
import "./Pages.css";

export default function ManagementGame() {
  const [title] = createSignal("Управление играми");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Здесь администратор сможет добавлять, редактировать и удалять игры.</p>
    </div>
  );
}

