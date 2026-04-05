## 📐 Syntax examples

This document describes syntax of the `Squirrel` programming language.

### Data types
| Data type | Description                                                               |   Rust representation            |
|-----------|---------------------------------------------------------------------------|----------------------------------|
| int       | integer number                                                            | `i64`                            |
| decimal   | floating-point number                                                     | `f64`                            |
| bool      | logical (bool) type: `true` or `false`                                    | `bool`                           |
| string    | text data                                                                 | `String`                         |
| callable  | represents reference to  any callable: function, native, bound, etc.      | `Rc<Function>`                   |
| class     | represents reference to the class.                                        | `Rc<Class>`                      |
| enum      | represents reference to enumeration.                                      | `Rc<Enum>`                       |
| instance  | represents reference to instance of the type.                             | `Rc<RefCell<Instance>>`          |
| null      | represents null value or `nothing`.                                       | `()`                             |
| module    | represents reference to the module.                                       | `Rc<RefCell<Module>>`            |
| any       | represents internal rusts `std::Any` variable                             | `Rc<RefCell<dyn std::any::Any>>` |

### Variable declaration
`Squirrel` does not support variables shadowing, so here's
a way to define variable and to reassign it.

Variable definition:
```Squirrel
let id = value;
```

Variable assignment:
```
id = value;
```

### Binary operations
`Squirrel` supports following binary operations:

```Squirrel
+ - * / % && & || | ^ > < == !=
```

### Unary operations
`Squirrel` supports following unary operations:

```
- !
```

### Compound operators
`Squirrel` supports following compound operators:

```
id += value;
id -= value;
id *= value;
id /= value;
id %= value;
id &= value;
id |= value;
```

### Value examples
Examples of the values:

| Data type | Example of the value       |
|-----------|----------------------------|
| int       | 123                        | 
| decimal   | 123.456                    |
| bool      | true / false               |
| string    | "text"                     |
| function  | fn(x, y) {} return x + y } |
| class     | AnyDeclaredClass           |
| enum      | AnyDeclaredEnum            |
| instance  | AnyDeclaredClass()         |
| null      | null                       |
| native    | declared native            |
| module    | module                     |
| any       | any_native_value           |

### Functions example
Here's an example on how you can define function in `Squirrel`:

```Squirrel
fn fib(x) {
  if x <= 1 {
    return x;
  } else {
    return fib(x - 1) + fib(x - 2);
  }
}
```

Squirrel supports closures:

```Squirrel
fn a() {
  let x = 1;
  fn b() {
    x += 1;
  }
  b(); // x = 2
  return b;
}

let b = a();
b(); // x = 3
b(); // x = 4
b(); // x = 5
```

### Classes or custom data types
Squirrel supports custom data types. Here is example:
```Squirrel
type Dog {
  fn init() {
    self.food = 3;
    self.water = 3;
  }
  fn get_food() {
    return self.food;
  }
}
let dog = Dog();
let a = dog.get_food();
let b = dog.food;
# a == b
```

### Comments
Squirrel comments examples:

```
#[
Here is multiline 
comment in 
square
brackets
]#
```

```
# Here is single line comment
```

### Usings
Squirrel is modular:
```
use a # import `a` as `a`
use a as b # import `a` as `b`
use a for b # import `b` from `a` directly by `shallow copying` it
use a for b, c # import multiple items
```

### Loops
Squirrel loops examples:

For loop with range examples.
You can use any expression instead of numbers in range.
```
for i in 0..100 {
  println(i);
}

for i in 100..0 {
  println(i);
}

for i in 0..=100 {
  println(i);
}

for i in 100..=0 {
  println(i)
}
```

While loop examples. You can see, that `Squirrel` supports `continue` and `break` keywords
```
let i = 0;
while true {
  if i == 100 {
    continue;
    i -= 200;
  }
  i += 1;
  if i == -199 {
    break;
  }
}
```

### Logical statements
If examples:
```Squirrel
let a = scan();
if int(a) > 5 {
  ...
} else if int(a) < 5 {
  ...
} else {
  ...
}
```

### Errors raising
Bail immediately breaks execution with error:
```Squirrel
bail "some text"
```

### Anonymous function
Squirrel supports rust-like anonymous functions:
```Squirrel
let a = || 1;
let b = |a| a + 1;
let c = |a| {
  return a + 1;
};
println(a());
println(b(1));
println(c(2));
```

### Enumerations
Squirrel supports enumerations. Every variant is just an int variable inside `Dog`.
```Squirrel
enum Dog {
  Poodle,  # 0
  Bulldog, # 1
  Beagle,
  Husky
}
let dog = Dog.Poodle;
println(dog == 0); # true
println(dog == Dog.Beagle); # false
```
