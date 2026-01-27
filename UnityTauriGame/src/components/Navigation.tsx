import { Show } from "solid-js";
import { useAuth } from "../contexts/AuthContext";
import { useNotifications } from "../services/NotificationService";
import "./Navigation.css";

export default function Navigation() {
  const { user, logout, isAdmin, isUser } = useAuth();
  const { success } = useNotifications();

  const handleLogout = () => {
    logout();
    success("Вы вышли из системы");
  };

  return (
    <Show when={user()}>
      <nav class="navigation">
        <div class="nav-container">
          <div class="nav-brand">
            <h2>🎮 UnityTauriGame</h2>
          </div>
          
          <div class="nav-menu">
            <Show when={isAdmin()}>
              <span class="nav-link active">
                📊 Управление играми
              </span>
            </Show>
            
            <Show when={isUser()}>
              <span class="nav-link active">
                🎮 Каталог игр
              </span>
            </Show>
          </div>

          <div class="nav-user">
            <span class="nav-username">
              👤 {user()?.username}
              {isAdmin() && <span class="nav-badge">Админ</span>}
            </span>
            <button class="btn btn-primary btn--sm" onClick={handleLogout}>
              Выйти
            </button>
          </div>
        </div>
      </nav>
    </Show>
  );
}
