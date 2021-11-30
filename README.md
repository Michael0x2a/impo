# Impo

Impo is a minimalist language that I hope to one day use as part of a larger
project to help teach data structures and algorithms and other computer science
fundamentals. (But in practice, is just an excuse for me to play around with
Rust and LLVM.)

To support this ultimate goal, Impo has the following design goals:

1. The syntax should be as minimal as possible and closely resemble pseudocode.
   It should look aesthetically pleasing when presented within a code snippet online.

2. The language should have similar semantics to mainstream programming languages
   (C#, Java, JavaScript, Python...) while still remaining minimal. It should stick
   to supporting just the lowest common denominator of these languages and avoid
   innovating too aggressively.

3. The language should as easy to transpile as possible to the above languages.

This language will support the following (conventional) language features:

1.  Garbage collection
2.  Basic imperative control flow features
3.  Generics
4.  Fixed-sized arrays
5.  Static type checking (with type inference -- only function signatures and fields
    require type annotations)

Some unusual aspects of this language include:

1.  Comments are _not_ ignored by the parser. Instead, they are treated as metadata
    attached to the following statement. As a consequence, the following 

    1.  Comments _must_ be preceded by either `# ` or `#\n` -- the octothorpe followed
        by either a space or a newline.

    2.  A `#` _must_ be preceded by whitespace characters. That is, comments must stand
        alone on a separate line, and cannot be used to annotate existing lines.

    3.  A comment-line _must_ be followed by either another comment-line or by a
        statement. This implies comments cannot exist at the end of a file or
        at the end of a function.

    This makes it significantly easier to transpile Impo, since comments can now
    be treated as first-class citizens in the AST.

2.  Inheritance is not supported: languages are only allowed to implement interfaces.

    This helps reduce complexity in the language spec.

3.  Statements may be preceded by "hints" which take the form `!key: value`.
    These are ignored by the language, but are attached to the AST node for use by
    the transpiler.

    Hints attached to interface definitions are always applied to the respective
    methods in the implementing classes.

Some decisions I'm still thinking about:

1.  Should covariance or contravariance be supported?

2.  Should operator overloading or optional arguments be supported?

3.  How should errors be propagated? Especially in a way that supports
    transpiling to both languages that use exceptions (e.g. Java) and
    languages that return errors or Result objects (Go or Rust).

4.  Should union/sum types be supported?

5.  How do modules work?

6.  Should subtyping be supported?


## Task list

- [x] Implement the parsing and lexing machinery.
- [ ] Finish implementing the entire parser
    - [x] For expressions
    - [ ] For statements
    - [ ] For types
- [ ] Implement a type checker
- [ ] Implement an interpreter and/or compiler using LLVM
- [ ] Implement support for transpiling
    - [ ] To Java
    - [ ] To Python
    - [ ] To JavaScript
    - [ ] To Go

## Example program and ad-hoc language spec

```
const _DEFAULT_CAPACITY = 8

interface List[T]:
    fn get_item(index: Int) -> T
    fn set_item(index: Int, value: T)
    fn append(value: T)
    fn length() -> Int
    fn equals(other: Self) -> Bool

sentinal IteratorDone

interface Iterator[T]:
    fn next() -> T | IteratorDone

    !Java: Omit()
    fn has_next() -> bool


class ArrayList[T] implements List[T]:
    _array: Array[T]
    _length: Int

    fn constructor():
        this._array = Array[T](8)
        this._length = 0

    fn get_item(index: Int) -> T:
        this._check_bounds(index)
        return this._array[index]

    fn set_item(index: Int, value: T):
        this._check_bounds(index)
        return this._array[index]

    fn append(value: T):
        if this._length == this._array.length():
            var new_array = Array[T](this._length * 2)
            copy(this._array, new_array)
            this._array = new_array
        this._array[this._length] = value
        this._length += 1

    fn length() -> Int:
        return this._length

    fn equals(other: This) -> Bool:
        if this._length != other._length:
            return false
        for i from 0 to this._length:
            if this._array[i] != other._array[i]:
                return false
        return true
            
    fn _check_bounds(index: Int):
        if index < 0 or index >= this._length():
            !Java: TypeOverride(Exception, IndexOutOfBounds)
            panic "Index out of bounds"

class _ArrayListIterator[T] implements Iterator[T]:
    _array: Array[T]
    _position: Int

    constructor(array: Array[T]):
        this._array = array
        this._position = 0

    fn next() -> T | IteratorDone:
        if this._position == this._array.length():
            return IteratorDone
        else:
            var out = this._array[this._position]
            this._position += 1
            return out

    fn has_next() -> bool:
        return this._position < this._array.length()
    
fn copy[T](src: Array[T], dst: Array[T]):
    assert(src.length() <= dst.length(), "Dst array cannot be smaller then src array")
    for i from 0 to src.length():
        dst[i] = src[i]

fn assert(cond: Bool, message: String):
    if !cond:
        panic message
```

## Misc language details

### Builtins

There are several build-in types:

1.  `Array[T]`, a fixed-size array containing items of type `T`
2.  `Object`, an interface containing no methods and fields
3.  `Int`, a integer (usually 64 bits)
4.  `Float`, a floating-point (usually 64 bits)
5.  `Bool`, a true or false value
6.  `Nil`, which means basically the same thing as Python's `None`
7.  `Empty`, which is a type containing no values.
8.  `String`, an opaque UTF-8 string. That is, string indexing and iteration
    is not supported.
9.  `Byte`, an 8-bit unsigned int

There are also several build-in values:

1.  `nil`
2.  `true` and `false`

Within a method, the following builtins are also defined:

1.  The `This` type, a special generic type referring to the current instance type.
2.  The `this` variable, which refers to the current instance.

### Scoping and declarations

Other language decisions:

1.  Scoping is per block, not per function.
2.  Variables must be assigned a value upon declaration.

### Comparisons

Some method names are special and must be implemented according to a specific
type signature. Specifically:

1.  The `equals` method must always be `func equals(other: Self) -> Bool`.
2.  The `hash` method must always be `func hash() -> Int`.
3.  The `less_than` method must always be `func less_than(other: Self) -> Bool`

Other rules:

1.  If `hash` is implemented, `equals` must also be implemented
2.  If `less_than` is implemented, `equals` must also be implemented. That is,
    we only support total ordering, not partial totaling. (The goal is partly
    to ensure we can cleanly transpile to a maximal subset of languages, and
    partly to avoid falling into this trap: https://arxiv.org/abs/1911.12338)

We add support for the following syntactic sugar:

1.  Implementing `equals` adds syntatic sugar for `a == b` and `a != b`.
2.  Implementing `less_than` (and therefore also `equals`) adds syntactic
    sugar for using `a < b`, `a <= b`, `a > b`, and `a >= b`.

### Tuples

Tuples have relatively limited power in the language. In particular:

1.  Tuples are not iterable or indexable. Instead, the syntax is `tup.0`, `tup.1`, etc.
2.  The empty tuple is identical to the unit type.
3.  There is no way of constructing a 1-element tuple.
4.  Tuples must have a fixed length.
5.  There is no way of dynamically constructing a tuple (e.g. from a list).
