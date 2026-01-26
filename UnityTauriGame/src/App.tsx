import { Show } from "solid-js";
import Auth from "./pages/Auth";
import MainAdmin from "./pages/MainAdmin";
import MainUser from "./pages/MainUser";
import { useAuth } from "./contexts/AuthContext";
import "./App.css";

function App() {
  const { user, setUser } = useAuth();

  const handleLogin = (user: { id: number; username: string; role: number }) => {
    setUser(user);
  };

  return (
    <div class="app">
      <main class="container">
        <Show when={!user()} fallback={
          <Show when={user()?.role === 1} fallback={<MainUser />}>
            <MainAdmin />
          </Show>
        }>
          <Auth onLogin={handleLogin} />
        </Show>
      </main>
    </div>
  );
}

export default App;
