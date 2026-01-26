import { Show } from "solid-js";
import "./Pages.css";
import { RatingPageViewModel } from "../viewmodels/RatingPageViewModel";
import { useNotifications } from "../services/NotificationService";

interface RatingProps {
  gameId: number;
  userId: number;
  gameTitle?: string;
  onSave?: (rating: number) => void;
  onSkip?: () => void;
}

export default function Rating({ gameId, userId, gameTitle = "Title", onSave, onSkip }: RatingProps) {
  const viewModel = new RatingPageViewModel();
  const { error: showError } = useNotifications();

  const stars = [1, 2, 3, 4, 5];

  const handleSave = async () => {
    const success = await viewModel.save(userId, gameId);
    if (success && onSave) {
      onSave(viewModel.getRating());
    } else {
      const error = viewModel.getError();
      if (error) {
        showError(error);
      }
    }
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
                  onMouseEnter={() => viewModel.setHoverValue(star)}
                  onMouseLeave={() => viewModel.setHoverValue(0)}
                  onClick={() => viewModel.setRatingValue(star)}
                  style={{
                    color: (viewModel.getHover() >= star || viewModel.getRating() >= star) ? "#f59e0b" : "#e2e8f0"
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
            <button 
              class="overlay-save btn-primary" 
              onClick={handleSave}
              disabled={viewModel.getLoading() || viewModel.getRating() === 0}
            >
              {viewModel.getLoading() ? "Сохранение..." : "Сохранить оценку"}
            </button>
            <button 
              class="overlay-cancel" 
              onClick={handleSkip}
              disabled={viewModel.getLoading()}
            >
              Пропустить
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
