import { createSignal } from "solid-js";
import Auth from "./pages/Auth";
import MainAdmin from "./pages/MainAdmin";
import MainUser from "./pages/MainUser";
import "./App.css";

function App() {
  const [currentPage, setCurrentPage] = createSignal<"auth" | "admin" | "user">("auth");

  return (
    <div class="app">
      <main class="container">
        {currentPage() === "auth" && (
          <Auth 
            // временно переключаем на user для теста
            onLogin={() => setCurrentPage("user")} 
          />
        )}

        {currentPage() === "admin" && <MainAdmin />}
        {currentPage() === "user" && <MainUser />}
      </main>
    </div>
  );
}

export default App;
