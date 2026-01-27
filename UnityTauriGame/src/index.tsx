/* @refresh reload */
import { render } from "solid-js/web";
import App from "./App";
import { NotificationProvider } from "./services/NotificationService";
import { AuthProvider } from "./contexts/AuthContext";

render(
  () => (
    <NotificationProvider>
      <AuthProvider>
        <App />
      </AuthProvider>
    </NotificationProvider>
  ),
  document.getElementById("root") as HTMLElement
);
