# Learning Plan: Programming Language Theory for Language Design

This learning plan is designed for someone with Rust experience, some Haskell knowledge, and basic lambda calculus
understanding. It provides a structured path to master the theoretical foundations needed for designing a new
programming language with advanced features like fixed points, algebraic data types (ADTs), generalized algebraic data
types (GADTs), higher-kinded types (HKTs), and algebraic effects.

## Overview

This plan is organized in a progressive sequence, with each section building on the previous:

1. **Lambda Calculus Foundations** (2 weeks)
2. **Type Theory Fundamentals** (4 weeks)
3. **Category Theory Essentials** (6 weeks)
4. **Algebraic Data Types & Advanced Type Features** (4 weeks)
5. **Fixed Points & Recursion** (3 weeks)
6. **Algebraic Effects & Handlers** (3 weeks)
7. **Integration & Language Design** (4 weeks)

Total duration: Approximately 6 months (26 weeks)

---

## 1. Lambda Calculus Foundations

**Time Estimate:** 2 weeks

**Description:**  
Build on your existing knowledge of lambda calculus syntax and β-reduction by deepening your understanding of its
theoretical underpinnings and practical applications in programming language design.

**Key Concepts:**

- Church encodings (booleans, numbers)
- Combinators (S, K, I)
- Fixed-point combinators (Y combinator)
- Evaluation strategies (call-by-name, call-by-value)
- Relationship to recursive functions

**Learning Resources:**

- [A Tutorial Introduction to the Lambda Calculus](https://personal.utdallas.edu/~gupta/courses/apl/lambda.pdf) by Raul
  Rojas
- [The Y Combinator (no, not that one)](https://medium.com/@ayanonagon/the-y-combinator-no-not-that-one-7268d8d9c46) -
  For understanding recursion
- [Learn Lambda Calculus in Y Minutes](https://learnxinyminutes.com/docs/lambda-calculus/)
- [Lambda Calculus Chapter in Programming Language Theory](https://eecs390.github.io/notes/theory.html)

**Learning Approach:**

- Review the basics you already know
- Implement Church encodings and reducers in Rust/Haskell
- Practice beta-reduction by hand for complex expressions
- Implement a simple lambda calculus interpreter
- Focus on understanding the Y-combinator thoroughly

---

## 2. Type Theory Fundamentals

**Time Estimate:** 4 weeks

**Description:**  
Type theory provides the mathematical foundation for designing type systems. This section builds the formal framework
needed to understand modern type systems and their implementation in programming languages.

**Key Concepts:**

- Simply typed lambda calculus
- Type judgments and inference rules
- Polymorphic types (universal quantification)
- Hindley-Milner type inference
- Curry-Howard isomorphism (propositions as types)
- Dependent types (introduction)

**Learning Resources:**

- [Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/tapl/) by Benjamin C. Pierce (chapters 1-12,
  22-23)
- [Implementing a Hindley-Milner Type System](https://blog.stemsisna.com/posts/hindley-milner-1.html)
- [Stanford CS242: Programming Languages (Fall 2022)](https://github.com/danvk/Stanford-CS-242-Programming-Languages) (Type Systems lectures)
- [Computerphile: What is a Type?](https://www.youtube.com/watch?v=SWTWkYbcWU0)

**Learning Approach:**

- Read one chapter of TAPL per day, taking notes
- Implement the typing rules in a small interpreter
- Practice type inference by hand, then automate it
- Write a small type checker for a subset of Haskell or ML
- Connect type theory to your experience with Rust's type system

---

## 3. Category Theory Essentials

**Time Estimate:** 6 weeks

**Description:**  
Category theory provides a unified mathematical language for expressing relationships between structures. This abstract
framework will give you powerful tools for understanding and implementing advanced programming language features.

**Key Concepts:**

- Categories, objects, and morphisms
- Functors and natural transformations
- Products and coproducts
- Monads and monoids
- Adjunctions
- F-algebras and initial algebras
- Cartesian closed categories

**Learning Resources:**

- [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
  by Bartosz Milewski
    - [PDF Version](https://github.com/hmemcpy/milewski-ctfp-pdf)
    - [Accompanying YouTube Playlist](https://www.youtube.com/playlist?list=PLbgaMIhjbmEnaH_LTkxLI7FMa2HsnawM_)
- [Basic Category Theory for Computer Scientists](https://mitpress.mit.edu/9780262660716/basic-category-theory-for-computer-scientists/)
  by Benjamin C. Pierce
- [Cats library documentation](https://typelevel.org/cats/) - For practical examples in Scala
- [Understanding F-Algebras](https://www.fpcomplete.com/blog/2017/07/the-functor-combinatorplete-blog-post/)

**Learning Approach:**

- Start with the first part of Milewski's book (Chapters 1-10)
- Watch the corresponding videos for visual explanations
- Take notes on key categorical concepts and their programming analogues
- Implement examples in Haskell to solidify understanding
- Focus particularly on functors, monads, and F-algebras as these relate to your language design goals
- Don't rush this section; category theory takes time to internalize

---

## 4. Algebraic Data Types & Advanced Type Features

**Time Estimate:** 4 weeks

**Description:**  
This section covers algebraic data types (ADTs) and their extensions, focusing on how they provide structured ways to
model data in programming languages. You'll learn how GADTs and HKTs extend the expressive power of type systems.

**Key Concepts:**

- Sum and product types
- Pattern matching and elimination
- Generalized Algebraic Data Types (GADTs)
- Phantom types
- Higher-kinded types (HKTs)
- Kind systems
- Type-level programming

**Learning Resources:**

- [Algebraic Data Types Explained](https://jrsinclair.com/articles/2019/algebraic-data-types-what-i-wish-someone-had-explained-about-functional-programming/)
- [GHC User Guide: GADTs](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/gadt.html)
- [Generalized Algebraic Data Types](https://en.wikipedia.org/wiki/Generalized_algebraic_data_type)
- [Kinds and Higher-Kinded Types in Haskell](https://serokell.io/blog/kinds-and-hkts-in-haskell)
- [TAPL](https://www.cis.upenn.edu/~bcpierce/tapl/) chapters on polymorphism and advanced types
- [OCaml Manual: GADTs](https://v2.ocaml.org/manual/gadts.html)

**Learning Approach:**

- Start with basic ADTs in Haskell and their implementation
- Explore how Rust's enums implement algebraic data types
- Implement progressively more complex examples using GADTs
- Study how HKTs enable abstraction in functional libraries
- Create type-safe interpreters using GADTs
- Compare implementations across languages (Haskell, OCaml, Scala)

---

## 5. Fixed Points & Recursion

**Time Estimate:** 3 weeks

**Description:**  
Fixed points are essential for defining recursive structures and functions. This section connects lambda calculus,
category theory, and type theory through the lens of recursion and fixed points.

**Key Concepts:**

- Fixed point combinators in lambda calculus
- Fixpoint types and recursive types
- Initial algebras as fixpoints of functors
- Catamorphisms (folds) and anamorphisms (unfolds)
- Recursion schemes
- Fixed point theorems in category theory

**Learning Resources:**

- [Type theory and fixed points of datatypes](https://cstheory.stackexchange.com/questions/44917/type-theory-and-fixed-points-of-datatypes)
- [Functional Programming with Bananas, Lenses, Envelopes and Barbed Wire](https://maartenfokkinga.github.io/utwente/mmf91m.pdf)
- [Recursion Schemes: A Field Guide](https://medium.com/@jaredtobin/practical-recursion-schemes-c10648ec1c29)
- [F-Algebras and initial algebras](https://en.wikipedia.org/wiki/Initial_algebra)
- [Recursive Types for Free!](https://homepages.inf.ed.ac.uk/wadler/papers/free-rectypes/free-rectypes.txt) by Philip
  Wadler

**Learning Approach:**

- Review the Y combinator from the lambda calculus section
- Implement recursive data structures using fixed point types
- Study how Haskell and other functional languages implement recursion
- Implement basic recursion schemes (cata, ana, hylo)
- Connect fixed points in different domains (lambda calculus, category theory, type theory)
- Practice designing recursive data structures for your language

---

## 6. Algebraic Effects & Handlers

**Time Estimate:** 3 weeks

**Description:**  
Algebraic effects provide a principled way to handle computational effects (I/O, state, exceptions) in functional
languages. This modern approach separates effect declaration from their implementation.

**Key Concepts:**

- Effects vs. monads
- Effect systems
- Handlers and interpreters
- Effect composition
- Free monads and algebraic effects
- Effect polymorphism
- Effect inference

**Learning Resources:**

- [Algebraic Effects for the Rest of Us](https://overreacted.io/algebraic-effects-for-the-rest-of-us/) by Dan Abramov
- [An Introduction to Algebraic Effects and Handlers](https://www.eff-lang.org/handlers-tutorial.pdf)
- [Algebraic Effects for Functional Programming](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/08/algeff-tr-2016-v2.pdf)
- [Programming with Algebraic Effects and Handlers](https://arxiv.org/pdf/1203.1539.pdf)
- [What is algebraic about algebraic effects?](https://math.andrej.com/2019/09/03/what-is-algebraic-about-algebraic-effects/)
- [Koka Language Documentation](https://koka-lang.github.io/koka/doc/book.html#why-handlers)

**Learning Approach:**

- Compare monadic effects with algebraic effects
- Study implementations in Eff, Koka, and OCaml
- Implement a simple effect system in Haskell using free monads
- Design effect signatures for common computational effects
- Practice composing and interpreting effects
- Design an effect system for your own language

---

## 7. Integration & Language Design

**Time Estimate:** 4 weeks

**Description:**  
In this final section, you'll integrate the theoretical knowledge into practical language design, focusing on how these
concepts interact in a coherent programming language.

**Key Concepts:**

- Type system design tradeoffs
- Surface syntax vs. core calculus
- Type checking and inference algorithms
- Language implementation strategies
- Correctness properties and proofs
- Module systems and abstraction mechanisms
- Evaluation strategies and runtime considerations

**Learning Resources:**

- [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom
- [Practical Foundations for Programming Languages](https://www.cs.cmu.edu/~rwh/pfpl/) by Robert Harper
- [Programming Language Design Concepts](https://www.cs.cityu.edu.hk/~hwchun/PLDesign/chapter1.pdf)
- [r/ProgrammingLanguages](https://www.reddit.com/r/ProgrammingLanguages/) community
- [How to Design Languages](https://www.youtube.com/watch?v=DwLcGpxrZJM) by Simon Peyton Jones

**Learning Approach:**

- Design the core calculus of your language
- Implement a type checker based on your type system
- Create a simple interpreter or compiler
- Formalize key properties of your language
- Get feedback from the programming language community
- Iterate on your design based on practical implementation challenges
- Create a simple but complete language that demonstrates your target features

---

## Additional Resources

### Books

- [Advanced Topics in Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/attapl/) edited by Benjamin
  C. Pierce
- [Types and Programming Languages](https://www.cis.upenn.edu/~bcpierce/tapl/) by Benjamin C. Pierce
- [Category Theory for Programmers](https://bartoszmilewski.com/2014/10/28/category-theory-for-programmers-the-preface/)
  by Bartosz Milewski
- [Practical Foundations for Programming Languages](https://www.cs.cmu.edu/~rwh/pfpl/) by Robert Harper

### Online Courses

- [Programming Languages](https://www.coursera.org/learn/programming-languages) by Dan Grossman (Coursera)
- [Category Theory for Programmers](https://www.youtube.com/playlist?list=PLbgaMIhjbmEnaH_LTkxLI7FMa2HsnawM_) by Bartosz
  Milewski
- [Oregon Programming Languages Summer School](https://www.cs.uoregon.edu/research/summerschool/archives.html) archives

### Communities & Forums

- [r/ProgrammingLanguages](https://www.reddit.com/r/ProgrammingLanguages/)
- [Lambda the Ultimate](http://lambda-the-ultimate.org/)
- [TypeLevel](https://typelevel.org/) community

### Research Papers

- [A Taste of Linear Logic](https://homepages.inf.ed.ac.uk/wadler/papers/lineartaste/lineartaste-revised.pdf) by Philip
  Wadler
- [Propositions as Types](https://homepages.inf.ed.ac.uk/wadler/papers/propositions-as-types/propositions-as-types.pdf)
  by Philip Wadler
- [Theorems for Free!](https://people.mpi-sws.org/~dreyer/tor/papers/wadler.pdf) by Philip Wadler

---

## Suggested Schedule

Here's a suggested weekly breakdown to help you stay on track:

**Weeks 1-2: Lambda Calculus Foundations**

- Week 1: Review, Church encodings, combinators
- Week 2: Y-combinator, evaluation strategies, interpreter project

**Weeks 3-6: Type Theory Fundamentals**

- Week 3: Simply typed lambda calculus, type judgments
- Week 4: Polymorphic types, type inference basics
- Week 5: Hindley-Milner algorithm, implementation
- Week 6: Curry-Howard, introduction to dependent types

**Weeks 7-12: Category Theory Essentials**

- Weeks 7-8: Categories, objects, morphisms, functors
- Weeks 9-10: Products/coproducts, monads, monoids
- Weeks 11-12: F-algebras, initial algebras, applications

**Weeks 13-16: Algebraic Data Types & Advanced Type Features**

- Week 13: Sum/product types, pattern matching
- Week 14: GADTs basics and applications
- Week 15: Phantom types, existential types
- Week 16: Higher-kinded types, kind systems

**Weeks 17-19: Fixed Points & Recursion**

- Week 17: Fixed point combinators, fixpoint types
- Week 18: Initial algebras, folds/unfolds
- Week 19: Recursion schemes implementation

**Weeks 20-22: Algebraic Effects & Handlers**

- Week 20: Effect systems basics, comparison with monads
- Week 21: Handlers and interpreters
- Week 22: Effect composition and polymorphism

**Weeks 23-26: Integration & Language Design**

- Week 23: Core calculus design, type system specification
- Week 24: Type checker implementation
- Week 25: Interpreter implementation
- Week 26: Testing, refinement, documentation

---

## Final Notes

- **Balance theory and practice**: For each concept, try to implement a small example to test your understanding.
- **Set realistic goals**: This is an intensive learning plan. Feel free to adjust the timeline based on your pace.
- **Connect with communities**: Join forums like r/ProgrammingLanguages to discuss concepts and get feedback.
- **Build incrementally**: Start with a minimal language core and add features one by one.
- **Take notes**: Keep a journal of your learning journey, especially insights that connect different concepts.
- **Revisit concepts**: Many of these ideas will make more sense after you've seen them in multiple contexts.

This learning plan is ambitious but structured to build your knowledge progressively. The estimated times are
flexible—some sections may take longer based on your learning style and prior experience. Focus on deep understanding
rather than rushing through the material.

Good luck with your programming language design journey!