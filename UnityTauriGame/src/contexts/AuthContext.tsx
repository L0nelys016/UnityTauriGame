import { createSignal, createContext, useContext, ParentComponent } from "solid-js";

export interface User {
  id: number;
  username: string;
  role: number; // 0 = User, 1 = Admin
}

interface AuthContextType {
  user: () => User | null;
  setUser: (user: User | null) => void;
  logout: () => void;
  isAdmin: () => boolean;
  isUser: () => boolean;
}

const AuthContext = createContext<AuthContextType>();

export function AuthProvider(props: ParentComponent) {
  const [user, setUser] = createSignal<User | null>(null);

  const logout = () => {
    setUser(null);
  };

  const isAdmin = () => user()?.role === 1;
  const isUser = () => user()?.role === 0;

  return (
    <AuthContext.Provider
      value={{
        user,
        setUser,
        logout,
        isAdmin,
        isUser,
      }}
    >
      {props.children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within AuthProvider");
  }
  return context;
}
