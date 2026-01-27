import { createSignal } from "solid-js";
import { RatingViewModel } from "./RatingViewModel";

export class RatingPageViewModel {
  private ratingViewModel: RatingViewModel;
  private rating = createSignal(0);
  private hover = createSignal(0);
  private loading = createSignal(false);
  private error = createSignal<string | null>(null);

  constructor() {
    this.ratingViewModel = new RatingViewModel();
  }

  getRating = () => {
    const [rating] = this.rating;
    return rating();
  };
  getHover = () => {
    const [hover] = this.hover;
    return hover();
  };
  getLoading = () => {
    const [loading] = this.loading;
    return loading();
  };
  getError = () => {
    const [error] = this.error;
    return error();
  };

  setRatingValue = (value: number) => {
    const [, setRating] = this.rating;
    setRating(value);
  };
  
  setHoverValue = (value: number) => {
    const [, setHover] = this.hover;
    setHover(value);
  };

  async save(userId: number, gameId: number): Promise<boolean> {
    const [rating] = this.rating;
    const [, setLoading] = this.loading;
    const [, setError] = this.error;
    
    if (rating() === 0) {
      setError("Пожалуйста, выберите оценку");
      return false;
    }

    setLoading(true);
    setError(null);

    try {
      await this.ratingViewModel.rateGame(userId, gameId, rating());
      return true;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Не удалось сохранить оценку");
      return false;
    } finally {
      setLoading(false);
    }
  }

  reset() {
    const [, setRating] = this.rating;
    const [, setHover] = this.hover;
    const [, setError] = this.error;
    
    setRating(0);
    setHover(0);
    setError(null);
  }
}
