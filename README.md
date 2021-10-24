# Impo

Impo -- a minimalist language designed to be transpiled into mainstream imperative
and object-oriented languages.

To help make Impo easy to transpile, the language supports a very minimal featureset:
effectively the lowest common denominator of the languages it transpiles.

The language supports the following:

1.  Garbage collection
2.  Basic imperative control flow features
3.  Generics
4.  Fixed-sized arrays
5.  Static type checking (with type inference -- only function signatures and fields
    require type annotations)

The language enforces the following conventions:

1.  Function and method names should be `lower_camel_case`
2.  Class names should be `UpperCamelCase`
3.  Fields and methods that are intended to be private must be prefixed with an underscore.

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

3.  Covariance or contravariance -- generics _must_ be invariant. Again, this helps
    simplify the language design.

4.  Statements may be preceded by "hints" which take the form `!key: value`.
    These are ignored by the language, but are attached to the AST node for use by
    the transpiler.

    Hints attached to interface definitions are always applied to the respective
    methods in the implementing classes.

5.  Neither operator overloading nor optional arguments are supported.
    (TODO: Revisit)

6.  Some method names are special and must be implemented according to a specific
    type signature. For example, the `equals` method must always match the type
    signature `func equals(other: Object) -> Bool`.

Things that need to be revisited:

-   Exceptions vs panics vs returning a sentinal


## Language spec

```
const _DEFAULT_CAPACITY = 8

interface List[T]:
    fn get_item(index: Int) -> T
    fn set_item(index: Int, value: T)
    fn append(value: T)
    fn length() -> Int
    fn equals(other: Object) -> Bool

sentinal IteratorDone

interface Iterator[T]:
    fn next() -> T | IteratorDone

    !Java: Omit()
    fn has_next() -> bool


class ArrayList[T] implements List[T]:
    _array: Array[T]
    _length: Int

    constructor():
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

    # TODO: Figure out this corner of the language
    fn equals(other: Object) -> Bool:
        if !(this instanceof ArrayList[T]):
            return false
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

There are several build-in types:

1.  `Array[T]`, a fixed-size array containing items of type `T`
2.  `Object`, an interface containing no methods and fields
3.  `Int`, a integer (usually 64 bits)
4.  `Float`, a floating-point (usually 64 bits)
5.  `Bool`, a true or false value
6.  `None`, which means basically the same thing as Python's `None`
7.  `Empty`, which is a type containing no values.
8.  `String`, an opaque UTF-8 string. That is, string indexing and iteration
    is not supported.
9.  `Byte`, an 8-bit unsigned int

There are also several build-in values:

1.  `none`
2.  `true` and `false`

Other language decisions:

1.  Scoping is per block, not per function.
2.  Variables must be assigned a value upon declaration.
