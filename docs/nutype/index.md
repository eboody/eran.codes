<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/nutype/latest/nutype/index.html -->
<!-- Crawl timestamp: 2026-02-26T18:09:53Z -->

[ Docs.rs ][1]

  * [ nutype-0.6.2 ][2]

    * nutype 0.6.2 
    * [ Permalink ][3]
    * [ Docs.rs crate page ][4]
    * [MIT][5]

    * Links
    * [ Homepage ][6]
    * [ Repository ][6]
    * [ crates.io ][7]
    * [ Source ][8]

    * Owners
    * [ greyblake ][9]

    * Dependencies
    *       * [ nutype_macros ^0.6.2 _normal_ ][10]
      * [ lazy_static ^1.0 _dev_ ][11]
      * [ once_cell ^1.0 _dev_ ][12]
      * [ regex ^1.0 _dev_ ][13]

    * Versions
    *     * [ **100%** of the crate is documented ][14]

  * [ Platform ][15]
    * [aarch64-apple-darwin][16]
    * [aarch64-unknown-linux-gnu][17]
    * [i686-pc-windows-msvc][18]
    * [x86_64-pc-windows-msvc][19]
    * [x86_64-unknown-linux-gnu][20]
  * [ Feature flags ][21]



  * [docs.rs][15]
    * [ About docs.rs][22]
    * [ Badges][23]
    * [ Builds][24]
    * [ Metadata][25]
    * [ Shorthand URLs][26]
    * [ Download][27]
    * [ Rustdoc JSON][28]
    * [ Build queue][29]
    * [ Privacy policy][30]


  * [Rust][15]
    * [Rust website][31]
    * [The Book][32]
    * [Standard Library API Reference][33]
    * [Rust by Example][34]
    * [The Cargo Guide][35]
    * [Clippy Documentation][36]



## [Crate nutype][15]

## [nutype][37]0.6.2

  * [All Items][38]



### [Sections][15]

  * [Quick start][39]
  * [A few more examples][40]
  * [Inner types][41]
  * [String][42]
    * [String sanitizers][43]
    * [String validators][44]
    * [String derivable traits][45]
  * [Integer][46]
    * [Integer sanitizers][47]
    * [Integer validators][48]
    * [Integer derivable traits][49]
  * [Float][50]
    * [Float sanitizers][51]
    * [Float validators][52]
    * [Float derivable traits][53]
  * [Other inner types and generics][54]
  * [Custom sanitizers][55]
  * [Custom validation with predicate][56]
  * [Custom validation with a custom error type][57]
  * [Deriving Traits][58]
    * [`derive`][59]
    * [`derive_unsafe`][60]
  * [Constants][61]
  * [Recipes][62]
    * [Obtaining a reference to the inner value][63]
    * [Derive `Default`][64]
    * [Derive `Eq` and `Ord` on float types][65]
  * [How to break the constraints?][66]
  * [Feature flags][67]
  * [Support Ukrainian military forces 🇺🇦][68]



### [Crate Items][69]

  * [Attribute Macros][70]



# Crate nutype Copy item path

[Source][71]

Expand description

![Rust Nutype Logo][72]

## The newtype with guarantees.

Nutype is a proc macro that allows adding extra constraints like _sanitization_ and _validation_ to the regular [newtype pattern][73]. The generated code makes it impossible to instantiate a value without passing the checks. It works this way even with `serde` deserialization.

### [§][74]Quick start
``` 
use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, PartialEq),
)]
pub struct Username(String);

// Now we can create usernames:
assert_eq!(
    Username::try_new("   FooBar  ").unwrap().into_inner(),
    "foobar"
);

// But we cannot create invalid ones:
assert_eq!(
    Username::try_new("   "),
    Err(UsernameError::NotEmptyViolated),
);

assert_eq!(
    Username::try_new("TheUserNameIsVeryVeryLong"),
    Err(UsernameError::LenCharMaxViolated),
);
```

Note, that we also got `UsernameError` enum generated implicitly.

Ok, but let’s try to obtain an instance of `Username` that violates the validation rules:

[ⓘ][75]
```
let username = Username("".to_string())

// error[E0423]: cannot initialize a tuple struct which contains private fields
```

[ⓘ][75]
```
let mut username = Username::try_new("foo").unwrap();
username.0 = "".to_string();

// error[E0616]: field `0` of struct `Username` is private
```

Haha. It’s does not seem to be easy!

### [§][76]A few more examples

Here are some other examples of what you can do with `nutype`.

You can skip `sanitize` and use a custom validator `predicate`:
``` 
use nutype::nutype;

#[nutype(validate(predicate = |n| n % 2 == 1))]
struct OddNumber(i64);
```

You can skip validation, if you need sanitization only:
``` 
use nutype::nutype;

#[nutype(sanitize(trim, lowercase))]
struct Username(String);
```

In that case, `Username::new(String)` simply returns `Username`, not `Result`.

### [§][77]Inner types

Available sanitizers, validators, and derivable traits are determined by the inner type, which falls into the following categories:

  * String
  * Integer (`u8`, `u16`,`u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `usize`, `isize`)
  * Float (`f32`, `f64`)
  * Any other arbitrary type



### [§][78]String

At the moment the string inner type supports only `String` (owned) type.

#### [§][79]String sanitizers

Sanitizer| Description| Example  
---|---|---  
`trim`| Removes leading and trailing whitespaces| `trim`  
`lowercase`| Converts the string to lowercase| `lowercase`  
`uppercase`| Converts the string to uppercase| `uppercase`  
`with`| Custom sanitizer. A function or closure that receives `String` and returns `String`| `with = |mut s: String| { s.truncate(5); s }`  
  
#### [§][80]String validators

Validator| Description| Error variant| Example  
---|---|---|---  
`len_char_min`| Min length of the string (in chars, not bytes)| `LenCharMinViolated`| `len_char_min = 5`  
`len_char_max`| Max length of the string (in chars, not bytes)| `LenCharMaxViolated`| `len_char_max = 255`  
`not_empty`| Rejects an empty string| `NotEmptyViolated`| `not_empty`  
`regex`| Validates format with a regex. Requires `regex` feature.| `RegexViolated`| `regex = "^[0-9]{7}$"` or `regex = ID_REGEX`  
`predicate`| Custom validator. A function or closure that receives `&str` and returns `bool`| `PredicateViolated`| `predicate = |s: &str| s.contains('@')`  
`with`| Custom validator with a custom error| N/A| (see example below)  
  
##### [§][81]Regex validation

Requirements:

  * `regex` feature of `nutype` is enabled.
  * You crate have to explicitly include `regex` as a dependency.



There are a number of ways you can use regex.

A regular expression can be defined right in place:
``` 
use nutype::nutype;

#[nutype(validate(regex = "^[0-9]{3}-[0-9]{3}$"))]
pub struct PhoneNumber(String);

```

or it can be defined with `std::sync::LazyLock`:
``` 
use nutype::nutype;
use std::sync::LazyLock;
use regex::Regex;

static PHONE_NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[0-9]{3}-[0-9]{3}$").unwrap());

#[nutype(validate(regex = PHONE_NUMBER_REGEX))]
pub struct PhoneNumber(String);

```

or it can be defined with `lazy_static`:
``` 
use nutype::nutype;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new("^[0-9]{3}-[0-9]{3}$").unwrap();
}

#[nutype(validate(regex = PHONE_NUMBER_REGEX))]
pub struct PhoneNumber(String);

```

or `once_cell`:
``` 
use nutype::nutype;
use once_cell::sync::Lazy;
use regex::Regex;

static PHONE_NUMBER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("[0-9]{3}-[0-9]{3}$").unwrap());

#[nutype(validate(regex = PHONE_NUMBER_REGEX))]
pub struct PhoneNumber(String);

```

#### [§][82]String derivable traits

The following traits can be derived for a string-based type: `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `FromStr`, `AsRef`, `Deref`, `From`, `TryFrom`, `Into`, `Hash`, `Borrow`, `Display`, `Default`, `Serialize`, `Deserialize`.

### [§][83]Integer

The integer inner types are: `u8`, `u16`,`u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `usize`, `isize`.

#### [§][84]Integer sanitizers

Sanitizer| Description| Example  
---|---|---  
`with`| Custom sanitizer.| `with = |raw| raw.clamp(0, 100)`  
  
#### [§][85]Integer validators

Validator| Description| Error variant| Example  
---|---|---|---  
`less`| Exclusive upper bound| `LessViolated`| `less = 100`  
`less_or_equal`| Inclusive upper bound| `LessOrEqualViolated`| `less_or_equal = 99`  
`greater`| Exclusive lower bound| `GreaterViolated`| `greater = 17`  
`greater_or_equal`| Inclusive lower bound| `GreaterOrEqualViolated`| `greater_or_equal = 18`  
`predicate`| Custom predicate| `PredicateViolated`| `predicate = |num| num % 2 == 0`  
`with`| Custom validator with a custom error| N/A| (see example below)  
  
#### [§][86]Integer derivable traits

The following traits can be derived for an integer-based type: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `FromStr`, `AsRef`, `Deref`, `Into`, `From`, `TryFrom`, `Hash`, `Borrow`, `Display`, `Default`, `Serialize`, `Deserialize`.

### [§][87]Float

The float inner types are: `f32`, `f64`.

#### [§][88]Float sanitizers

Sanitizer| Description| Example  
---|---|---  
`with`| Custom sanitizer.| `with = |val| val.clamp(0.0, 100.0)`  
  
#### [§][89]Float validators

Validator| Description| Error variant| Example  
---|---|---|---  
`less`| Exclusive upper bound| `LessViolated`| `less = 100.0`  
`less_or_equal`| Inclusive upper bound| `LessOrEqualViolated`| `less_or_equal = 100.0`  
`greater`| Exclusive lower bound| `GreaterViolated`| `greater = 0.0`  
`greater_or_equal`| Inclusive lower bound| `GreaterOrEqualViolated`| `greater_or_equal = 0.0`  
`finite`| Check against NaN and infinity| `FiniteViolated`| `finite`  
`predicate`| Custom predicate| `PredicateViolated`| `predicate = |val| val != 50.0`  
`with`| Custom validator with a custom error| N/A| (see example below)  
  
#### [§][90]Float derivable traits

The following traits can be derived for a float-based type: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `FromStr`, `AsRef`, `Deref`, `Into`, `From`, `TryFrom`, `Hash`, `Borrow`, `Display`, `Default`, `Serialize`, `Deserialize`.

It’s also possible to derive `Eq` and `Ord` if the validation rules guarantee that `NaN` is excluded. This can be done by applying `finite` validation. For example:
``` 
use nutype::nutype;

#[nutype(
    validate(finite),
    derive(PartialEq, Eq, PartialOrd, Ord),
)]
struct Size(f64);
```

### [§][91]Other inner types and generics

For any other type it is possible to define custom sanitizers with `with` and custom validations with `predicate` or `with`:
``` 
use nutype::nutype;

#[nutype(
    derive(Debug, PartialEq, AsRef, Deref),
    sanitize(with = |mut guests| { guests.sort(); guests }),
    validate(predicate = |guests| !guests.is_empty() ),
)]
pub struct GuestList(Vec<String>);

```

It’s also possible to use generics:
``` 
use nutype::nutype;

#[nutype(
    sanitize(with = |mut v| { v.sort(); v }),
    validate(predicate = |vec| !vec.is_empty()),
    derive(Debug, PartialEq, AsRef, Deref),
)]
struct SortedNotEmptyVec<T: Ord>(Vec<T>);

let wise_friends = SortedNotEmptyVec::try_new(vec!["Seneca", "Zeno", "Plato"]).unwrap();
assert_eq!(wise_friends.as_ref(), &["Plato", "Seneca", "Zeno"]);
assert_eq!(wise_friends.len(), 3);

let numbers = SortedNotEmptyVec::try_new(vec![4, 2, 7, 1]).unwrap();
assert_eq!(numbers.as_ref(), &[1, 2, 4, 7]);
assert_eq!(numbers.len(), 4);
```

### [§][92]Custom sanitizers

You can set custom sanitizers using the `with` option. A custom sanitizer is a function or closure that receives a value of an inner type with ownership and returns a sanitized value back.

For example, this one
``` 
use nutype::nutype;

fn new_to_old(s: String) -> String {
    s.replace("New", "Old")
}

#[nutype(sanitize(with = new_to_old))]
struct CityName(String);

```

is equal to the following one:
``` 
use nutype::nutype;

#[nutype(sanitize(with = |s| s.replace("New", "Old") ))]
struct CityName(String);

// And works the same way:
let city = CityName::new("New York");
assert_eq!(city.into_inner(), "Old York");
```

### [§][93]Custom validation with predicate

In similar fashion it’s possible to define custom validators, but a validation function receives a reference and returns `bool`. Think of it as a predicate.
``` 
use nutype::nutype;

#[nutype(validate(predicate = is_valid_name))]
pub struct Name(String);

fn is_valid_name(name: &str) -> bool {
    // A fancy way to verify if the first character is uppercase
    name.chars().next().map(char::is_uppercase).unwrap_or(false)
}

fn main() { }
```

### [§][94]Custom validation with a custom error type

To define your own error type and implement custom validation logic, you can combine the `with` and `error` attributes:
``` 
use nutype::nutype;

// Define a custom error type for validation failures.
// Although it's best practice to implement `std::error::Error` for custom error types,
// we are omitting that for simplicity here.
#[derive(Debug, PartialEq)]
enum NameError {
    TooShort,
    TooLong,
}

// Define a custom validation function for `Name`.
// The function returns `Result<(), NameError>`, where `Ok(())` indicates a valid name,
// and `Err(NameError)` represents a specific validation failure.
fn validate_name(name: &str) -> Result<(), NameError> {
    if name.len() < 3 {
        Err(NameError::TooShort)
    } else if name.len() > 10 {
        Err(NameError::TooLong)
    } else {
        Ok(())
    }
}

// Define a newtype `Name` with custom validation logic and custom error.
#[nutype(
    validate(with = validate_name, error = NameError),
    derive(Debug, PartialEq),
)]
struct Name(String);
```

It’s important to ensure that the type specified in the `error` attribute matches the error type returned by the validation function.

### [§][95]Deriving Traits

There are two ways to derive traits for a `nutype`.

#### [§][96]`derive`

The recommended approach is to use the `derive(..)` attribute within the `#[nutype(..)]` macro:
``` 
use nutype::nutype;

#[nutype(derive(Debug))]
pub struct Username(String);
```

When using `derive`, `nutype` ensures that the derived traits do not compromise the type’s invariants (i.e., validation constraints).

However, this approach has a limitation: only a predefined set of traits is supported. Deriving arbitrary third-party traits is not allowed via `derive`.

#### [§][97]`derive_unsafe`

To overcome this limitation, you can use the `derive_unsafe(..)` attribute (requires the corresponding feature flag to be enabled):
``` 
use nutype::nutype;

#[nutype(derive_unsafe(std::fmt::Debug))]
pub struct Username(String);
```

This enables deriving arbitrary traits, including those from third-party crates. However, **use this with caution** : `nutype` cannot verify that these traits preserve the invariants of the type. It is the developer’s responsibility to ensure that the derived traits do not introduce ways to bypass validation (e.g., by allowing mutable access to the inner value).

### [§][98]Constants

You can mark a type with the `const_fn` flag. In that case, its `new` and `try_new` functions will be declared as `const`:
``` 
use nutype::nutype;

#[nutype(
    const_fn,
    derive(Debug),
    validate(greater_or_equal = -273.15),
)]
pub struct Celsius(f64);

// Since `Result::unwrap()` is not allowed in `const` contexts,
// we must manually handle the `Result` when creating constants.
// Any attempt to instantiate an invalid `Celsius` at compile time
// will trigger a compilation error:
const FREEZING_POINT: Celsius = match Celsius::try_new(0.0) {
    Ok(value) => value,
    Err(_) => panic!("Invalid value"),
};

assert_eq!(FREEZING_POINT.into_inner(), 0.0);

// Alternatively, you can use a helper macro like this:
macro_rules! nutype_const {
    ($name:ident, $ty:ty, $value:expr) => {
        const $name: $ty = match <$ty>::try_new($value) {
            Ok(value) => value,
            Err(_) => panic!("Invalid value"),
        };
    };
}

nutype_const!(WATER_BOILING_POINT, Celsius, 100.0);

assert_eq!(WATER_BOILING_POINT.into_inner(), 100.0);
```

Note that `const` works only for stack allocated types. If you are dealing with a heap allocated type (e.g. `String`) you should consider using `static` with [`LazyLock`][99].

### [§][100]Recipes

#### [§][101]Obtaining a reference to the inner value

The function `.into_inner()` takes ownership of the newtype and returns its inner type. However, if you only need to borrow the inner value (rather than consume it), you can derive `AsRef`. This allows you to call `as_ref()` to obtain a reference to the underlying data:
``` 
use nutype::nutype;

#[nutype(derive(AsRef))]
struct Username(String);

let username = Username::new("Jack");
assert_eq!(username.as_ref(), "Jack");
```

#### [§][102]Derive `Default`
``` 
use nutype::nutype;

#[nutype(
    derive(Default),
    default = "Anonymous",
)]
pub struct Name(String);
```

#### [§][103]Derive `Eq` and `Ord` on float types

With nutype it’s possible to derive `Eq` and `Ord` if there is `finite` validation set. The `finite` validation ensures that the valid value excludes `NaN`.
``` 
use nutype::nutype;

#[nutype(
    validate(finite),
    derive(PartialEq, Eq, PartialOrd, Ord),
)]
pub struct Weight(f64);
```

### [§][104]How to break the constraints?

It’s discouraged, but it’s possible to bypass the constraints by enabling `new_unchecked` crate feature and marking a type with `new_unchecked`:
``` 
use nutype::nutype;

#[nutype(
    new_unchecked,
    sanitize(trim),
    validate(len_char_min = 8),
)]
pub struct Name(String);

// Yes, you're forced to use `unsafe` here, so everyone will point fingers at YOU.
let name = unsafe { Name::new_unchecked(" boo ".to_string()) };

// `name` violates the sanitization and validation rules!!!
assert_eq!(name.into_inner(), " boo ");
```

### [§][105]Feature flags

  * `arbitrary` \- enables derive of [`arbitrary::Arbitrary`][106].
  * `derive_unsafe` \- enables `derive_unsafe` attribute to derive any arbitrary trait.
  * `new_unchecked` \- enables generation of unsafe `::new_unchecked()` function.
  * `regex` \- allows to use `regex = ` validation on string-based types. Note: your crate also has to explicitly have `regex` within its dependencies.
  * `serde` \- integrations with [`serde`][107] crate. Allows to derive `Serialize` and `Deserialize` traits.
  * `schemars08` \- allows to derive [`JsonSchema`][108] trait of [schemars][109] crate. Note that at the moment validation rules are not respected.
  * `std` \- enabled by default. Use `default-features = false` to disable.



### [§][110]Support Ukrainian military forces 🇺🇦

Today I live in Berlin, I have the luxury to live a physically safe life. But I am Ukrainian. The first 25 years of my life I spent in [Kharkiv][111], the second-largest city in Ukraine, 60km away from the border with russia. Today about [a third of my home city is destroyed][112] by russians. My parents, my relatives and my friends had to survive the artillery and air attack, living for over a month in basements.

Some of them have managed to evacuate to EU. Some others are trying to live “normal lives” in Kharkiv, doing there daily duties. And some are at the front line right now, risking their lives every second to protect the rest.

I encourage you to donate to [Charity foundation of Serhiy Prytula][113]. Just pick the project you like and donate. This is one of the best-known foundations, you can watch a [little documentary][114] about it. Your contribution to the Ukrainian military force is a contribution to my calmness, so I can spend more time developing the project.

Thank you.

## Attribute Macros[§][69]

[nutype][115]
    Defines sanitizers and validators on a newtype. Guarantees that the type can be instantiated only with valid values. See the documentation for [nutype][116] crate for more information.

   [1]: https://docs.rs/
   [2]: index.html# (The newtype with guarantees.)
   [3]: https://docs.rs/nutype/0.6.2/nutype/ (Get a link to this specific version)
   [4]: https://docs.rs/crate/nutype/latest (See nutype in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/greyblake/nutype
   [7]: https://crates.io/crates/nutype (See nutype in crates.io)
   [8]: https://docs.rs/crate/nutype/latest/source/ (Browse source of nutype-0.6.2)
   [9]: https://crates.io/users/greyblake
   [10]: https://docs.rs/nutype_macros/^0.6.2/
   [11]: https://docs.rs/lazy_static/^1.0/
   [12]: https://docs.rs/once_cell/^1.0/
   [13]: https://docs.rs/regex/^1.0/
   [14]: https://docs.rs/crate/nutype/latest
   [15]: index.html#
   [16]: https://docs.rs/crate/nutype/latest/target-redirect/aarch64-apple-darwin/nutype/
   [17]: https://docs.rs/crate/nutype/latest/target-redirect/aarch64-unknown-linux-gnu/nutype/
   [18]: https://docs.rs/crate/nutype/latest/target-redirect/i686-pc-windows-msvc/nutype/
   [19]: https://docs.rs/crate/nutype/latest/target-redirect/x86_64-pc-windows-msvc/nutype/
   [20]: https://docs.rs/crate/nutype/latest/target-redirect/nutype/
   [21]: https://docs.rs/crate/nutype/latest/features (Browse available feature flags of nutype-0.6.2)
   [22]: https://docs.rs/about
   [23]: https://docs.rs/about/badges
   [24]: https://docs.rs/about/builds
   [25]: https://docs.rs/about/metadata
   [26]: https://docs.rs/about/redirections
   [27]: https://docs.rs/about/download
   [28]: https://docs.rs/about/rustdoc-json
   [29]: https://docs.rs/releases/queue
   [30]: https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs
   [31]: https://www.rust-lang.org/
   [32]: https://doc.rust-lang.org/book/
   [33]: https://doc.rust-lang.org/std/
   [34]: https://doc.rust-lang.org/rust-by-example/
   [35]: https://doc.rust-lang.org/cargo/guide/
   [36]: https://doc.rust-lang.org/nightly/clippy
   [37]: index.html
   [38]: all.html
   [39]: index.html#quick-start (Quick start)
   [40]: index.html#a-few-more-examples (A few more examples)
   [41]: index.html#inner-types (Inner types)
   [42]: index.html#string (String)
   [43]: index.html#string-sanitizers (String sanitizers)
   [44]: index.html#string-validators (String validators)
   [45]: index.html#string-derivable-traits (String derivable traits)
   [46]: index.html#integer (Integer)
   [47]: index.html#integer-sanitizers (Integer sanitizers)
   [48]: index.html#integer-validators (Integer validators)
   [49]: index.html#integer-derivable-traits (Integer derivable traits)
   [50]: index.html#float (Float)
   [51]: index.html#float-sanitizers (Float sanitizers)
   [52]: index.html#float-validators (Float validators)
   [53]: index.html#float-derivable-traits (Float derivable traits)
   [54]: index.html#other-inner-types-and-generics (Other inner types and generics)
   [55]: index.html#custom-sanitizers (Custom sanitizers)
   [56]: index.html#custom-validation-with-predicate (Custom validation with predicate)
   [57]: index.html#custom-validation-with-a-custom-error-type (Custom validation with a custom error type)
   [58]: index.html#deriving-traits (Deriving Traits)
   [59]: index.html#derive (`derive`)
   [60]: index.html#derive_unsafe (`derive_unsafe`)
   [61]: index.html#constants (Constants)
   [62]: index.html#recipes (Recipes)
   [63]: index.html#obtaining-a-reference-to-the-inner-value (Obtaining a reference to the inner value)
   [64]: index.html#derive-default (Derive `Default`)
   [65]: index.html#derive-eq-and-ord-on-float-types (Derive `Eq` and `Ord` on float types)
   [66]: index.html#how-to-break-the-constraints (How to break the constraints?)
   [67]: index.html#feature-flags (Feature flags)
   [68]: index.html#support-ukrainian-military-forces- (Support Ukrainian military forces 🇺🇦)
   [69]: index.html#attributes
   [70]: index.html#attributes (Attribute Macros)
   [71]: https://docs.rs/nutype/latest/src/nutype/lib.rs.html#1-612
   [72]: https://raw.githubusercontent.com/greyblake/nutype/master/art/rust_nutype.png
   [73]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
   [74]: index.html#quick-start
   [75]: index.html# (This example is not tested)
   [76]: index.html#a-few-more-examples
   [77]: index.html#inner-types
   [78]: index.html#string
   [79]: index.html#string-sanitizers
   [80]: index.html#string-validators
   [81]: index.html#regex-validation
   [82]: index.html#string-derivable-traits
   [83]: index.html#integer
   [84]: index.html#integer-sanitizers
   [85]: index.html#integer-validators
   [86]: index.html#integer-derivable-traits
   [87]: index.html#float
   [88]: index.html#float-sanitizers
   [89]: index.html#float-validators
   [90]: index.html#float-derivable-traits
   [91]: index.html#other-inner-types-and-generics
   [92]: index.html#custom-sanitizers
   [93]: index.html#custom-validation-with-predicate
   [94]: index.html#custom-validation-with-a-custom-error-type
   [95]: index.html#deriving-traits
   [96]: index.html#derive
   [97]: index.html#derive_unsafe
   [98]: index.html#constants
   [99]: https://doc.rust-lang.org/beta/std/sync/struct.LazyLock.html
   [100]: index.html#recipes
   [101]: index.html#obtaining-a-reference-to-the-inner-value
   [102]: index.html#derive-default
   [103]: index.html#derive-eq-and-ord-on-float-types
   [104]: index.html#how-to-break-the-constraints
   [105]: index.html#feature-flags
   [106]: https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html
   [107]: https://crates.io/crates/serde
   [108]: https://docs.rs/schemars/0.8.12/schemars/trait.JsonSchema.html
   [109]: https://crates.io/crates/schemars
   [110]: index.html#support-ukrainian-military-forces-
   [111]: https://en.wikipedia.org/wiki/Kharkiv
   [112]: https://www.youtube.com/watch?v=ihoufBFSZds
   [113]: https://prytulafoundation.org/en
   [114]: https://www.youtube.com/watch?v=VlmWqoeub1Q
   [115]: attr.nutype.html (attr nutype::nutype)
   [116]: https://docs.rs/nutype

