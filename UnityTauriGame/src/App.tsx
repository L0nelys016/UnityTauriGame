import { Show, createSignal } from "solid-js";
import Auth from "./pages/Auth";
import MainUser from "./pages/MainUser";
import Navigation from "./components/Navigation";
import { useAuth } from "./contexts/AuthContext";
import { Game } from "./viewmodels/GameViewModel";
import { webglApi, type WebglStatus } from "./webgl.api";
import "./App.css";

function App() {
  const { user, setUser } = useAuth();
  const [launchGame, setLaunchGame] = createSignal<Game | null>(null);
  const [webglUrl, setWebglUrl] = createSignal<string>("");

  const handleLogin = (user: { id: number; username: string; role: number }) => {
    setUser(user);
  };

  const handleGameLaunch = async (game: Game) => {
    try {
      // используем webglApi
      const status: WebglStatus = await webglApi.start();

      if (!status.running || !status.url) {
        throw new Error("Не удалось запустить WebGL сервер");
      }

      setWebglUrl(status.url);
      setLaunchGame(game);
    } catch (err) {
      console.error("Ошибка запуска игры:", err);
    }
  };

  const closeGame = () => {
    setLaunchGame(null);
    // Опционально: остановить сервер WebGL
    // webglApi.stop();
  };

  return (
    <div class="app">
      <Navigation />
      <main class="container">
        <Show when={!user()} fallback={
          <Show when={user()?.role === 1} fallback={<MainUser onGameLaunch={handleGameLaunch} />}>
            <MainUser onGameLaunch={handleGameLaunch} />
          </Show>
        }>
          <Auth onLogin={handleLogin} />
        </Show>
      </main>
      {/* Оверлей для WebGL - теперь находится на уровне приложения, чтобы перекрывать Navigation */}
      <Show when={launchGame() && webglUrl()}>
        <div class="game-launch-overlay">
          <div class="overlay-header">
            <h2>{launchGame()!.title}</h2>
            <button class="btn-close" onClick={closeGame}>
              ✖
            </button>
          </div>
          <iframe
            src={webglUrl()}
            style={{
              width: "100%",
              height: "100%",
              border: "none",
              "background-color": "#000",
            }}
            allowfullscreen
          />
        </div>
      </Show>
    </div>
  );
}

export default App;
