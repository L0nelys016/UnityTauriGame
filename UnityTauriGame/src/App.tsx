import { createSignal } from "solid-js";
import Auth from "./pages/Auth";
import MainAdmin from "./pages/MainAdmin";
import "./App.css";

function App() {
  const [currentPage, setCurrentPage] = createSignal<"auth" | "admin">("auth");

  return (
    <div class="app">
      <main class="container">
        {currentPage() === "auth" ? (
          <Auth onLogin={() => setCurrentPage("admin")} />
        ) : (
          <MainAdmin />
        )}
      </main>
    </div>
  );
}

export default App;
