import { createSignal, createContext, useContext, ParentComponent } from "solid-js";

export type NotificationType = "success" | "error" | "info" | "warning";

export interface Notification {
  id: number;
  message: string;
  type: NotificationType;
  duration?: number;
}

interface NotificationContextType {
  notifications: () => Notification[];
  showNotification: (message: string, type?: NotificationType, duration?: number) => void;
  removeNotification: (id: number) => void;
  success: (message: string) => void;
  error: (message: string) => void;
  info: (message: string) => void;
  warning: (message: string) => void;
}

const NotificationContext = createContext<NotificationContextType>();

let notificationIdCounter = 0;

export function NotificationProvider(props: ParentComponent) {
  const [notifications, setNotifications] = createSignal<Notification[]>([]);

  const showNotification = (
    message: string,
    type: NotificationType = "info",
    duration: number = 3000
  ) => {
    const id = ++notificationIdCounter;
    const notification: Notification = { id, message, type, duration };

    setNotifications((prev) => [...prev, notification]);

    if (duration > 0) {
      setTimeout(() => {
        removeNotification(id);
      }, duration);
    }
  };

  const removeNotification = (id: number) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  };

  const success = (message: string) => showNotification(message, "success");
  const error = (message: string) => showNotification(message, "error", 5000);
  const info = (message: string) => showNotification(message, "info");
  const warning = (message: string) => showNotification(message, "warning");

  return (
    <NotificationContext.Provider
      value={{
        notifications,
        showNotification,
        removeNotification,
        success,
        error,
        info,
        warning,
      }}
    >
      {props.children}
      <NotificationContainer />
    </NotificationContext.Provider>
  );
}

export function useNotifications() {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error("useNotifications must be used within NotificationProvider");
  }
  return context;
}

function NotificationContainer() {
  const { notifications, removeNotification } = useNotifications();

  return (
    <div
      style={{
        position: "fixed",
        top: "20px",
        right: "20px",
        "z-index": "10000",
        display: "flex",
        "flex-direction": "column",
        gap: "10px",
        "max-width": "400px",
      }}
    >
      {notifications().map((notification) => (
        <div
          onClick={() => removeNotification(notification.id)}
          style={{
            padding: "12px 16px",
            "border-radius": "8px",
            "box-shadow": "0 4px 12px rgba(0, 0, 0, 0.15)",
            "background-color":
              notification.type === "success"
                ? "#10b981"
                : notification.type === "error"
                ? "#ef4444"
                : notification.type === "warning"
                ? "#f59e0b"
                : "#3b82f6",
            color: "white",
            cursor: "pointer",
            "font-size": "14px",
            "font-weight": "500",
            animation: "slideIn 0.3s ease-out",
            "word-wrap": "break-word",
            transition: "all 0.3s ease-out",
          }}
        >
          {notification.message}
        </div>
      ))}
      <style>{`
        @keyframes slideIn {
          from {
            transform: translateX(100%);
            opacity: 0;
          }
          to {
            transform: translateX(0);
            opacity: 1;
          }
        }
      `}</style>
    </div>
  );
}
