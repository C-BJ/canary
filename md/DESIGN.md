# Design of Jaye

## Functions
In Jaye functions are done differently from most other programming<br>
languages.

```
const function_name: i32(n: i32) = {
    return n
}
```

## Variables
In Jaye there is 3 ways of creating variables
```
const main: void() = {}      // Constant (type must be indicated)
var radius: i8 = 45          // Like let mut in Rust
val pi: float = 3.14159265   // Like val in Kotlin
```
You can also enable type inference on a varaible like this
```
var radius = 45
```

## Datatypes
In Jaye we do a mix of Go style and Rust style datatypes.
```
int(like isize in Rust), i8, i16, i32, i64,
uint(like usize in Rust), u8, u16, u32, u64
float(like fsize), f32, f64
```

## If statements
In Jaye there is 2 ways of doing ifs and one is normal and the<br>
other one lets you return a result into a variable
```
if(1 == 1) {
    print("Hello World")
}

val result: int = if(1 == 1) {
    _ = 1+1  //Last line
}
```

## Switch statements
In Jaye switch statements are very similar to C and Go.
```
val c: int = 123
switch(c) {
    321 => {
      print("not 123")
    }
    231 => print("not 123")
    123 => print("is 123")
    default => print("......")
}
```
