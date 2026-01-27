import { createSignal } from "solid-js";
import "./Pages.css";
import { AuthViewModel } from "../viewmodels/AuthViewModel";
import { useNotifications } from "../services/NotificationService";

interface AuthProps {
  onLogin: (user: { id: number; username: string; role: number }) => void;
}

export default function Auth({ onLogin }: AuthProps) {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [showPassword, setShowPassword] = createSignal(false);
  const [loading, setLoading] = createSignal(false);
  const authViewModel = new AuthViewModel();
  const { success, error: showError } = useNotifications();

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setLoading(true);

    try {
      const user = await authViewModel.login(username(), password());
      success(`Добро пожаловать, ${user.username}!`);
      onLogin(user);
    } catch (err) {
      showError(err instanceof Error ? err.message : "Ошибка входа");
    } finally {
      setLoading(false);
    }
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
            <label class="auth-label" for="login">
              Логин:
            </label>
            <input
              id="login"
              type="text"
              class="auth-input"
              placeholder="unitytaurigame"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
              disabled={loading()}
            />
          </div>
    
          <div class="auth-field">
            <label class="auth-label" for="password">
              Пароль
            </label>
            <div class="auth-input-wrapper">
              <input
                id="password"
                type={showPassword() ? "text" : "password"}
                class="auth-input"
                placeholder="• • • • • • • •"
                value={password()}
                onInput={(e) => setPassword(e.currentTarget.value)}
                disabled={loading()}
              />
              <button
                type="button"
                class={`auth-password-toggle ${showPassword() ? 'visible' : ''}`}
                onClick={() => setShowPassword(!showPassword())}
                disabled={loading()}
                tabIndex={-1}
                title={showPassword() ? "Скрыть пароль" : "Показать пароль"}
              >
                <span class="eye-icon">{showPassword() ? '👁️' : '⌣'}</span>
              </button>
            </div>
          </div>

          <button type="submit" class="btn btn-primary" disabled={loading()}>
            {loading() ? "Вход..." : "Войти"}
          </button>
        </form>
      </div>
    </div>
  );
}

