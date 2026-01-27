import { invoke } from "@tauri-apps/api/core";

export interface User {
  id: number;
  username: string;
  role: number; // 0 = User, 1 = Admin
}

export class AuthViewModel {
  async login(username: string, password: string): Promise<User> {
    try {
      const user = await invoke<User>("login", { username, password });
      return user;
    } catch (error) {
      throw new Error(`Ошибка входа: ${error}`);
    }
  }

  async getCurrentUser(userId: number): Promise<User | null> {
    try {
      // This would need a new command if needed
      return null;
    } catch (error) {
      throw new Error(`Не удалось получить пользователя: ${error}`);
    }
  }
}
