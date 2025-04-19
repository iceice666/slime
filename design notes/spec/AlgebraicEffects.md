# Algebraic Effects

## Introduction

Algebraic effects are a structured approach to handling computational effects, providing a balance between the
expressiveness of monads and the simplicity of direct-style programming. They allow for separating the definition of
effectful operations from their implementations, enabling more modular and reusable code.

## Core Concepts

### 1. Effects

An effect is a declaration of capabilities that a function may use. Effects are declared using the `effect` keyword and
contain one or more operation signatures.

```
effect Logger {
  let log = (message: String) -> ();
}

effect State<T> {
  let get = () -> T;
  let set = (value: T) -> ();
}

effect Exception<E> {
  let throw = (error: E) -> never;
}
```

### 2. Effect Operations

Operations are the actions that can be performed within an effect. They are defined within effect declarations and can
be invoked by functions that declare they use the effect.

### 3. Effect Handlers

Handlers interpret the operations of an effect, providing concrete implementations. They can be thought of as a form of
exception handling but for arbitrary operations.

```
let withConsoleLogger<T> = (fn: () -> T with Logger) -> T {
  handle fn() with {
    let log = (message: String) -> () {
      console.write(message);
    }
  }
}
```

### 4. Effect Typing

Functions that use effects must declare them in their type signature using the `with` keyword:

```
let logMessage = (message: String) -> () with Logger {
  Logger.log(message);
}
```

Using `+` to combine multiple effects:

```
let complexOperation = () -> T with Logger + State<Int> + Exception<String> {
  // Can use operations from all three effects
}
```

## Advanced Features

### 1. Resumable Operations

Unlike exceptions, effect operations can resume computation after being handled. This allows for more sophisticated
control flow patterns.

```
effect Coroutine {
  let yield = (value: T) -> ();
}

let runGenerator<T> = (generator: () -> () with Coroutine) -> List<T> {
  let values = List.empty<T>();

  handle generator() with {
    let yield = (value: T) -> () {
      values.push(value);
      resume(); // Continue execution after the yield
    }
  }

  return values;
}
```

### 2. Effect Composition

Multiple effects can be combined and used together:

```
let complexOperation = () -> T with Logger + State<Int> + Exception<String> {
  // Can use operations from all three effects
}
```

### 3. Effect Forwarding

Handlers can forward effect operations to outer handlers:

```
let withLogging<T, E> = (fn: () -> T with Exception<E> + Logger) -> T with Exception<E> {
  handle fn() with {
    let log = (message: String) -> () {
      console.write(f"[LOG]: {message}");
      resume();
    }
    // Exception operations are forwarded to outer handlers
  }
}
```

### 4. Effect Inference

Type inference works with effects to reduce boilerplate:

```
// Effect types inferred from usage
let process = data: List<Int> -> List<Int> {
  if (data.isEmpty()) {
    Exception.throw("Empty input");
  }

  Logger.log("Processing data...");
  return data.map(x => x * 2);
}
// Inferred type: List<Int> -> List<Int> with Logger + Exception<String>
```

## Implementation Strategy

### 1. Continuation-Passing Style Transformation

At the compiler level, effect operations are implemented via a CPS transformation, converting direct-style code with
effects into continuation-passing style code.

### 2. Effect Row Polymorphism

The type system uses row polymorphism to support extensible effects, allowing functions to work with any superset of
their required effects.

### 3. Performance Considerations

- Static analysis to eliminate unnecessary CPS transformations
- Optimizations for common effect patterns
- Specialized implementations for built-in effects

## Syntax Summary

### Effect Declaration

```
effect EffectName<Generic> {
  let operation1 = (param1: Type1, param2: Type2) -> ReturnType;
  let operation2 = () -> ReturnType;
}
```

### Effect Usage

```
let function = (param: Type) -> ReturnType with Effect1 + Effect2 {
  Effect1.operation(args);
  Effect2.operation(args);
}
```

### Effect Handling

```
let handler<T> = (fn: () -> T with Effect) -> T {
  handle fn() with {
    let operation1 = (param1: Type1, param2: Type2) -> ReturnType {
      // Implementation
      resume(returnValue); // Continue execution
    }

    let operation2 = () -> ReturnType {
      // Implementation without resuming (terminates the computation)
    }
  }
}
```

## Examples

### Error Handling

```
effect Try<E> {
  let throw = (error: E) -> never;
  let catch = (handler: (error: E) -> T) -> T;
}

let divideWithTry = (a: Int, b: Int) -> Int with Try<String> {
  if (b == 0) {
    Try.throw("Division by zero");
  }
  a / b
}

let safeDivide = (a: Int, b: Int) -> Option<Int> {
  handle {
    return Some(divideWithTry(a, b));
  } with {
    let throw = (msg: String) -> never {
      return None;
    }
  }
}
```

### State Management

```
effect State<S> {
  let get = () -> S;
  let set = (newState: S) -> ();
  let modify = (fn: (S) -> S) -> ();
}

let counter = () -> Int with State<Int> {
  let current = State.get();
  State.set(current + 1);
  return current;
}

let runState<S, T> = (initialState: S, fn: () -> T with State<S>) -> (T, S) {
  let state = initialState;

  let result = handle fn() with {
    let get = () -> S {
      resume(state)
    }

    let set = (newState: S) -> () {
      state = newState;
      resume()
    }

    let modify = (fn: (S) -> S) -> () {
      state = fn(state);
      resume()
    }
  }

  (result, state)
}
```

### Asynchronous Programming

```
effect Async {
  let await = (promise: Future<T>) -> T;
}

let fetchData = (url: String) -> String with Async {
  let response = Async.await(Http.get(url));
  response.body
}

let runAsync<T> = (fn: () -> T with Async) -> Future<T> {
  // Implementation using the platform's async mechanisms
}
```

