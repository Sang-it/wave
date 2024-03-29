# Wave
An over-engineered multi paradigm (toy) language that comes with its whole ecosystem.
The language will have many implementations; I just want to learn how everything works.

## Current Spec (Summarized).
```js
// Functions
function fibonacci(number){
    if (number == 1) return 0;
    if (number == 2) return 1;
    return fibonacci(number - 1) + fibonacci(number - 2);
}
// Classes
class Rectangle{
    constructor(length) {
        this.length = length;
    }
    getArea() {
        return this.length ** 2;
    }
}
// Inheritance
class Cube extends Rectangle{
    constructor(length, height) {
        super(length, length);
        this.height = height;
    }
    getVolume() {
        return this.getArea() * this.height;
    }
}
// Variable Declarations
let cube = new Cube(fibonacci(10),fibonnacci(11))
// StdOut
print(cube.getVolume)
```

Planned Language Implementations:
- Interpreter
- ByteCode-Interpreter(VM)
- LLVM

Planned components:
- Linter
- Formatter
- Minifier
- LSP (maybe)

<!-- ROADMAP -->
## Roadmap => Parser (Completed)
- [x] Implement arena allocator.
- [x] Implement compact-str and optimized span.
- [x] Implement diagnostics system.
- [x] Implement let statement parser
- [x] Implement const statement parser
- [x] Implement binary operations
- [x] Implement control statments
- [x] Implement function declarations
- [x] Implement return statements
- [x] Implement array declarations
- [x] Implement function call expressions
- [x] Implement unary, unary update and logical expressions
- [x] Implement while loops
- [x] Implement break and continue
- [x] Implement member expressions
- [x] Implement class related expressions

## Roadmap => Interpreter (will be updated as we go)
- [x] Implement binary operations
- [x] Implement logical operations
- [x] Implement environment scopes
- [x] Implement variable declarations
- [x] Implement if statements
- [x] Implement functions
- [x] Implement return statement
- [x] Implement function calls
- [x] Implement scoped environment
- [x] Implement assignment expressions
- [x] Implement while statement
- [x] Implement unary operators
- [x] Implement sequence expressions
- [x] Implement update operators
- [x] Implement class expressions
- [x] Implement static member expression
- [x] Implement constructors
- [x] Implement this expressions
- [x] Implement extend expressions
- [x] Implement super calls
- [x] Implement modules

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [oxc-project](https://oxc-project.github.io/docs/learn/parser_in_rust/intro.html)

