import { createSignal } from "solid-js";
import "./Pages.css";

export default function Rating() {
  const [title] = createSignal("Рейтинг игр");

  return (
    <div class="page">
      <h1>{title()}</h1>
      <p>Здесь будет общий рейтинг игр и подробная статистика.</p>
    </div>
  );
}

