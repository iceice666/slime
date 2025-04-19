# Operators

## `+` `-` `*` `/` `**`

`<T: Num> T -> T -> T`

## `==` `!=` `<` `<=` `>` `>=`

`<T: Ord> T -> T -> Bool`

## `&&` `\|\|`

`Bool -> Bool -> Bool`

## `!`

`Bool -> Bool`

## `++`

`<T> List<T> -> List<T> -> List<T>`  
`<T> T -> List<T> -> List<T>`  
`<T> List<T> -> T -> List<T>`

## `.`

`(b -> c) -> (a -> b) -> a -> c`

## `|>` && `->`

`(a -> b) -> a -> b`

### What's their difference?

`->` pass the LHS to the RHS as the first argument, while `|>` pass the LHS to the RHS as the last argument.

For example:

```
let add = (a: Num, b: Num) -> Num { a + b };
let multiply = (a: Num, b: Num) -> Num { a * b };

// Using `->`
let result = 5 -> add(3) -> multiply(2); // Equivalent to multiply(add(5, 3), 2)

// Using `|>`
let result = 5 |> add(3) |> multiply(2); // Equivalent to multiply(2, add(3, 5))

```

### About method call

Since the `->` operator pass the LHS to the RHS as the first argument, it can simulate the OOP's method call:

```
let distance_to = (a: Num, b: Num) -> Num { abs(a - b) };
let result = 1 -> distance_to(2); // Equivalent to distance_to(1, 2)
```