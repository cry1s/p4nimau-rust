# p4nimau-rust
## Автоматический пост контента из беседы в паблик
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/cry1s/p4nimau-rust/rust.yml?style=for-the-badge)

### Описание
p4nimau-rust - это Rust проект, который следит за беседой в группе ВКонтакте и автоматически выкладывает из неё картинки, видео и другой контент в паблик.

### Требования
Для сборки и запуска проекта вам потребуется установить следующие инструменты:
- [Rust](https://www.rust-lang.org/tools/install) - язык программирования Rust и его инструменты.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - пакетный менеджер и сборщик проектов Rust.

### Установка и сборка
1. Клонируйте репозиторий с помощью следующей команды:
   ```shell
   git clone https://github.com/cry1s/p4nimau-rust.git
   ```

2. Перейдите в директорию проекта:
   ```shell
   cd p4nimau-rust
   ```

3. Установите зависимости и выполните сборку проекта с помощью `Cargo`:
   ```shell
   cargo build --release
   ```

### Настройка
Для работы p4nimau-rust требуется указать некоторые переменные окружения. Создайте файл `.env` в корневой директории проекта и заполните его следующим образом:

```plaintext
VK_USER_TOKEN=<Токен пользователя ВКонтакте>
VK_GROUP_TOKEN=<Токен группы ВКонтакте>
```

- `VK_USER_TOKEN` - токен пользователя ВКонтакте, который позволит вам взаимодействовать с API.
- `VK_GROUP_TOKEN` - токен группы ВКонтакте, который позволит вам взаимодействовать с API.

### Запуск
Для запуска p4nimau-rust выполните следующую команду:
```shell
cargo run --release
```

### Тестирование
Для запуска тестов используйте команду:
```shell
cargo test
```

### Авторы
- Иван Крайников - cry1s@ya.ru
