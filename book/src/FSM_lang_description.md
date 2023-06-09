#  Описание языка

Язык описывает FSM ( Finite State Machine ) с добавленными конструкциями, благодаря чему может 
компилироваться в конечном счете в Verilog 

Три главных конструкции в fsm_lang
- **Состояния** (States)
- **Входы и выходы** (Inputs and Outputs)
- **Переходы и действия** (Transitions and Actions)

## Описание элементов языка:
### Состояния (States)
Состояния: Состояние определяется ключевым словом state, за которым следует уникальное имя состояния и двоеточие.


Пример:
```
state StateA:
state StateB:
```

### Входы и выходы (Inputs and Outputs)
Входы и выходы: Входы и выходы определяются ключевыми словами input и output, за которыми следует уникальное имя входа или выхода и количество бит, заключенное в круглые скобки.

Пример:

```
input(1) Input1;
output(1) Output1;
```

### Переходы и действия (Transitions and Actions)
Переходы и действия: Переходы и действия определяются внутри блока состояния. Каждый переход начинается с ключевого слова on, за которым следует условие перехода, стрелка "->", имя целевого состояния и действие при переходе в фигурных скобках.

Пример:

```
on Input1 == 1 -> StateB {Output1 = 1};
```

## Пример 

```
state StateA:
  input(1) Input1;
  output(1) Output1;
  
  on Input1 == 1 -> StateB {Output1 = 1};
  on Input1 == 0 -> StateC {Output1 = 0};

state StateB:
  input(2) Input2;
  output(2) Output2;

  on Input2 == 2'b10 -> StateC {Output2 = 2'b01};

state StateC:
  input(2) Input2;
  output(2) Output2;

  on Input2 == 2'b01 -> StateA {Output2 = 2'b10};
```
