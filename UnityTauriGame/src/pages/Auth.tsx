import { createSignal } from "solid-js";
import "./Pages.css";

export default function Auth() {
  const [title] = createSignal("Авторизация");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Здесь будет форма входа и регистрации пользователей.</p>
    </div>
  );
}

