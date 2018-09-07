ЛОЛкоин
=======

ЛОЛкоин -- это виртуальная валюта, разработанная специально для [ЛОЛ](https://github.com/qbit-org-ua/LOL).

День рождения ЛОЛкоина 15 августа 2018 года.

Этот репозиторий содержит реализацию ЛОЛкоинов, которая состоит из двух компонент:

* RESTful API сервер, реализованный на Rust (`/src`)
* WebUI сервер, реализованный на JavaScript (`/web`)

Проект реализован с использованием кросплатформенных технологий и может быть
запущен под ОС Linux, Windows и MacOS.

Запуск проекта
--------------

Для запуска проекта, необходимо сконфигурировать, собрать и запустить API и Web сервера.

### Настройка и запуск API сервера

Для сборки RESTful API сервера требуется компилятор Rust (nightly). Сборка
проекта осуществляется следующей командой (запущенной в корне проекта):

```
cargo build --release
```

Перед запуском, необходимо:

1. Создать файл-базу пользователей `users.json` в текущей директории, например:

    ```json
    {
      "00000000-0000-0000-0000-000000000000": {"full_name": "LOL", "secret": "password"},
      "384fc6ad-0556-4301-ba3b-757c9ad29423": {"full_name": "Фролов Владислав Владимирович", "secret": "test"},
    }
    ```
2. Создать пустой файл истории переводов `transfers.json.log` в текущей директории.

Для запуска, выполните скомпилированный файл из директории `target/release/`:

```
./target/release/lolcoin
```

Запущенный RESTful API сервер должен оставаться запущенным всё время и
обслуживать запросы, поступающие на TCP порт 3030 (0.0.0.0:3030).


### Настройка и запуск WebUI сервера

Для сборки WebUI сервера требуется Node.js. Сборка проекта осуществляется
следующей командой (запущенной в корне проекта):

```
npm install
npm run build
```

Запуск проекта:

```
npm run start
```

Запущенный WebUI сервер должен оставаться запущенным всё время и обслуживать
запросы, поступающие на TCP порт 3000 (0.0.0.0:3000).


### Взаимодействие с ЛОЛкоин проектом

После запуска RESTful API и WebUI серверов, ЛОЛкоин проект будет доступен по
адресу http://127.0.0.1:3000.
