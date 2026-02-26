<!-- Generated from docs.rs rustdoc HTML: https://docs.rs/strum/latest/strum/enum.ParseError.html -->
<!-- Crawl timestamp: 2026-02-26T18:14:00Z -->

[ Docs.rs ][1]

  * [ strum-0.28.0 ][2]

    * strum 0.28.0 
    * [ Permalink ][3]
    * [ Docs.rs crate page ][4]
    * [MIT][5]

    * Links
    * [ Homepage ][6]
    * [ Repository ][6]
    * [ crates.io ][7]
    * [ Source ][8]

    * Owners
    * [ Peternator7 ][9]

    * Dependencies
    *       * [ phf ^0.13 _normal_ _optional_ ][10]
      * [ strum_macros ^0.28 _normal_ _optional_ ][11]

    * Versions
    *     * [ **41.18%** of the crate is documented ][12]

  * [ Platform ][13]
    * [aarch64-apple-darwin][14]
    * [aarch64-unknown-linux-gnu][15]
    * [i686-pc-windows-msvc][16]
    * [x86_64-pc-windows-msvc][17]
    * [x86_64-unknown-linux-gnu][18]
  * [ Feature flags ][19]



  * [docs.rs][13]
    * [ About docs.rs][20]
    * [ Badges][21]
    * [ Builds][22]
    * [ Metadata][23]
    * [ Shorthand URLs][24]
    * [ Download][25]
    * [ Rustdoc JSON][26]
    * [ Build queue][27]
    * [ Privacy policy][28]


  * [Rust][13]
    * [Rust website][29]
    * [The Book][30]
    * [Standard Library API Reference][31]
    * [Rust by Example][32]
    * [The Cargo Guide][33]
    * [Clippy Documentation][34]



[Skip to main content][35]

## [ParseError][13]

## [strum][36]0.28.0

## [ParseError][13]

### [Variants][37]

  * [VariantNotFound][38]



### [Trait Implementations][39]

  * [Clone][40]
  * [Copy][41]
  * [Debug][42]
  * [Display][43]
  * [Eq][44]
  * [Error][45]
  * [Hash][46]
  * [PartialEq][47]
  * [StructuralPartialEq][48]



### [Auto Trait Implementations][49]

  * [Freeze][50]
  * [RefUnwindSafe][51]
  * [Send][52]
  * [Sync][53]
  * [Unpin][54]
  * [UnsafeUnpin][55]
  * [UnwindSafe][56]



### [Blanket Implementations][57]

  * [Any][58]
  * [Borrow<T>][59]
  * [BorrowMut<T>][60]
  * [CloneToUninit][61]
  * [From<T>][62]
  * [Into<U>][63]
  * [ToOwned][64]
  * [ToString][65]
  * [TryFrom<U>][66]
  * [TryInto<U>][67]



## [In crate strum][36]

[strum][36]

# Enum ParseError Copy item path

[Source][68]
``` 
pub enum ParseError {
    VariantNotFound,
}
```

Expand description

The `ParseError` enum is a collection of all the possible reasons an enum can fail to parse from a string.

## Variants[§][37]

[§][69]

### VariantNotFound

## Trait Implementations[§][39]

[Source][70][§][71]

### impl [Clone][72] for [ParseError][73]

[Source][70][§][74]

#### fn [clone][75](&self) -> [ParseError][73]

Returns a duplicate of the value. [Read more][75]

1.0.0 · [Source][76][§][77]

#### fn [clone_from][78](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][78]

[Source][70][§][79]

### impl [Debug][80] for [ParseError][73]

[Source][70][§][81]

#### fn [fmt][82](&self, f: &mut [Formatter][83]<'_>) -> [Result][84]

Formats the value using the given formatter. [Read more][82]

[Source][85][§][86]

### impl [Display][87] for [ParseError][73]

[Source][88][§][89]

#### fn [fmt][90](&self, f: &mut [Formatter][83]<'_>) -> [Result][91]<[()][92], [Error][93]>

Formats the value using the given formatter. [Read more][90]

[Source][94][§][95]

### impl [Error][96] for [ParseError][73]

Available on **crate feature`std`** only.

[Source][97][§][98]

#### fn [description][99](&self) -> &[str][100]

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more][99]

1.30.0 · [Source][101][§][102]

#### fn [source][103](&self) -> [Option][104]<&(dyn [Error][96] \+ 'static)>

Returns the lower-level source of this error, if any. [Read more][103]

1.0.0 · [Source][105][§][106]

#### fn [cause][107](&self) -> [Option][104]<&dyn [Error][96]>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source][108][§][109]

#### fn [provide][110]<'a>(&'a self, request: &mut [Request][111]<'a>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more][110]

[Source][70][§][112]

### impl [Hash][113] for [ParseError][73]

[Source][70][§][114]

#### fn [hash][115]<__H: [Hasher][116]>(&self, state: [&mut __H][117])

Feeds this value into the given [`Hasher`][116]. [Read more][115]

1.3.0 · [Source][118][§][119]

#### fn [hash_slice][120]<H>(data: &[Self], state: [&mut H][117])

where H: [Hasher][116], Self: [Sized][121],

Feeds a slice of this type into the given [`Hasher`][116]. [Read more][120]

[Source][70][§][122]

### impl [PartialEq][123] for [ParseError][73]

[Source][70][§][124]

#### fn [eq][125](&self, other: &[ParseError][73]) -> [bool][126]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][127][§][128]

#### fn [ne][129](&self, other: [&Rhs][117]) -> [bool][126]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][70][§][130]

### impl [Copy][131] for [ParseError][73]

[Source][70][§][132]

### impl [Eq][133] for [ParseError][73]

[Source][70][§][134]

### impl [StructuralPartialEq][135] for [ParseError][73]

## Auto Trait Implementations[§][49]

[§][136]

### impl [Freeze][137] for [ParseError][73]

[§][138]

### impl [RefUnwindSafe][139] for [ParseError][73]

[§][140]

### impl [Send][141] for [ParseError][73]

[§][142]

### impl [Sync][143] for [ParseError][73]

[§][144]

### impl [Unpin][145] for [ParseError][73]

[§][146]

### impl [UnsafeUnpin][147] for [ParseError][73]

[§][148]

### impl [UnwindSafe][149] for [ParseError][73]

## Blanket Implementations[§][57]

[Source][150][§][151]

### impl<T> [Any][152] for T

where T: 'static + ?[Sized][121],

[Source][153][§][154]

#### fn [type_id][155](&self) -> [TypeId][156]

Gets the `TypeId` of `self`. [Read more][155]

[Source][157][§][158]

### impl<T> [Borrow][159]<T> for T

where T: ?[Sized][121],

[Source][160][§][161]

#### fn [borrow][162](&self) -> [&T][117]

Immutably borrows from an owned value. [Read more][162]

[Source][163][§][164]

### impl<T> [BorrowMut][165]<T> for T

where T: ?[Sized][121],

[Source][166][§][167]

#### fn [borrow_mut][168](&mut self) -> [&mut T][117]

Mutably borrows from an owned value. [Read more][168]

[Source][169][§][170]

### impl<T> [CloneToUninit][171] for T

where T: [Clone][72],

[Source][172][§][173]

#### unsafe fn [clone_to_uninit][174](&self, dest: [*mut ][175][u8][176])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][174]

[Source][177][§][178]

### impl<T> [From][179]<T> for T

[Source][180][§][181]

#### fn [from][182](t: T) -> T

Returns the argument unchanged.

[Source][183][§][184]

### impl<T, U> [Into][185]<U> for T

where U: [From][179]<T>,

[Source][186][§][187]

#### fn [into][188](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][179]<T> for U` chooses to do.

[Source][189][§][190]

### impl<T> [ToOwned][191] for T

where T: [Clone][72],

[Source][192][§][193]

#### type [Owned][194] = T

The resulting type after obtaining ownership.

[Source][195][§][196]

#### fn [to_owned][197](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][197]

[Source][198][§][199]

#### fn [clone_into][200](&self, target: [&mut T][117])

Uses borrowed data to replace owned data, usually by cloning. [Read more][200]

[Source][201][§][202]

### impl<T> [ToString][203] for T

where T: [Display][87] \+ ?[Sized][121],

[Source][204][§][205]

#### fn [to_string][206](&self) -> [String][207]

Converts the given value to a `String`. [Read more][206]

[Source][208][§][209]

### impl<T, U> [TryFrom][210]<U> for T

where U: [Into][185]<T>,

[Source][211][§][212]

#### type [Error][213] = [Infallible][214]

The type returned in the event of a conversion error.

[Source][215][§][216]

#### fn [try_from][217](value: U) -> [Result][91]<T, <T as [TryFrom][210]<U>>::[Error][218]>

Performs the conversion.

[Source][219][§][220]

### impl<T, U> [TryInto][221]<U> for T

where U: [TryFrom][210]<T>,

[Source][222][§][223]

#### type [Error][224] = <U as [TryFrom][210]<T>>::[Error][218]

The type returned in the event of a conversion error.

[Source][225][§][226]

#### fn [try_into][227](self) -> [Result][91]<U, <U as [TryFrom][210]<T>>::[Error][218]>

Performs the conversion.

   [1]: https://docs.rs/
   [2]: enum.ParseError.html# (Helpful macros for working with enums and strings)
   [3]: https://docs.rs/strum/0.28.0/strum/enum.ParseError.html (Get a link to this specific version)
   [4]: https://docs.rs/crate/strum/latest (See strum in docs.rs)
   [5]: https://spdx.org/licenses/MIT
   [6]: https://github.com/Peternator7/strum
   [7]: https://crates.io/crates/strum (See strum in crates.io)
   [8]: https://docs.rs/crate/strum/latest/source/ (Browse source of strum-0.28.0)
   [9]: https://crates.io/users/Peternator7
   [10]: https://docs.rs/phf/^0.13/
   [11]: https://docs.rs/strum_macros/^0.28/
   [12]: https://docs.rs/crate/strum/latest
   [13]: enum.ParseError.html#
   [14]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-apple-darwin/strum/enum.ParseError.html
   [15]: https://docs.rs/crate/strum/latest/target-redirect/aarch64-unknown-linux-gnu/strum/enum.ParseError.html
   [16]: https://docs.rs/crate/strum/latest/target-redirect/i686-pc-windows-msvc/strum/enum.ParseError.html
   [17]: https://docs.rs/crate/strum/latest/target-redirect/x86_64-pc-windows-msvc/strum/enum.ParseError.html
   [18]: https://docs.rs/crate/strum/latest/target-redirect/strum/enum.ParseError.html
   [19]: https://docs.rs/crate/strum/latest/features (Browse available feature flags of strum-0.28.0)
   [20]: https://docs.rs/about
   [21]: https://docs.rs/about/badges
   [22]: https://docs.rs/about/builds
   [23]: https://docs.rs/about/metadata
   [24]: https://docs.rs/about/redirections
   [25]: https://docs.rs/about/download
   [26]: https://docs.rs/about/rustdoc-json
   [27]: https://docs.rs/releases/queue
   [28]: https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs
   [29]: https://www.rust-lang.org/
   [30]: https://doc.rust-lang.org/book/
   [31]: https://doc.rust-lang.org/std/
   [32]: https://doc.rust-lang.org/rust-by-example/
   [33]: https://doc.rust-lang.org/cargo/guide/
   [34]: https://doc.rust-lang.org/nightly/clippy
   [35]: enum.ParseError.html#main-content
   [36]: index.html
   [37]: enum.ParseError.html#variants
   [38]: enum.ParseError.html#variant.VariantNotFound (VariantNotFound)
   [39]: enum.ParseError.html#trait-implementations
   [40]: enum.ParseError.html#impl-Clone-for-ParseError (Clone)
   [41]: enum.ParseError.html#impl-Copy-for-ParseError (Copy)
   [42]: enum.ParseError.html#impl-Debug-for-ParseError (Debug)
   [43]: enum.ParseError.html#impl-Display-for-ParseError (Display)
   [44]: enum.ParseError.html#impl-Eq-for-ParseError (Eq)
   [45]: enum.ParseError.html#impl-Error-for-ParseError (Error)
   [46]: enum.ParseError.html#impl-Hash-for-ParseError (Hash)
   [47]: enum.ParseError.html#impl-PartialEq-for-ParseError (PartialEq)
   [48]: enum.ParseError.html#impl-StructuralPartialEq-for-ParseError (StructuralPartialEq)
   [49]: enum.ParseError.html#synthetic-implementations
   [50]: enum.ParseError.html#impl-Freeze-for-ParseError (Freeze)
   [51]: enum.ParseError.html#impl-RefUnwindSafe-for-ParseError (RefUnwindSafe)
   [52]: enum.ParseError.html#impl-Send-for-ParseError (Send)
   [53]: enum.ParseError.html#impl-Sync-for-ParseError (Sync)
   [54]: enum.ParseError.html#impl-Unpin-for-ParseError (Unpin)
   [55]: enum.ParseError.html#impl-UnsafeUnpin-for-ParseError (UnsafeUnpin)
   [56]: enum.ParseError.html#impl-UnwindSafe-for-ParseError (UnwindSafe)
   [57]: enum.ParseError.html#blanket-implementations
   [58]: enum.ParseError.html#impl-Any-for-T (Any)
   [59]: enum.ParseError.html#impl-Borrow%3CT%3E-for-T (Borrow<T>)
   [60]: enum.ParseError.html#impl-BorrowMut%3CT%3E-for-T (BorrowMut<T>)
   [61]: enum.ParseError.html#impl-CloneToUninit-for-T (CloneToUninit)
   [62]: enum.ParseError.html#impl-From%3CT%3E-for-T (From<T>)
   [63]: enum.ParseError.html#impl-Into%3CU%3E-for-T (Into<U>)
   [64]: enum.ParseError.html#impl-ToOwned-for-T (ToOwned)
   [65]: enum.ParseError.html#impl-ToString-for-T (ToString)
   [66]: enum.ParseError.html#impl-TryFrom%3CU%3E-for-T (TryFrom<U>)
   [67]: enum.ParseError.html#impl-TryInto%3CU%3E-for-T (TryInto<U>)
   [68]: https://docs.rs/strum/latest/src/strum/lib.rs.html#42-44
   [69]: enum.ParseError.html#variant.VariantNotFound
   [70]: https://docs.rs/strum/latest/src/strum/lib.rs.html#41
   [71]: enum.ParseError.html#impl-Clone-for-ParseError
   [72]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [73]: enum.ParseError.html (enum strum::ParseError)
   [74]: enum.ParseError.html#method.clone
   [75]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [76]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [77]: enum.ParseError.html#method.clone_from
   [78]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [79]: enum.ParseError.html#impl-Debug-for-ParseError
   [80]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [81]: enum.ParseError.html#method.fmt
   [82]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [83]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [84]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [85]: https://docs.rs/strum/latest/src/strum/lib.rs.html#46-54
   [86]: enum.ParseError.html#impl-Display-for-ParseError
   [87]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [88]: https://docs.rs/strum/latest/src/strum/lib.rs.html#47-53
   [89]: enum.ParseError.html#method.fmt-1
   [90]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [91]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [92]: https://doc.rust-lang.org/nightly/std/primitive.unit.html
   [93]: https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html (struct core::fmt::Error)
   [94]: https://docs.rs/strum/latest/src/strum/lib.rs.html#57-66
   [95]: enum.ParseError.html#impl-Error-for-ParseError
   [96]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html (trait core::error::Error)
   [97]: https://docs.rs/strum/latest/src/strum/lib.rs.html#58-65
   [98]: enum.ParseError.html#method.description
   [99]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description
   [100]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [101]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#111
   [102]: enum.ParseError.html#method.source
   [103]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source
   [104]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [105]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#147
   [106]: enum.ParseError.html#method.cause
   [107]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause
   [108]: https://doc.rust-lang.org/nightly/src/core/error.rs.html#260
   [109]: enum.ParseError.html#method.provide
   [110]: https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide
   [111]: https://doc.rust-lang.org/nightly/core/error/struct.Request.html (struct core::error::Request)
   [112]: enum.ParseError.html#impl-Hash-for-ParseError
   [113]: https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html (trait core::hash::Hash)
   [114]: enum.ParseError.html#method.hash
   [115]: https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash
   [116]: https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html (trait core::hash::Hasher)
   [117]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [118]: https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236
   [119]: enum.ParseError.html#method.hash_slice
   [120]: https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice
   [121]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [122]: enum.ParseError.html#impl-PartialEq-for-ParseError
   [123]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [124]: enum.ParseError.html#method.eq
   [125]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [126]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [127]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [128]: enum.ParseError.html#method.ne
   [129]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [130]: enum.ParseError.html#impl-Copy-for-ParseError
   [131]: https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html (trait core::marker::Copy)
   [132]: enum.ParseError.html#impl-Eq-for-ParseError
   [133]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html (trait core::cmp::Eq)
   [134]: enum.ParseError.html#impl-StructuralPartialEq-for-ParseError
   [135]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [136]: enum.ParseError.html#impl-Freeze-for-ParseError
   [137]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [138]: enum.ParseError.html#impl-RefUnwindSafe-for-ParseError
   [139]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [140]: enum.ParseError.html#impl-Send-for-ParseError
   [141]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [142]: enum.ParseError.html#impl-Sync-for-ParseError
   [143]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [144]: enum.ParseError.html#impl-Unpin-for-ParseError
   [145]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [146]: enum.ParseError.html#impl-UnsafeUnpin-for-ParseError
   [147]: https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html (trait core::marker::UnsafeUnpin)
   [148]: enum.ParseError.html#impl-UnwindSafe-for-ParseError
   [149]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [150]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [151]: enum.ParseError.html#impl-Any-for-T
   [152]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [153]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [154]: enum.ParseError.html#method.type_id
   [155]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [156]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [157]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [158]: enum.ParseError.html#impl-Borrow%3CT%3E-for-T
   [159]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [160]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [161]: enum.ParseError.html#method.borrow
   [162]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [163]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [164]: enum.ParseError.html#impl-BorrowMut%3CT%3E-for-T
   [165]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [166]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [167]: enum.ParseError.html#method.borrow_mut
   [168]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [169]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [170]: enum.ParseError.html#impl-CloneToUninit-for-T
   [171]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [172]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [173]: enum.ParseError.html#method.clone_to_uninit
   [174]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [175]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [176]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [177]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [178]: enum.ParseError.html#impl-From%3CT%3E-for-T
   [179]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [180]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [181]: enum.ParseError.html#method.from
   [182]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [183]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [184]: enum.ParseError.html#impl-Into%3CU%3E-for-T
   [185]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [186]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [187]: enum.ParseError.html#method.into
   [188]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [189]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [190]: enum.ParseError.html#impl-ToOwned-for-T
   [191]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [192]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [193]: enum.ParseError.html#associatedtype.Owned
   [194]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [195]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [196]: enum.ParseError.html#method.to_owned
   [197]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [198]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [199]: enum.ParseError.html#method.clone_into
   [200]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [201]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891
   [202]: enum.ParseError.html#impl-ToString-for-T
   [203]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [204]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [205]: enum.ParseError.html#method.to_string
   [206]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [207]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [208]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [209]: enum.ParseError.html#impl-TryFrom%3CU%3E-for-T
   [210]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [211]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [212]: enum.ParseError.html#associatedtype.Error-1
   [213]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [214]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [215]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [216]: enum.ParseError.html#method.try_from
   [217]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [218]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [219]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [220]: enum.ParseError.html#impl-TryInto%3CU%3E-for-T
   [221]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [222]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [223]: enum.ParseError.html#associatedtype.Error
   [224]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [225]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [226]: enum.ParseError.html#method.try_into
   [227]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into

