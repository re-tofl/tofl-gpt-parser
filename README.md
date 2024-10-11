# tofl-gpt-parser

## Запуск сервера

Запуск осуществляется одним из двух способов:

1. Если на ПК установлен **Rust**, то с помощью команды:
   ```bash
   run --package tofl-gpt-parser --bin tofl-gpt-parser
   ```
2. Если установлен **Docker**, то с помощью команды:
   ```bash
   docker run -dp 127.0.0.1:8090:8090 parser
   ```

После запуска веб-сервер слушает на **8090 порту**.

Доступно два пути:

1. `GET /` — веб-страница для отладки.
2. `POST /parse` — сам парсер, данные передаются через **JSON**.

### Пример входных данных:

```json
{
  "TRS": "variables = x,y\nF(x) = G(y)",
  "Interpretation": "F(x) = x\nG(y) = 2*y\n"
}
```

### Пример выходных данных:

```json
{
  "json_TRS": [
    {
      "left": {
        "value": "F",
        "childs": [
          {
            "value": "x",
            "childs": []
          }
        ]
      },
      "right": {
        "value": "G",
        "childs": [
          {
            "value": "y",
            "childs": []
          }
        ]
      }
    }
  ],
  "json_interpret": {
    "functions": [
      {
        "name": "F",
        "variables": ["x"],
        "expression": "(x)"
      },
      {
        "name": "G",
        "variables": ["y"],
        "expression": "(2 * y)"
      }
    ]
  }
}
```

### Пример ошибочного вывода:

```json
{
  "error_trs": [
    "Ошибка в строке 1, на позиции 5, ожидалось b, считано ' '"
  ]
}
```