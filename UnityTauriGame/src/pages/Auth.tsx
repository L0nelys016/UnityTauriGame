import "./Pages.css";

interface AuthProps {
  onLogin: () => void;
}

export default function Auth({ onLogin }: AuthProps) {
  const handleSubmit = (e: SubmitEvent) => {
    e.preventDefault();
    onLogin();
  };

  return (
    <div class="auth-root">
      <div class="auth-background" />

      <div class="auth-card">
        <div class="auth-card-header">
          <h1 class="auth-title">Вход в аккаунт</h1>
          <p class="auth-subtitle">
            Пожалуйста, введите логин и пароль для продолжения
          </p>
        </div>

        <form class="auth-form" onSubmit={handleSubmit}>
          <div class="auth-field">
            <label class="auth-label" for="email">
              Логин:
            </label>
            <input
              id="login"
              type="login"
              class="auth-input"
              placeholder="unitytaurigame"
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
        </form>
      </div>
    </div>
  );
}

