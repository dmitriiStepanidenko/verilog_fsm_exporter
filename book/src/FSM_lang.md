# Язык FSM

## Зачем?
Несмотря на изящество решений, использующих конечные автоматы в Verilog, нельзя не отметить 
их громоздкость и, порой, избыточность в плане написания кода, поэтому необходимо
удобное и более лаконичное средство описания Verilog-кода как конечного автомата.  

## Почему Rust? Почему Nom?
Причины использовать Rust:
1. Возможность в перспективе использовать макросы, которые смогут полностью парсить предложения
  at compile time. 
2. Более сглаженная кривая роста сложности программ (дискуссионный вопрос)
  ![RUST complexity](./imgs/RUST_complexity.png)

Причины НЕ использовать Rust:
1. Не так много библиотек.
2. Что еще важнее - не так много генераторов парсеров. Bison и antlr сюда не завезли

### А какой парсер собственно говоря нам нужен?


// Архитектура

// Грамматика

// Выбор парсера

// Что транслятору можно подать на вход и как он реагирует на ошибки