# Language Overview

## Core Language Syntax

### Comments

- Single line: `// This is a comment`
- Block: `/* This is a block comment */`
- Docstring: `/// This is a docstring`

### Types

`T` means a data type,  
`&T` means a reference to a data type,  
`&mut T` means a mutable reference to a data type.

Data is moved by default, but can be passed by reference using `&` or `&mut`.

#### Primitive Types

- Unit
- Bool
- U8, U16, U32, U64
- I8, I16, I32, I64
- F32, F64
- String: List\<u8\>

#### Built-in Types

- List<T>
- Set<T>
- Map<T, U>
- Option<T>
- Result<T,E>

### Variables: Let binding

```
let <name> [: <type>] = <value>;
```

### Let-in

```
let
  <name1> = <value1>
  <name2> = <value2>
in <expression>;
```

### Type Definitions

```
type <name> = (<type1>, <type2>, ...);      // Tuple
type <name> = {<name1>: <type1>, ... };     // Struct
type <name> = <name1>                       // Enum
            | (<name2,name3, ...>)
            | {<name4>: <type4>, ... };
type <name> = new <type>;                   // Same memory structure but different name/identifier
type <name> = alias <type>;                 // Just an alias
```

### Functions

```
let <name> <Generic Types> = (<name1>: <type1> [, <name2>: <type2> ...]) [-> <ret_type>] {};
```

Omit return type means return `Unit`

Generic Types can be: (type parameters are comma-seperated)

- `T`: Just a type parameter
- `T: Ord`: Add a trait bound
- `T: with Effect` Add a effect bound

The trailing `return <expression>;` statement can be simplified to just `<expression>`:

```
// With return keyword
let add = (a: Int, b: Int) -> Int {
  return a + b;
};

// With expression-only return
let add = (a: Int, b: Int) -> Int {
  a + b
};
```

### If-expression

```
if <condition> then <expression> else <expression>
```

### Pattern Matching

```
match <value> {
    | <pattern1> => <expression1>
    | <pattern2> => <expression2>
    | _ => <expression>
}
```

### Loops

```
// For loop
for i in <iterable> {
  // Do something
}

// While loop
while <condition> {
  // Do something
}
```

### String

Multiple line strings support by default

- Normal string: `"Hello, world!"`
- String interpolation: `f"Hello, {name}!"`
- Regular expression: `re"^[a-zA-Z0-9_]+$"`
- Raw string: `r"Hello, "world"!"`

## Language Features

### Type Inference

Explicitly support type inference to reduce verbosity while maintaining type safety:

```
// Type inferred from right side
let numbers = [1, 2, 3];

// Type parameters can often be inferred
let emptyList = List.empty();  // Instead of List.empty<Int>()
```

### Destructuring Assignments

Add support for pattern matching in variable assignments:

```
type Person = {name: String, age: Int};

// Destructure structs
let Person{name, age} = person;

// Destructure lists with rest pattern
let [first, second, ..rest] = someList;

// Destructure in function parameters
let processUser = (user: Person{name, age}) -> String {
  f"Processing ${user.name}, age ${user.age}"
};
```

### Type traits

Add a Rust-like trait system for generic programming:

```
// Defining a trait
trait Printable<T> {
    let toString = (value: T) -> String;
}

// Implementing a trait
impl Printable<User> {
  let toString = (user: User) -> String {
    f"User({user.name})"
  };
}

// Using a trait
let print<T: Printable> = (obj: T) {
  stdout.write(obj.toString())
};
```

### Module System

Define a clear module/namespacing system:

```
mod Collections {
  // Public exports
  pub type List<T> = ...

  // Private to module
  type Node<T> = ...
}

// Import specific items
import Collections.{List, Map};

// Import entire module
import Collections;
```

### Immutability By Default

Make variables immutable by default with explicit syntax for mutability:

```
// Immutable (default)
let counter = 0;

// Mutable
let mut counter = 0;
counter = counter + 1;
```

### Pattern Guards

Extend pattern matching with guards for more expressive conditionals:

```
match value {
    | n if n > 0 -> "Positive"
    | n if n < 0 -> "Negative"
    | _ -> "Zero"
}
```

### Algebraic Effects

See [Algebraic Effects](./AlgebraicEffects.md) for more details.

### Others

- ADT
- HKTs

## AI-friendly Features

Reference: [MoonBit: Explore the Design of an AI-Friendly Programming Language](https://llm4code.github.io/2024/assets/pdf/papers/7.pdf)

- **incremental parser**
- **semantic parser**


