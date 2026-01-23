import "./Pages.css";

export default function Auth() {
  return (
    <div class="auth-root">
      <div class="auth-background" />

      <div class="auth-card">
        <div class="auth-card-header">
          <h1 class="auth-title">Вход в аккаунт</h1>
          <p class="auth-subtitle">
            Пожалуйста, введите адрес электронной почты и пароль для продолжения
          </p>
        </div>

        <form class="auth-form">
          <div class="auth-field">
            <label class="auth-label" for="email">
              Адрес электронной почты:
            </label>
            <input
              id="email"
              type="email"
              class="auth-input"
              placeholder="unitytaurigame@gmail.com"
            />
          </div>

          <div class="auth-field">
            <label class="auth-label" for="password">
              Пароль
            </label>
            <input
              id="password"
              type="password"
              class="auth-input"
              placeholder="• • • • • • • •"
            />
          </div>

          <div class="auth-remember-row">
            <label class="auth-remember-label">
              <input type="checkbox" class="auth-checkbox" />
              <span>Запомнить пароль</span>
            </label>
          </div>

          <button type="submit" class="auth-submit">
            Войти
          </button>

          <p class="auth-footer-text">
            Нет аккаунта?{" "}
            <button type="button" class="auth-link-button">
              Создать аккаунт
            </button>
          </p>
        </form>
      </div>
    </div>
  );
}

