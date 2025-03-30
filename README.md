# UniversalPath.org

Веб-сайт для публикации духовных учений и материалов, разработанный на Rust с использованием фреймворка Rocket.

![UniversalPath Logo](static/img/logo.svg)

## Содержание

- [Обзор](#обзор)
- [Требования](#требования)
- [Локальная разработка](#локальная-разработка)
- [Деплой](#деплой)
  - [Подготовка сервера](#подготовка-сервера)
  - [Настройка базы данных](#настройка-базы-данных)
  - [Настройка Nginx](#настройка-nginx)
  - [Деплой приложения](#деплой-приложения)
  - [Настройка SSL](#настройка-ssl)
  - [Настройка CI/CD](#настройка-cicd)
- [Обслуживание](#обслуживание)
- [API](#api)
- [Тестирование](#тестирование)
- [Архитектура](#архитектура)
- [Лицензия](#лицензия)

## Обзор

UniversalPath.org - это веб-сайт на русском языке для публикации духовных учений, включающий в себя:

- Статьи, организованные по категориям
- Глоссарий терминов
- Поиск по сайту
- Административная панель управления
- API для мобильных приложений

Сайт разработан с использованием следующих технологий:

- **Backend**: Rust, Rocket, SQLx, MySQL
- **Frontend**: HTML, CSS (Tailwind), JavaScript
- **База данных**: MySQL/MariaDB

## Требования

- Rust 1.70+ (с cargo)
- MySQL/MariaDB 5.7+
- Node.js 14+ (для сборки Tailwind CSS)
- Nginx (для продакшн-деплоя)

## Локальная разработка

1. **Клонирование репозитория**

```bash
git clone https://github.com/your-org/universalpath.org.git
cd universalpath.org
```

2. **Настройка базы данных**

```bash
# Создайте базу данных
mysql -u root -p -e "CREATE DATABASE catman CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
mysql -u root -p -e "CREATE USER 'catman'@'localhost' IDENTIFIED BY 'xB6udTcQaR2afdDX';"
mysql -u root -p -e "GRANT ALL PRIVILEGES ON catman.* TO 'catman'@'localhost';"
mysql -u root -p -e "FLUSH PRIVILEGES;"

# Импортируйте начальную схему
mysql -u catman -pxB6udTcQaR2afdDX catman < migrations/20250329_initial.sql
```

3. **Настройка переменных окружения**

Создайте файл `.env` в корне проекта:

```env
DATABASE_URL=mysql://catman:xB6udTcQaR2afdDX@localhost:3306/catman
JWT_SECRET=your_secure_random_string_here
SITE_URL=localhost:8000
```

4. **Сборка и запуск проекта**

```bash
# Сборка проекта
cargo build

# Запуск сервера в режиме разработки
cargo run
```

5. **Доступ к сайту**

Откройте браузер и перейдите по адресу `http://localhost:8000`

## Деплой

### Подготовка сервера

1. **Установка необходимых пакетов**

```bash
# Для Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev mariadb-server nginx

# Установка rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

2. **Настройка пользователя**

```bash
# Создание пользователя для запуска приложения
sudo useradd -m -s /bin/bash universalpath
sudo usermod -aG sudo universalpath

# Переключение на нового пользователя
sudo su - universalpath
```

### Настройка базы данных

```bash
# Создание базы данных и пользователя
sudo mysql -e "CREATE DATABASE catman CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
sudo mysql -e "CREATE USER 'catman'@'localhost' IDENTIFIED BY 'strong_password_here';"
sudo mysql -e "GRANT ALL PRIVILEGES ON catman.* TO 'catman'@'localhost';"
sudo mysql -e "FLUSH PRIVILEGES;"

# Импорт схемы базы данных
sudo mysql catman < /path/to/migrations/20250329_initial.sql
```

### Настройка Nginx

1. **Создание конфигурации Nginx**

```bash
sudo nano /etc/nginx/sites-available/universalpath.org
```

2. **Добавьте следующую конфигурацию:**

```nginx
server {
    listen 80;
    server_name universalpath.org www.universalpath.org;
    
    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
    
    # Статические файлы
    location /static/ {
        alias /home/universalpath/universalpath.org/static/;
        expires 30d;
        add_header Cache-Control "public, max-age=2592000";
    }
    
    # Для больших загрузок
    client_max_body_size 20M;
}
```

3. **Активация конфигурации**

```bash
sudo ln -s /etc/nginx/sites-available/universalpath.org /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### Деплой приложения

1. **Клонирование репозитория**

```bash
cd ~
git clone https://github.com/your-org/universalpath.org.git
cd universalpath.org
```

2. **Настройка переменных окружения**

```bash
nano .env
```

Добавьте:

```env
DATABASE_URL=mysql://catman:strong_password_here@localhost:3306/catman
JWT_SECRET=your_secure_random_string_here
SITE_URL=universalpath.org
```

3. **Сборка проекта**

```bash
cargo build --release
```

4. **Настройка systemd-сервиса**

```bash
sudo nano /etc/systemd/system/universalpath.service
```

Содержимое:

```ini
[Unit]
Description=UniversalPath.org Web Server
After=network.target mysql.service

[Service]
User=universalpath
WorkingDirectory=/home/universalpath/universalpath.org
Environment="ROCKET_ENV=production"
EnvironmentFile=/home/universalpath/universalpath.org/.env
ExecStart=/home/universalpath/universalpath.org/target/release/universal_path
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

5. **Запуск сервиса**

```bash
sudo systemctl enable universalpath
sudo systemctl start universalpath
sudo systemctl status universalpath
```

### Настройка SSL

Для включения HTTPS с Let's Encrypt:

```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d universalpath.org -d www.universalpath.org
```

### Настройка CI/CD

Для автоматизации деплоя можно использовать GitHub Actions. Создайте файл `.github/workflows/deploy.yml`:

```yaml
name: Deploy

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to production server
        uses: appleboy/ssh-action@master
        with:
          host: ${{ secrets.HOST }}
          username: ${{ secrets.USERNAME }}
          key: ${{ secrets.SSH_KEY }}
          script: |
            cd ~/universalpath.org
            git pull
            cargo build --release
            sudo systemctl restart universalpath
```

## Обслуживание

### Обновление сайта

```bash
cd ~/universalpath.org
git pull
cargo build --release
sudo systemctl restart universalpath
```

### Резервное копирование базы данных

```bash
# Создание резервной копии
mysqldump -u catman -p catman > backup_$(date +"%Y%m%d").sql

# Восстановление из резервной копии
mysql -u catman -p catman < backup_file.sql
```

### Просмотр логов

```bash
sudo journalctl -u universalpath.service -f
```

## API

API доступен по адресу `/api` и требует API-ключа, переданного в заголовке `x-api-key`.

Основные эндпоинты:
- `GET /api/articles` - Список статей
- `GET /api/articles/{id}` - Получение отдельной статьи
- `GET /api/categories` - Список категорий
- `GET /api/categories/{id}` - Получение отдельной категории
- `GET /api/terms` - Список терминов
- `GET /api/terms/{id}` - Получение отдельного термина

Для административных операций требуется JWT-токен, полученный через `/api/auth/login`.

Полную документацию API можно найти в файле [API.md](API.md).

## Тестирование

Для запуска тестов:

```bash
# Запуск автоматизированных тестов
chmod +x tests/run_tests.sh
./tests/run_tests.sh

# Или с указанием API URL
API_URL=http://localhost:8000/api WEB_URL=http://localhost:8000 ./tests/run_tests.sh
```

## Архитектура

Проект организован следующим образом:

```
universal_path/
├── src/                 # Исходный код Rust
│   ├── main.rs          # Точка входа
│   ├── config.rs        # Конфигурация
│   ├── db/              # Модели данных и схемы БД
│   ├── api/             # API эндпоинты
│   ├── routes/          # Веб-маршруты
│   ├── templates/       # Шаблоны Tera
│   └── utils/           # Вспомогательные функции
├── static/              # Статические ресурсы
│   ├── css/             # CSS стили
│   ├── js/              # JavaScript
│   └── img/             # Изображения
├── migrations/          # SQL-миграции
├── tests/               # Тесты
├── Cargo.toml           # Манифест пакета Rust
└── Rocket.toml          # Конфигурация Rocket
```

## Лицензия

Copyright © 2025 UniversalPath.org. Все права защищены.