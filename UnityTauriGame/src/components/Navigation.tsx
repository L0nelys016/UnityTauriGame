import "./Navigaion.css";

export default function Navigaion() {
  return (
    <nav class="navigation">
      <ul>
        <li>
          <a href="#/">Главная</a>
        </li>
        <li>
          <a href="#/auth">Авторизация</a>
        </li>
        <li>
          <a href="#/user">Пользователь</a>
        </li>
        <li>
          <a href="#/rating">Рейтинг</a>
        </li>
        <li>
          <a href="#/admin">Админ</a>
        </li>
        <li>
          <a href="#/management-game">Управление играми</a>
        </li>
      </ul>
    </nav>
  );
}
