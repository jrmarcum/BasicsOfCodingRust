# Basics of Coding Rust
## Preface
What this text is and what it is not: This text is intended to introduce the reader to the basics of the Rust programming language in the sense that they will be able to write minimal types of programs and run the code. It is not intended to go into advanced topics like advanced concurrency, unsafe code, macro writing, object-oriented patterns, testing and debugging techniques and other software engineering principals.

The programs are intended to be run in the terminal as that is common to most operating systems. Linux and Mac come preinstalled with a terminal. Windows may or may not have it pre-installed. "Windows Terminal" can be installed from the Microsoft store. Just do a search for it and install it.
## Installation and Setup of Rust
To setup Rust for use, navigate to the [Rust](https://www.rust-lang.org/learn/get-started) programming language website and follow the directions to install for your operating system including the dependencies. If the language has been installed properly you will be able to type the following command in the terminal command line and receive the response shown:
```
$ rustc --version
rustc 1.82.0 (f6e511eec 2024-10-15)
(Note: the version shown here varies with your installed version)
```
## How to Run Examples

Each lesson folder is a Cargo crate containing a `.rs` source file and a `.md` file showing the run command and expected output. To run any example, navigate to its folder in your terminal first, then use the command shown in the `.md` file:

```
$ cd 01_hello-world
$ cargo run
```

This project uses a Cargo workspace. You can also build all lessons at once from the root:

```
$ cargo build
```

## Comments
Comments are used to document what your code does so that others can understand it when reviewing your code. Comments also document items that are non-performant in order to debug the code items at a later date.
## Keywords
|          |        |        |        |          |
|:--------:|:------:|:------:|:------:|:--------:|
| as       | async  | await  | break  | const    |
| continue | crate  | dyn    | else   | enum     |
| extern   | false  | fn     | for    | if       |
| impl     | in     | let    | loop   | match    |
| mod      | move   | mut    | pub    | ref      |
| return   | self   | Self   | static | struct   |
| super    | trait  | true   | type   | union    |
| unsafe   | use    | where  | while  |          |
## Operators
> ### Arithmetic Operators
> |        |                                                    |
> |:------:|:---------------------------------------------------|
> | **+**  | add one number to another number                   |
> | **-**  | subtract one number from another number            |
> | **\*** | multiply one number by another number              |
> | **/**  | divide one number by another number                |
> | **%**  | remainder of dividing one number by another number |
> ### Comparison Operators
> |        |                                                         |
> |:------:|:--------------------------------------------------------|
> | **==** | check if a variable is equal to another                 |
> | **!=** | check if a variable is not equal to another             |
> | **>**  | check if a variable is greater than another             |
> | **<**  | check if a variable is less than another                |
> | **>=** | check if a variable is greater than or equal to another |
> | **<=** | check if a variable is less than or equal to another    |
> ### Logical Operators
> |          |                                                 |
> |:--------:|:------------------------------------------------|
> | **&&**   | returns true if both statements are true        |
> | **\|\|** | returns true if one of the statements are true  |
> | **!**    | reverse the result of a true or false statement |
> ### Assignment Operators
> |         |                                                                                                              |
> |:-------:|:-------------------------------------------------------------------------------------------------------------|
> | **=**   | assign a data type to a variable                                                                             |
> | **+=**  | add a number to the existing value of a variable and assign the result to variable                           |
> | **-=**  | subtract a number from the existing value of a variable and assign the result to variable                    |
> | **\*=** | multiply a number to the existing value of a variable and assign the result to variable                      |
> | **/=**  | divide the existing value of a variable by a number and assign the result to variable                        |
> | **%=**  | take the remainder of the existing value of a variable divided by a number and assign the result to variable |
## Data Types
  ###  1.  Variables
>> #### a.  **String, string, str**: used for storing text and/or characters
>> 
>> #### b.  **Char, char, Character, character**: a single character/letter/number, or ASCII values, UTF-8 code unit
>> 
>> #### c.  **wchar**: UTF-16 code unit
>> 
>> #### d.  **dchar**: UTF-32 code unit and Unicode code point
>> 
>> #### e.  **Numbers**
>>
>>> #####  1)  *Number, number, numeric*: stores numeric data with or without decimal
>>>
>>> #####  2)  *Int8, int8, i8, sbyte*: stores positive or negative integers from -2^7 to (2^7)-1, with 3 significant figure precision
>>>
>>> #####  3)  *Int16, int16, i16, Short, short*: stores positive or negative integers from -2^15 to (2^15)-1, with 5 significant figure precision
>>>
>>> #####  4)  *Int32, int32, i32, Int, int, Integer, integer*: stores positive or negative integers from -2^31 to (2^31)-1, with 10 significant figure precision
>>>
>>> #####  5)  *Int64, int64, i64, bigint, Long, long*: stores positive or negative integers from -2^63 to (2^63)-1, with 19 significant figure precision
>>>
>>> #####  6)  *Int128, i128*: stores positive or negative integers from -2^127 to (2^127)-1, with 39 significant figure precision
>>>
>>> #####  7)  *isize*: same as i32 or i64 depending on computer architecture
>>>
>>> #####  8)  *UInt8, uint8, u8, ubyte, Byte, byte, bytes*: stores positive integers from 0 to (2^8)-1, with 3 significant figure precision
>>>
>>> #####  9)  *UInt16, uint16, u16, ushort*: stores positive integers from 0 to (2^16)-1, with 5 significant figure precision
>>>
>>> ##### 10)  *UInt32, uint32, u32, uint*: stores positive integers from 0 to (2^32)-1, with 10 significant figure precision
>>>
>>> ##### 11)  *UInt64, uint64, u64, ulong*: stores positive integers from 0 to (2^64)-1, with 19 significant figure precision
>>>
>>> ##### 12)  *UInt128, u128*: stores positive integer numbers from 0 to (2^128)-1, with 39 significant figure precision
>>>
>>> ##### 13)  *usize*: same as u32 or u64 depending on computer architecture
>>>
>>> ##### 14)  *Real, real*: either the largest floating point type that the hardware supports, or double; whichever is larger
>>>
>>> ##### 15)  *Float16*: stores fractional numbers from -2^15 to (2^15)-1, with 5 significant figure precision
>>>
>>> ##### 16)  *Float32, float32, f32, Float, float*: stores fractional numbers from -2^31 to (2^31)-1, with 10 significant figure precision 
>>>
>>> ##### 17)  *Double, double, Float64, float64, f64*: stores fractional numbers from -2^63 to (2^63)-1, with 19 significant figure precision
>>>
>>> ##### 18)  *decimal*: stores numbers from -7.9E-28 to +7.9E28 (28 digits of precision)
>>>
>>> ##### 19)  *BigRational*: construction of number from an i32 numerator and i32 denominator
>>>
>>> ##### 20)  *Complex, complex, cfloat*: complex number type made of two floats
>>>
>>> ##### 21)  *cdouble*: complex number type made of two doubles
>>>
>>> ##### 22)  *creal*: complex number type made of two reals 
>>>
>>> ##### 23)  *ifloat*: imaginary value type of float
>>>
>>> ##### 24)  *idouble*: imaginary value type of double
>>>
>>> ##### 25)  *ireal*: imaginary value type of real
> ###  2. **Constants** : 
> ###  3. **Boolean, boolean, Bool, bool, Logical, logical**: values of True, true, False, false or None
> ###  4. **Lists**
> ###  5. **Arrays**      
> ###  6. **Tuples**
> ###  7. **Dictionaries**
> ###  8. **Sets**
> ###  9. **Frozen Set**
## Functions
> ###  1. **Range**: Data range function typically using in for loops
> ###  2. **Date and Time**: produces the date and time in a specified format
## Statements
A statement is an instruction that a program can execute. They are usually made line by line in your coding file. Lines in your code can have multiple statements that are separated typically by semicolons.
> ### Conditional Statements
> |             |                                                                                                     |
> |:-----------:|:----------------------------------------------------------------------------------------------------|
> | **if-else** | performs a statement 'if' a condition is met and if not (ie. 'else') performs the default statement |
> | **if-else** | performs a statement 'if' a condition is met and if not, performs a statement 'else if' a condition is met and if not (ie. 'else') performs the default statement |
> | **match**   | compares a value against a series of patterns and executes the block for the first match            |
> ### Iterative Statements
> |                   |                                                                                              |
> |:-----------------:|:---------------------------------------------------------------------------------------------|
> | **for**           | loop statement over an iterator                                                              |
> | **for-in**        | loop statement over each element in an iterator                                              |
> | **while**         | indefinite loop that is terminated while a condition is true                                 |
> | **while-let**     | while let destructures a variable into another variable and performs a block of code else breaks |
> | **if-let**        | if let destructures a variable into another variable and performs a block of code            |
> | **loop**          | indefinite loop that requires a break statement to terminate the loop                        |
> ### Transfer Statements
> |                 |                                             |
> |:---------------:|:--------------------------------------------|
> | **break**       | terminates a statement at the point of call |
> | **continue**    | continues to the next loop increment        |
> | **return**      | returns a value from a function             |
## File Input and Output
> ###  1. File Input
>> #### a. Input casting.
>>
>> #### b. Handling Errors from incorrect input types.
>>
>> #### c. json
>>
>> #### d. html
>>
>> #### e. xml
>>
>> #### f. csv
>>
> ###  2. File Output
>> #### a. String Formatting
>>
>> #### b. json
>>
>> #### c. html
>>
>> #### d. xml
>>
>> #### e. csv

## Attribution

This project is adapted in part from **[Go by Example](https://github.com/mmcgrana/gobyexample)**
by [Mark McGranaghan](https://github.com/mmcgrana), licensed under the
[Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/),
via **[Basics of Coding Go](https://github.com/jrmarcum/BasicsOfCodingGo)**
by [Jon Marcum](https://github.com/jrmarcum).

This project exists as a platform for multi-language comparative study of syntax,
language simplicity, lines of code required, and compile/run performance.

## License

This repository contains two tiers of content:

| Content | License |
| --- | --- |
| Lesson files and code examples adapted from *Go by Example* via *Basics of Coding Go* | [CC BY 3.0](http://creativecommons.org/licenses/by/3.0/) — see NOTICE |
| Original contributions by Jon Marcum (project structure, README, comparative study additions) | [CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) — see LICENSE |

The root `LICENSE` file (CC0) applies to Jon Marcum's original contributions.
The `NOTICE` file clarifies that CC BY 3.0 governs all content adapted from *Go by Example*.
