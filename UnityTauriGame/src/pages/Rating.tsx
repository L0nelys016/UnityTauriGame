import { createSignal } from "solid-js";
import "./Pages.css";

interface RatingProps {
  gameTitle?: string;
  onSave?: (rating: number) => void;
  onSkip?: () => void;
}

export default function Rating({ gameTitle = "Title", onSave, onSkip }: RatingProps) {
  const [rating, setRating] = createSignal(0);
  const [hover, setHover] = createSignal(0);

  const stars = [1, 2, 3, 4, 5];

  const handleSave = () => {
    if (onSave) onSave(rating());
  };

  const handleSkip = () => {
    if (onSkip) onSkip();
  };

  return (
    <div class="overlay-root">
      <div class="overlay-backdrop" onClick={handleSkip}></div>

      <div class="overlay-card">
        <h2 class="overlay-title">Оцените игру: "{gameTitle}"</h2>
        <div class="overlay-form">
          <div style={{ display: "flex", "flex-direction": "column", "align-items": "center", gap: "8px" }}>
            <div style={{ display: "flex", gap: "4px", "font-size": "32px", cursor: "pointer" }}>
              {stars.map((star) => (
                <span
                  onMouseEnter={() => setHover(star)}
                  onMouseLeave={() => setHover(0)}
                  onClick={() => setRating(star)}
                  style={{
                    color: (hover() >= star || rating() >= star) ? "#f59e0b" : "#e2e8f0"
                  }}
                >
                  ★
                </span>
              ))}
            </div>

            <div style={{ display: "flex", gap: "16px", color: "#64748b", "font-size": "14px" }}>
              {stars.map((star) => (
                <span>{star}</span>
              ))}
            </div>
          </div>

          <div class="overlay-actions">
            <button class="overlay-save btn-primary" onClick={handleSave}>Сохранить оценку</button>
            <button class="overlay-cancel" onClick={handleSkip}>Пропустить</button>
          </div>
        </div>
      </div>
    </div>
  );
}
