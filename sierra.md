# Sierra

## Structure

### types

For now only numeric and boolean types are supported

#### Structs

Cairo syntax

```rs
struct FibResult { value: felt, index: felt, nothing: () }
```

Sierra syntax

```rs
type fib_struct::fib_struct::FibResult = Struct<ut@fib_struct::fib_struct::FibResult, felt, felt, Unit>;
```

Unit is considered as an empty  struct: `type Unit = Struct<ut@Tuple>;`
Ut stands for user type which means that it has been declared by the user and is
not part of the core lib

#### Boxes

Cairo syntax

```rs
a: Box::<felt>
```

Sierra syntax

```rs
type Box<felt> = Box<felt>;
```

I assume that if we could declare a type alias it'd go like that but it doesn't
seem to be possible for now

```rs
type custom = Box<felt>;
```

There's nothing added to the casm code it's really just for the compiler

#### Arrays

Cairo syntax

```rs
Array::<felt>
```

Sierra syntax

```rs
type Array<felt> = Array<felt>;
```

Arrays in casm are handled as they were before. Right now only appending the
array is supported. When popping will be supported most likely a full copy of
the array will be done on another segment without the popped element

#### Enums

Cairo syntax

```rs
enum MyEnumShort { a: felt, b: felt }
enum MyEnumLong { a: felt, b: felt, c: felt }
enum MyEnumGeneric<S, T> { a: T, b: S, c: T }
```

Sierra syntax

```rs
type enum_flow::enum_flow::MyEnumShort = Enum<ut@enum_flow::enum_flow::MyEnumShort, felt, felt>;
type enum_flow::enum_flow::MyEnumLong = Enum<ut@enum_flow::enum_flow::MyEnumLong, felt, felt, felt>;
type Unit = Struct<ut@Tuple>;
type enum_flow::enum_flow::MyEnumGeneric::<(), core::felt> = Enum<ut@enum_flow::enum_flow::MyEnumGeneric::<(), core::felt>, felt, Unit, felt>;
```

Here enum generic is filled with the later declared types in the cairo code

```rs
let eg1: MyEnumGeneric::<(), felt> = MyEnumGeneric::<(), felt>::a(30);
let eg2: MyEnumGeneric::<(), felt> = MyEnumGeneric::<(), felt>::b(());
let eg3: MyEnumGeneric::<(), felt> = MyEnumGeneric::<(), felt>::c(32);
```

Enums, like structs are converted to tuples

#### Panicable

Panics are declared with the types and seem to be added by the compiler while
encountering any `panic` function call. The data displayed by panic seems to be
a felt array (most likely to display a long string).
The panic data type is a regular enum

Sierra syntax

```rs
type core::PanicResult::<core::felt> = Enum<ut@core::PanicResult::<core::felt>, felt, Array<felt>>;
```

#### Builtins

Custom implicit arguments have disappeared. Only the builtins are implicit
(even more than before). You now only have to declare the builtin type you need
for your function and the compiler will handle everything (even the revoked references)

example:

```rs
// Calculates fib...
fn fib() implicits(RangeCheck, GasBuiltin) {
}
```

Sierra

```rs
type RangeCheck = RangeCheck;
type GasBuiltin = GasBuiltin;
```

#### Booleans

I assume they are still felts 0/1 but not sure why it's stored in this
`type core::bool = Enum<ut@core::bool, Unit, Unit>;` enum

### lib functions

The lib functions are all the functions that will be called during the execution.
It contains all the functions that will be called during the execution even the
functions defined by the user.

Felt and u128 literals are functions for sure. For now
it seems like they're the only type that can be literals.

locals and tempvars are managed by the store_temp and store_local functions.

### list of instructions

This block is composed of instructions (all the functions declared in the lib
functions are called in the right order) each variable gets its own id (number)
This is a mix between casm and cairo.

### Functions

At the bottom of the file we have the functions types (params + return type)