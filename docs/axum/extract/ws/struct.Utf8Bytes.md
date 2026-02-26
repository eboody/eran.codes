<!-- Generated from rustdoc HTML: extract/ws/struct.Utf8Bytes.html -->
<!-- Source: /home/eran/code/axum (crate axum v0.8.8) -->

Skip to main content

## Utf8Bytes

## [axum][1]0.8.8

## Utf8Bytes

### Methods

  * as_str
  * from_static



### Methods from Deref<Target=str>

  * as_ascii
  * as_ascii_unchecked
  * as_bytes
  * as_ptr
  * as_str
  * bytes
  * ceil_char_boundary
  * char_indices
  * chars
  * contains
  * encode_utf16
  * ends_with
  * eq_ignore_ascii_case
  * escape_debug
  * escape_default
  * escape_unicode
  * find
  * floor_char_boundary
  * get
  * get_unchecked
  * is_ascii
  * is_char_boundary
  * is_empty
  * len
  * lines
  * lines_any
  * match_indices
  * matches
  * parse
  * repeat
  * replace
  * replacen
  * rfind
  * rmatch_indices
  * rmatches
  * rsplit
  * rsplit_once
  * rsplit_terminator
  * rsplitn
  * slice_unchecked
  * split
  * split_ascii_whitespace
  * split_at
  * split_at_checked
  * split_inclusive
  * split_once
  * split_terminator
  * split_whitespace
  * splitn
  * starts_with
  * strip_circumfix
  * strip_prefix
  * strip_suffix
  * substr_range
  * to_ascii_lowercase
  * to_ascii_uppercase
  * to_lowercase
  * to_uppercase
  * trim
  * trim_ascii
  * trim_ascii_end
  * trim_ascii_start
  * trim_end
  * trim_end_matches
  * trim_left
  * trim_left_matches
  * trim_matches
  * trim_prefix
  * trim_right
  * trim_right_matches
  * trim_start
  * trim_start_matches
  * trim_suffix



### Trait Implementations

  * Clone
  * Debug
  * Default
  * Deref
  * Display
  * Eq
  * From<&String>
  * From<&str>
  * From<String>
  * From<Utf8Bytes>
  * PartialEq
  * PartialEq<T>
  * StructuralPartialEq
  * TryFrom<Bytes>
  * TryFrom<Vec<u8>>



### Auto Trait Implementations

  * !Freeze
  * RefUnwindSafe
  * Send
  * Sync
  * Unpin
  * UnwindSafe



### Blanket Implementations

  * Any
  * Borrow<T>
  * BorrowMut<T>
  * CloneToUninit
  * Equivalent<K>
  * Equivalent<K>
  * From<T>
  * FromRef<T>
  * Instrument
  * Into<U>
  * PolicyExt
  * Receiver
  * Same
  * ServiceExt
  * ToOwned
  * ToString
  * ToStringFallible
  * TryFrom<U>
  * TryInto<U>
  * VZip<V>
  * WithSubscriber



## [In axum::extract::ws][2]

[axum][3]::[extract][4]::[ws][2]

# Struct Utf8Bytes Copy item path

[Source][5]
``` 
pub struct Utf8Bytes(/* private fields */);
```

Available on **crate feature`ws`** only.

Expand description

UTF-8 wrapper for [Bytes].

An [Utf8Bytes][6] is always guaranteed to contain valid UTF-8.

## Implementations§

[Source][7]§

### impl [Utf8Bytes][6]

[Source][8]

#### pub const fn from_static(str: &'static [str][9]) -> Self

Creates from a static str.

[Source][10]

#### pub fn as_str(&self) -> &[str][9]

Returns as a string slice.

## Methods from [Deref][11]<Target = [str][9]>§

1.0.0 · [Source][12]

#### pub fn len(&self) -> [usize][13]

Returns the length of `self`.

This length is in bytes, not [`char`][14]s or graphemes. In other words, it might not be what a human considers the length of the string.

##### §Examples
``` 
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0 · [Source][15]

#### pub fn is_empty(&self) -> [bool][16]

Returns `true` if `self` has a length of zero bytes.

##### §Examples
``` 
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0 · [Source][17]

#### pub fn is_char_boundary(&self, index: [usize][13]) -> [bool][16]

Checks that `index`-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when `index == self.len()`) are considered to be boundaries.

Returns `false` if `index` is greater than `self.len()`.

##### §Examples
``` 
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.91.0 · [Source][18]

#### pub fn floor_char_boundary(&self, index: [usize][13]) -> [usize][13]

Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`][19] is `true`.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

##### §Examples
``` 
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.91.0 · [Source][20]

#### pub fn ceil_char_boundary(&self, index: [usize][13]) -> [usize][13]

Finds the closest `x` not below `index` where [`is_char_boundary(x)`][19] is `true`.

If `index` is greater than the length of the string, this returns the length of the string.

This method is the natural complement to [`floor_char_boundary`][21]. See that method for more details.

##### §Examples
``` 
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0 · [Source][22]

#### pub fn as_bytes(&self) -> &[[u8][23]] ⓘ

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the [`from_utf8`][24] function.

##### §Examples
``` 
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.0.0 · [Source][25]

#### pub fn as_ptr(&self) -> [*const ][26][u8][23]

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`][27]. This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use [`as_mut_ptr`][28].

##### §Examples
``` 
let s = "Hello";
let ptr = s.as_ptr();
```

1.20.0 · [Source][29]

#### pub fn get<I>(&self, i: I) -> [Option][30]<&<I as [SliceIndex][31]<[str][9]>>::[Output][32]>

where I: [SliceIndex][31]<[str][9]>,

Returns a subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`][33] whenever equivalent indexing operation would panic.

##### §Examples
``` 
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0 · [Source][34]

#### pub unsafe fn get_unchecked<I>(&self, i: I) -> &<I as [SliceIndex][31]<[str][9]>>::[Output][32]

where I: [SliceIndex][31]<[str][9]>,

Returns an unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### §Safety

Callers of this function are responsible that these preconditions are satisfied:

  * The starting index must not exceed the ending index;
  * Indexes must be within bounds of the original slice;
  * Indexes must lie on UTF-8 sequence boundaries.



Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### §Examples
``` 
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.0.0 · [Source][35]

#### pub unsafe fn slice_unchecked(&self, begin: [usize][13], end: [usize][13]) -> &[str][9]

👎Deprecated since 1.29.0: use `get_unchecked(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`][36] and [`Index`][37].

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get a mutable string slice instead, see the [`slice_mut_unchecked`][38] method.

##### §Safety

Callers of this function are responsible that three preconditions are satisfied:

  * `begin` must not exceed `end`.
  * `begin` and `end` must be byte positions within the string slice.
  * `begin` and `end` must lie on UTF-8 sequence boundaries.



##### §Examples
``` 
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.4.0 · [Source][39]

#### pub fn split_at(&self, mid: [usize][13]) -> (&[str][9], &[str][9])

Divides one string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut`][40] method.

##### §Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_checked`][41].

##### §Examples
``` 
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.80.0 · [Source][42]

#### pub fn split_at_checked(&self, mid: [usize][13]) -> [Option][30]<(&[str][9], &[str][9])>

Divides one string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut_checked`][43] method.

##### §Examples
``` 
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.0.0 · [Source][44]

#### pub fn chars(&self) -> [Chars][45]<'_>

Returns an iterator over the [`char`][14]s of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`][14]. This method returns such an iterator.

It’s important to remember that [`char`][14] represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust’s standard library, check crates.io instead.

##### §Examples

Basic usage:
``` 
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`][14]s might not match your intuition about characters:
``` 
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0 · [Source][46]

#### pub fn char_indices(&self) -> [CharIndices][47]<'_>

Returns an iterator over the [`char`][14]s of a string slice, and their positions.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`][14]. This method returns an iterator of both these [`char`][14]s, as well as their byte positions.

The iterator yields tuples. The position is first, the [`char`][14] is second.

##### §Examples

Basic usage:
``` 
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember, [`char`][14]s might not match your intuition about characters:
``` 
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0 · [Source][48]

#### pub fn bytes(&self) -> [Bytes][49]<'_>

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.

##### §Examples
``` 
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0 · [Source][50]

#### pub fn split_whitespace(&self) -> [SplitWhitespace][51]<'_>

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`. If you only want to split on ASCII whitespace instead, use [`split_ascii_whitespace`][52].

##### §Examples

Basic usage:
``` 
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:
``` 
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:
``` 
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0 · [Source][53]

#### pub fn split_ascii_whitespace(&self) -> [SplitAsciiWhitespace][54]<'_>

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition as [`char::is_ascii_whitespace`][55]. To split by Unicode `Whitespace` instead, use [`split_whitespace`][56].

##### §Examples

Basic usage:
``` 
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered (see [`char::is_ascii_whitespace`][55]):
``` 
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:
``` 
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0 · [Source][57]

#### pub fn lines(&self) -> [Lines][58]<'_>

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (`\n`) or sequences of a carriage return followed by a line feed (`\r\n`).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (`\r`) not immediately followed by a line feed (`\n`) does not split a line. These carriage returns are thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line ending will return the same lines as an otherwise identical string without a final line ending.

An empty string returns an empty iterator.

##### §Examples

Basic usage:
``` 
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:
``` 
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

An empty string returns an empty iterator:
``` 
let text = "";
let mut lines = text.lines();

assert_eq!(lines.next(), None);
```

1.0.0 · [Source][59]

#### pub fn lines_any(&self) -> [LinesAny][60]<'_>

👎Deprecated since 1.4.0: use lines() instead now

Returns an iterator over the lines of a string.

1.8.0 · [Source][61]

#### pub fn encode_utf16(&self) -> [EncodeUtf16][62]<'_>

Returns an iterator of `u16` over the string encoded as native endian UTF-16 (without byte-order mark).

##### §Examples
``` 
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0 · [Source][63]

#### pub fn contains<P>(&self, pat: P) -> [bool][16]

where P: [Pattern][64],

Returns `true` if the given pattern matches a sub-slice of this string slice.

Returns `false` if it does not.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0 · [Source][66]

#### pub fn starts_with<P>(&self, pat: P) -> [bool][16]

where P: [Pattern][64],

Returns `true` if the given pattern matches a prefix of this string slice.

Returns `false` if it does not.

The [pattern][65] can be a `&str`, in which case this function will return true if the `&str` is a prefix of this string slice.

The [pattern][65] can also be a [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches. These will only be checked against the first character of this string slice. Look at the second example below regarding behavior for slices of [`char`][14]s.

##### §Examples
``` 
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```
``` 
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0 · [Source][67]

#### pub fn ends_with<P>(&self, pat: P) -> [bool][16]

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns `true` if the given pattern matches a suffix of this string slice.

Returns `false` if it does not.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0 · [Source][70]

#### pub fn find<P>(&self, pat: P) -> [Option][30]<[usize][13]>

where P: [Pattern][64],

Returns the byte index of the first character of this string slice that matches the pattern.

Returns [`None`][33] if the pattern doesn’t match.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples

Simple patterns:
``` 
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:
``` 
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:
``` 
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0 · [Source][71]

#### pub fn rfind<P>(&self, pat: P) -> [Option][30]<[usize][13]>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns the byte index for the first character of the last match of the pattern in this string slice.

Returns [`None`][33] if the pattern doesn’t match.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples

Simple patterns:
``` 
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:
``` 
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:
``` 
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0 · [Source][72]

#### pub fn split<P>(&self, pat: P) -> [Split][73]<'_, P>

where P: [Pattern][64],

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only item in the iterator.

##### §Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`][74] if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`][14], but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit`][75] method can be used.

##### §Examples

Simple patterns:
``` 
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:
``` 
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:
``` 
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up with empty strings in the output:
``` 
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.
``` 
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored by empty strings.
``` 
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates every character in the string, along with the beginning and end of the string.
``` 
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior when whitespace is used as the separator. This code is correct:
``` 
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It does _not_ give you:

ⓘ
```
assert_eq!(d, &["a", "b", "c"]);
```

Use [`split_whitespace`][56] for this behavior.

1.51.0 · [Source][76]

#### pub fn split_inclusive<P>(&self, pat: P) -> [SplitInclusive][77]<'_, P>

where P: [Pattern][64],

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

Differs from the iterator produced by `split` in that `split_inclusive` leaves the matched part as the terminator of the substring.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched, that element will be considered the terminator of the preceding substring. That substring will be the last item returned by the iterator.
``` 
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0 · [Source][78]

#### pub fn rsplit<P>(&self, pat: P) -> [RSplit][79]<'_, P>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`][74] if a forward/reverse search yields the same elements.

For iterating from the front, the [`split`][80] method can be used.

##### §Examples

Simple patterns:
``` 
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:
``` 
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0 · [Source][81]

#### pub fn split_terminator<P>(&self, pat: P) -> [SplitTerminator][82]<'_, P>

where P: [Pattern][64],

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

Equivalent to [`split`][80], except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_ , rather than _separated_ by a pattern.

##### §Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`][74] if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`][14], but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit_terminator`][83] method can be used.

##### §Examples
``` 
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0 · [Source][84]

#### pub fn rsplit_terminator<P>(&self, pat: P) -> [RSplitTerminator][85]<'_, P>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns an iterator over substrings of `self`, separated by characters matched by a pattern and yielded in reverse order.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

Equivalent to [`split`][80], except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_ , rather than _separated_ by a pattern.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be double ended if a forward/reverse search yields the same elements.

For iterating from the front, the [`split_terminator`][86] method can be used.

##### §Examples
``` 
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0 · [Source][87]

#### pub fn splitn<P>(&self, n: [usize][13], pat: P) -> [SplitN][88]<'_, P>

where P: [Pattern][64],

Returns an iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

If the pattern allows a reverse search, the [`rsplitn`][89] method can be used.

##### §Examples

Simple patterns:
``` 
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:
``` 
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0 · [Source][90]

#### pub fn rsplitn<P>(&self, n: [usize][13], pat: P) -> [RSplitN][91]<'_, P>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns an iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

For splitting from the front, the [`splitn`][92] method can be used.

##### §Examples

Simple patterns:
``` 
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:
``` 
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0 · [Source][93]

#### pub fn split_once<P>(&self, delimiter: P) -> [Option][30]<(&[str][9], &[str][9])>

where P: [Pattern][64],

Splits the string on the first occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### §Examples
``` 
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0 · [Source][94]

#### pub fn rsplit_once<P>(&self, delimiter: P) -> [Option][30]<(&[str][9], &[str][9])>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Splits the string on the last occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### §Examples
``` 
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=".rsplit_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0 · [Source][95]

#### pub fn matches<P>(&self, pat: P) -> [Matches][96]<'_, P>

where P: [Pattern][64],

Returns an iterator over the disjoint matches of a pattern within the given string slice.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`][74] if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`][14], but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatches`][97] method can be used.

##### §Examples
``` 
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0 · [Source][98]

#### pub fn rmatches<P>(&self, pat: P) -> [RMatches][99]<'_, P>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns an iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`][74] if a forward/reverse search yields the same elements.

For iterating from the front, the [`matches`][100] method can be used.

##### §Examples
``` 
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0 · [Source][101]

#### pub fn match_indices<P>(&self, pat: P) -> [MatchIndices][102]<'_, P>

where P: [Pattern][64],

Returns an iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.

For matches of `pat` within `self` that overlap, only the indices corresponding to the first match are returned.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`][74] if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`][14], but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatch_indices`][103] method can be used.

##### §Examples
``` 
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0 · [Source][104]

#### pub fn rmatch_indices<P>(&self, pat: P) -> [RMatchIndices][105]<'_, P>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns an iterator over the disjoint matches of a pattern within `self`, yielded in reverse order along with the index of the match.

For matches of `pat` within `self` that overlap, only the indices corresponding to the last match are returned.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`][74] if a forward/reverse search yields the same elements.

For iterating from the front, the [`match_indices`][106] method can be used.

##### §Examples
``` 
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0 · [Source][107]

#### pub fn trim(&self) -> &[str][9]

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### §Examples
``` 
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0 · [Source][108]

#### pub fn trim_start(&self) -> &[str][9]

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### §Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### §Examples

Basic usage:
``` 
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:
``` 
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0 · [Source][109]

#### pub fn trim_end(&self) -> &[str][9]

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### §Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### §Examples

Basic usage:
``` 
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:
``` 
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0 · [Source][110]

#### pub fn trim_left(&self) -> &[str][9]

👎Deprecated since 1.33.0: superseded by `trim_start`

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### §Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### §Examples

Basic usage:
``` 
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:
``` 
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0 · [Source][111]

#### pub fn trim_right(&self) -> &[str][9]

👎Deprecated since 1.33.0: superseded by `trim_end`

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### §Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### §Examples

Basic usage:
``` 
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:
``` 
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0 · [Source][112]

#### pub fn trim_matches<P>(&self, pat: P) -> &[str][9]

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [DoubleEndedSearcher][113]<'a>,

Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.

The [pattern][65] can be a [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples

Simple patterns:
``` 
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:
``` 
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0 · [Source][114]

#### pub fn trim_start_matches<P>(&self, pat: P) -> &[str][9]

where P: [Pattern][64],

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### §Examples
``` 
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0 · [Source][115]

#### pub fn strip_prefix<P>(&self, prefix: P) -> [Option][30]<&[str][9]>

where P: [Pattern][64],

Returns a string slice with the prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix, wrapped in `Some`. Unlike [`trim_start_matches`][116], this method removes the prefix exactly once.

If the string does not start with `prefix`, returns `None`.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0 · [Source][117]

#### pub fn strip_suffix<P>(&self, suffix: P) -> [Option][30]<&[str][9]>

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns a string slice with the suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix, wrapped in `Some`. Unlike [`trim_end_matches`][118], this method removes the suffix exactly once.

If the string does not end with `suffix`, returns `None`.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

[Source][119]

#### pub fn strip_circumfix<P, S>(&self, prefix: P, suffix: S) -> [Option][30]<&[str][9]>

where P: [Pattern][64], S: [Pattern][64], <S as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

🔬This is a nightly-only experimental API. (`strip_circumfix`)

Returns a string slice with the prefix and suffix removed.

If the string starts with the pattern `prefix` and ends with the pattern `suffix`, returns the substring after the prefix and before the suffix, wrapped in `Some`. Unlike [`trim_start_matches`][116] and [`trim_end_matches`][118], this method removes both the prefix and suffix exactly once.

If the string does not start with `prefix` or does not end with `suffix`, returns `None`.

Each [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
#![feature(strip_circumfix)]

assert_eq!("bar:hello:foo".strip_circumfix("bar:", ":foo"), Some("hello"));
assert_eq!("bar:foo".strip_circumfix("foo", "foo"), None);
assert_eq!("foo:bar;".strip_circumfix("foo:", ';'), Some("bar"));
```

[Source][120]

#### pub fn trim_prefix<P>(&self, prefix: P) -> &[str][9]

where P: [Pattern][64],

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix. Unlike [`strip_prefix`][121], this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`][30].

If the string does not start with `prefix`, returns the original string unchanged.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

[Source][122]

#### pub fn trim_suffix<P>(&self, suffix: P) -> &[str][9]

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix. Unlike [`strip_suffix`][123], this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`][30].

If the string does not end with `suffix`, returns the original string unchanged.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Examples
``` 
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0 · [Source][124]

#### pub fn trim_end_matches<P>(&self, pat: P) -> &[str][9]

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### §Examples

Simple patterns:
``` 
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:
``` 
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source][125]

#### pub fn trim_left_matches<P>(&self, pat: P) -> &[str][9]

where P: [Pattern][64],

👎Deprecated since 1.33.0: superseded by `trim_start_matches`

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### §Examples
``` 
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0 · [Source][126]

#### pub fn trim_right_matches<P>(&self, pat: P) -> &[str][9]

where P: [Pattern][64], <P as [Pattern][64]>::[Searcher][68]<'a>: for<'a> [ReverseSearcher][69]<'a>,

👎Deprecated since 1.33.0: superseded by `trim_end_matches`

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern][65] can be a `&str`, [`char`][14], a slice of [`char`][14]s, or a function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### §Examples

Simple patterns:
``` 
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:
``` 
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source][127]

#### pub fn parse<F>(&self) -> [Result][128]<F, <F as [FromStr][129]>::[Err][130]>

where F: [FromStr][129],

Parses this string slice into another type.

Because `parse` is so general, it can cause problems with type inference. As such, `parse` is one of the few times you’ll see the syntax affectionately known as the ‘turbofish’: `::<>`. This helps the inference algorithm understand specifically which type you’re trying to parse into.

`parse` can parse into any type that implements the [`FromStr`][129] trait.

##### §Errors

Will return [`Err`][131] if it’s not possible to parse this string slice into the desired type.

##### §Examples

Basic usage:
``` 
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotating `four`:
``` 
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:
``` 
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0 · [Source][132]

#### pub fn is_ascii(&self) -> [bool][16]

Checks if all characters in this string are within the ASCII range.

An empty string returns `true`.

##### §Examples
``` 
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

[Source][133]

#### pub fn as_ascii(&self) -> [Option][30]<&[[AsciiChar][134]]>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this string slice [`is_ascii`][135], returns it as a slice of [ASCII characters][134], otherwise returns `None`.

[Source][136]

#### pub unsafe fn as_ascii_unchecked(&self) -> &[[AsciiChar][134]]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this string slice into a slice of [ASCII characters][134], without checking whether they are valid.

##### §Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0 · [Source][137]

#### pub fn eq_ignore_ascii_case(&self, other: &[str][9]) -> [bool][16]

Checks that two strings are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

##### §Examples
``` 
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

1.80.0 · [Source][138]

#### pub fn trim_ascii_start(&self) -> &[str][9]

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`][139].

##### §Examples
``` 
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0 · [Source][140]

#### pub fn trim_ascii_end(&self) -> &[str][9]

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`][139].

##### §Examples
``` 
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0 · [Source][141]

#### pub fn trim_ascii(&self) -> &[str][9]

Returns a string slice with leading and trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`][139].

##### §Examples
``` 
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0 · [Source][142]

#### pub fn escape_debug(&self) -> [EscapeDebug][143]<'_>

Returns an iterator that escapes each char in `self` with [`char::escape_debug`][144].

Note: only extended grapheme codepoints that begin the string will be escaped.

##### §Examples

As an iterator:
``` 
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Using `println!` directly:
``` 
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:
``` 
println!("❤\\n!");
```

Using `to_string`:
``` 
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0 · [Source][145]

#### pub fn escape_default(&self) -> [EscapeDefault][146]<'_>

Returns an iterator that escapes each char in `self` with [`char::escape_default`][147].

##### §Examples

As an iterator:
``` 
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Using `println!` directly:
``` 
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:
``` 
println!("\\u{{2764}}\\n!");
```

Using `to_string`:
``` 
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0 · [Source][148]

#### pub fn escape_unicode(&self) -> [EscapeUnicode][149]<'_>

Returns an iterator that escapes each char in `self` with [`char::escape_unicode`][150].

##### §Examples

As an iterator:
``` 
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Using `println!` directly:
``` 
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:
``` 
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Using `to_string`:
``` 
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

[Source][151]

#### pub fn substr_range(&self, substr: &[str][9]) -> [Option][30]<[Range][152]<[usize][13]>>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range that a substring points to.

Returns `None` if `substr` does not point within `self`.

Unlike [`str::find`][153], **this does not search through the string**. Instead, it uses pointer arithmetic to find where in the string `substr` is derived from.

This is useful for extending [`str::split`][80] and similar methods.

Note that this method may return false positives (typically either `Some(0..0)` or `Some(self.len()..self.len())`) if `substr` is a zero-length `str` that points at the beginning or end of another, independent, `str`.

##### §Examples
``` 
#![feature(substr_range)]

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(0..1));
assert_eq!(iter.next(), Some(3..4));
assert_eq!(iter.next(), Some(6..7));
assert_eq!(iter.next(), Some(9..10));
```

[Source][154]

#### pub fn as_str(&self) -> &[str][9]

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same string as a string slice `&str`.

This method is redundant when used directly on `&str`, but it helps dereferencing other string-like types to string slices, for example references to `Box<str>` or `Arc<str>`.

1.0.0 · [Source][155]

#### pub fn replace<P>(&self, from: P, to: &[str][9]) -> [String][156]

where P: [Pattern][64],

Available on **non-`no_global_oom_handling`** only.

Replaces all matches of a pattern with another string.

`replace` creates a new [`String`][156], and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice.

##### §Examples
``` 
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice as [`String`][156]:
``` 
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0 · [Source][157]

#### pub fn replacen<P>(&self, pat: P, to: &[str][9], count: [usize][13]) -> [String][156]

where P: [Pattern][64],

Available on **non-`no_global_oom_handling`** only.

Replaces first N matches of a pattern with another string.

`replacen` creates a new [`String`][156], and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice at most `count` times.

##### §Examples
``` 
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice as [`String`][156]:
``` 
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0 · [Source][158]

#### pub fn to_lowercase(&self) -> [String][156]

Available on **non-`no_global_oom_handling`** only.

Returns the lowercase equivalent of this string slice, as a new [`String`][156].

‘Lowercase’ is defined according to the terms of the Unicode Derived Core Property `Lowercase`.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`][156] instead of modifying the parameter in-place.

##### §Examples

Basic usage:
``` 
let s = "HELLO";

assert_eq!("hello", s.to_lowercase());
```

A tricky example, with sigma:
``` 
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
```

Languages without case are not changed:
``` 
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

1.2.0 · [Source][159]

#### pub fn to_uppercase(&self) -> [String][156]

Available on **non-`no_global_oom_handling`** only.

Returns the uppercase equivalent of this string slice, as a new [`String`][156].

‘Uppercase’ is defined according to the terms of the Unicode Derived Core Property `Uppercase`.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`][156] instead of modifying the parameter in-place.

##### §Examples

Basic usage:
``` 
let s = "hello";

assert_eq!("HELLO", s.to_uppercase());
```

Scripts without case are not changed:
``` 
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:
``` 
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

1.16.0 · [Source][160]

#### pub fn repeat(&self, n: [usize][13]) -> [String][156]

Available on **non-`no_global_oom_handling`** only.

Creates a new [`String`][156] by repeating a string `n` times.

##### §Panics

This function will panic if the capacity would overflow.

##### §Examples

Basic usage:
``` 
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

ⓘ
```
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0 · [Source][161]

#### pub fn to_ascii_uppercase(&self) -> [String][156]

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`][162].

To uppercase ASCII characters in addition to non-ASCII characters, use `to_uppercase`.

##### §Examples
``` 
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0 · [Source][163]

#### pub fn to_ascii_lowercase(&self) -> [String][156]

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`][164].

To lowercase ASCII characters in addition to non-ASCII characters, use `to_lowercase`.

##### §Examples
``` 
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations§

[Source][165]§

### impl [Clone][166] for [Utf8Bytes][6]

[Source][165]§

#### fn [clone][167](&self) -> [Utf8Bytes][6]

Returns a duplicate of the value. [Read more][167]

1.0.0 · [Source][168]§

#### fn [clone_from][169](&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more][169]

[Source][165]§

### impl [Debug][170] for [Utf8Bytes][6]

[Source][165]§

#### fn [fmt][171](&self, f: &mut [Formatter][172]<'_>) -> [Result][173]

Formats the value using the given formatter. [Read more][171]

[Source][165]§

### impl [Default][174] for [Utf8Bytes][6]

[Source][165]§

#### fn [default][175]() -> [Utf8Bytes][6]

Returns the “default value” for a type. [Read more][175]

[Source][176]§

### impl [Deref][11] for [Utf8Bytes][6]

[Source][177]§

#### fn [deref][178](&self) -> &Self::[Target][179]
``` 
/// Example fn that takes a str slice
fn a(s: &str) {}

let data = axum::extract::ws::Utf8Bytes::from_static("foo123");

// auto-deref as arg
a(&data);

// deref to str methods
assert_eq!(data.len(), 6);
```

[Source][180]§

#### type [Target][181] = [str][9]

The resulting type after dereferencing.

[Source][182]§

### impl [Display][183] for [Utf8Bytes][6]

[Source][184]§

#### fn [fmt][185](&self, f: &mut [Formatter][172]<'_>) -> [Result][173]

Formats the value using the given formatter. [Read more][185]

[Source][186]§

### impl [From][187]<&[String][156]> for [Utf8Bytes][6]

[Source][188]§

#### fn [from][189](s: &[String][156]) -> Self

Converts to this type from the input type.

[Source][190]§

### impl [From][187]<&[str][9]> for [Utf8Bytes][6]

[Source][191]§

#### fn [from][189](s: &[str][9]) -> Self

Converts to this type from the input type.

[Source][192]§

### impl [From][187]<[String][156]> for [Utf8Bytes][6]

[Source][193]§

#### fn [from][189](s: [String][156]) -> Self

Converts to this type from the input type.

[Source][194]§

### impl [From][187]<[Utf8Bytes][6]> for Bytes

[Source][195]§

#### fn [from][189](Utf8Bytes: [Utf8Bytes][6]) -> Self

Converts to this type from the input type.

[Source][196]§

### impl<T> [PartialEq][197]<T> for [Utf8Bytes][6]

where for<'a> &'a [str][9]: [PartialEq][197]<T>,

[Source][198]§

#### fn [eq][199](&self, other: [&T][200]) -> [bool][16]
``` 
let payload = axum::extract::ws::Utf8Bytes::from_static("foo123");
assert_eq!(payload, "foo123");
assert_eq!(payload, "foo123".to_string());
assert_eq!(payload, &"foo123".to_string());
assert_eq!(payload, std::borrow::Cow::from("foo123"));
```

1.0.0 · [Source][201]§

#### fn [ne][202](&self, other: [&Rhs][200]) -> [bool][16]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][165]§

### impl [PartialEq][197] for [Utf8Bytes][6]

[Source][165]§

#### fn [eq][199](&self, other: &[Utf8Bytes][6]) -> [bool][16]

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source][201]§

#### fn [ne][202](&self, other: [&Rhs][200]) -> [bool][16]

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source][203]§

### impl [TryFrom][204]<Bytes> for [Utf8Bytes][6]

[Source][205]§

#### type [Error][206] = [Utf8Error][207]

The type returned in the event of a conversion error.

[Source][208]§

#### fn [try_from][209](bytes: Bytes) -> [Result][128]<Self, Self::[Error][210]>

Performs the conversion.

[Source][211]§

### impl [TryFrom][204]<[Vec][212]<[u8][23]>> for [Utf8Bytes][6]

[Source][213]§

#### type [Error][206] = [Utf8Error][207]

The type returned in the event of a conversion error.

[Source][214]§

#### fn [try_from][209](v: [Vec][212]<[u8][23]>) -> [Result][128]<Self, Self::[Error][210]>

Performs the conversion.

[Source][165]§

### impl [Eq][215] for [Utf8Bytes][6]

[Source][165]§

### impl [StructuralPartialEq][216] for [Utf8Bytes][6]

## Auto Trait Implementations§

§

### impl ![Freeze][217] for [Utf8Bytes][6]

§

### impl [RefUnwindSafe][218] for [Utf8Bytes][6]

§

### impl [Send][219] for [Utf8Bytes][6]

§

### impl [Sync][220] for [Utf8Bytes][6]

§

### impl [Unpin][221] for [Utf8Bytes][6]

§

### impl [UnwindSafe][222] for [Utf8Bytes][6]

## Blanket Implementations§

[Source][223]§

### impl<T> [Any][224] for T

where T: 'static + ?[Sized][225],

[Source][226]§

#### fn [type_id][227](&self) -> [TypeId][228]

Gets the `TypeId` of `self`. [Read more][227]

[Source][229]§

### impl<T> [Borrow][230]<T> for T

where T: ?[Sized][225],

[Source][231]§

#### fn [borrow][232](&self) -> [&T][200]

Immutably borrows from an owned value. [Read more][232]

[Source][233]§

### impl<T> [BorrowMut][234]<T> for T

where T: ?[Sized][225],

[Source][235]§

#### fn [borrow_mut][236](&mut self) -> [&mut T][200]

Mutably borrows from an owned value. [Read more][236]

[Source][237]§

### impl<T> [CloneToUninit][238] for T

where T: [Clone][166],

[Source][239]§

#### unsafe fn [clone_to_uninit][240](&self, dest: [*mut ][26][u8][23])

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more][240]

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][215] \+ ?[Sized][225], K: [Borrow][230]<Q> \+ ?[Sized][225],

§

#### fn equivalent(&self, key: [&K][200]) -> [bool][16]

Checks if this value is equivalent to the given key. Read more

§

### impl<Q, K> Equivalent<K> for Q

where Q: [Eq][215] \+ ?[Sized][225], K: [Borrow][230]<Q> \+ ?[Sized][225],

§

#### fn equivalent(&self, key: [&K][200]) -> [bool][16]

Compare self to `key` and return `true` if they are equal.

[Source][241]§

### impl<T> [From][187]<T> for T

[Source][242]§

#### fn [from][189](t: T) -> T

Returns the argument unchanged.

§

### impl<T> [FromRef][243]<T> for T

where T: [Clone][166],

§

#### fn [from_ref][244](input: [&T][200]) -> T

Converts to this type from a reference to the input type.

§

### impl<T> Instrument for T

§

#### fn instrument(self, span: Span) -> Instrumented<Self>

Instruments this type with the provided [`Span`], returning an `Instrumented` wrapper. Read more

§

#### fn in_current_span(self) -> Instrumented<Self>

Instruments this type with the [current][245] [`Span`][246], returning an `Instrumented` wrapper. Read more

[Source][247]§

### impl<T, U> [Into][248]<U> for T

where U: [From][187]<T>,

[Source][249]§

#### fn [into][250](self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From][187]<T> for U` chooses to do.

§

### impl<T> PolicyExt for T

where T: ?[Sized][225],

§

#### fn and<P, B, E>(self, other: P) -> And<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] only if `self` and `other` return `Action::Follow`. Read more

§

#### fn or<P, B, E>(self, other: P) -> Or<T, P>

where T: Policy<B, E>, P: Policy<B, E>,

Create a new `Policy` that returns [`Action::Follow`] if either `self` or `other` returns `Action::Follow`. Read more

[Source][251]§

### impl<P, T> [Receiver][252] for P

where P: [Deref][11]<Target = T> \+ ?[Sized][225], T: ?[Sized][225],

[Source][253]§

#### type [Target][254] = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source][255]§

### impl<T> [Same][256] for T

[Source][257]§

#### type [Output][258] = T

Should always be `Self`

§

### impl<T> ServiceExt for T

§

#### fn propagate_header(self, header: HeaderName) -> PropagateHeader<Self>

where Self: [Sized][225],

Available on **crate feature`propagate-header`** only.

Propagate a header from the request to the response. Read more

§

#### fn add_extension<T>(self, value: T) -> AddExtension<Self, T>

where Self: [Sized][225],

Available on **crate feature`add-extension`** only.

Add some shareable value to [request extensions][259]. Read more

§

#### fn map_request_body<F>(self, f: F) -> MapRequestBody<Self, F>

where Self: [Sized][225],

Available on **crate feature`map-request-body`** only.

Apply a transformation to the request body. Read more

§

#### fn map_response_body<F>(self, f: F) -> MapResponseBody<Self, F>

where Self: [Sized][225],

Available on **crate feature`map-response-body`** only.

Apply a transformation to the response body. Read more

§

#### fn compression(self) -> Compression<Self>

where Self: [Sized][225],

Available on **crate features`compression-br` or `compression-deflate` or `compression-gzip` or `compression-zstd`** only.

Compresses response bodies. Read more

§

#### fn decompression(self) -> Decompression<Self>

where Self: [Sized][225],

Available on **crate features`decompression-br` or `decompression-deflate` or `decompression-gzip` or `decompression-zstd`** only.

Decompress response bodies. Read more

§

#### fn trace_for_http(self) -> Trace<Self, SharedClassifier<ServerErrorsAsFailures>>

where Self: [Sized][225],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using HTTP status codes. Read more

§

#### fn trace_for_grpc(self) -> Trace<Self, SharedClassifier<GrpcErrorsAsFailures>>

where Self: [Sized][225],

Available on **crate feature`trace`** only.

High level tracing that classifies responses using gRPC headers. Read more

§

#### fn follow_redirects(self) -> FollowRedirect<Self>

where Self: [Sized][225],

Available on **crate feature`follow-redirect`** only.

Follow redirect resposes using the [`Standard`][260] policy. Read more

§

#### fn sensitive_headers( self, headers: impl [IntoIterator][261]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<SetSensitiveResponseHeaders<Self>>

where Self: [Sized][225],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][262] on both requests and responses. Read more

§

#### fn sensitive_request_headers( self, headers: impl [IntoIterator][261]<Item = HeaderName>, ) -> SetSensitiveRequestHeaders<Self>

where Self: [Sized][225],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][262] on requests. Read more

§

#### fn sensitive_response_headers( self, headers: impl [IntoIterator][261]<Item = HeaderName>, ) -> SetSensitiveResponseHeaders<Self>

where Self: [Sized][225],

Available on **crate feature`sensitive-headers`** only.

Mark headers as [sensitive][262] on responses. Read more

§

#### fn override_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Insert a header into the request. Read more

§

#### fn append_request_header<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Append a header into the request. Read more

§

#### fn insert_request_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetRequestHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Insert a header into the request, if the header is not already present. Read more

§

#### fn override_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Insert a header into the response. Read more

§

#### fn append_response_header<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Append a header into the response. Read more

§

#### fn insert_response_header_if_not_present<M>( self, header_name: HeaderName, make: M, ) -> SetResponseHeader<Self, M>

where Self: [Sized][225],

Available on **crate feature`set-header`** only.

Insert a header into the response, if the header is not already present. Read more

§

#### fn set_request_id<M>( self, header_name: HeaderName, make_request_id: M, ) -> SetRequestId<Self, M>

where Self: [Sized][225], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension. Read more

§

#### fn set_x_request_id<M>(self, make_request_id: M) -> SetRequestId<Self, M>

where Self: [Sized][225], M: MakeRequestId,

Available on **crate feature`request-id`** only.

Add request id header and extension, using `x-request-id` as the header name. Read more

§

#### fn propagate_request_id( self, header_name: HeaderName, ) -> PropagateRequestId<Self>

where Self: [Sized][225],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses. Read more

§

#### fn propagate_x_request_id(self) -> PropagateRequestId<Self>

where Self: [Sized][225],

Available on **crate feature`request-id`** only.

Propgate request ids from requests to responses, using `x-request-id` as the header name. Read more

§

#### fn catch_panic(self) -> CatchPanic<Self, DefaultResponseForPanic>

where Self: [Sized][225],

Available on **crate feature`catch-panic`** only.

Catch panics and convert them into `500 Internal Server` responses. Read more

§

#### fn request_body_limit(self, limit: [usize][13]) -> RequestBodyLimit<Self>

where Self: [Sized][225],

Available on **crate feature`limit`** only.

Intercept requests with over-sized payloads and convert them into `413 Payload Too Large` responses. Read more

§

#### fn trim_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][225],

Available on **crate feature`normalize-path`** only.

Remove trailing slashes from paths. Read more

§

#### fn append_trailing_slash(self) -> NormalizePath<Self>

where Self: [Sized][225],

Available on **crate feature`normalize-path`** only.

Append trailing slash to paths. Read more

[Source][263]§

### impl<T> [ToOwned][264] for T

where T: [Clone][166],

[Source][265]§

#### type [Owned][266] = T

The resulting type after obtaining ownership.

[Source][267]§

#### fn [to_owned][268](&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more][268]

[Source][269]§

#### fn [clone_into][270](&self, target: [&mut T][200])

Uses borrowed data to replace owned data, usually by cloning. [Read more][270]

[Source][271]§

### impl<T> [ToString][272] for T

where T: [Display][183] \+ ?[Sized][225],

[Source][273]§

#### fn [to_string][274](&self) -> [String][156]

Converts the given value to a `String`. [Read more][274]

§

### impl<T> ToStringFallible for T

where T: [Display][183],

§

#### fn try_to_string(&self) -> [Result][128]<[String][156], [TryReserveError][275]>

[`ToString::to_string`][276], but without panic on OOM.

[Source][277]§

### impl<T, U> [TryFrom][204]<U> for T

where U: [Into][248]<T>,

[Source][278]§

#### type [Error][206] = [Infallible][279]

The type returned in the event of a conversion error.

[Source][280]§

#### fn [try_from][209](value: U) -> [Result][128]<T, <T as [TryFrom][204]<U>>::[Error][210]>

Performs the conversion.

[Source][281]§

### impl<T, U> [TryInto][282]<U> for T

where U: [TryFrom][204]<T>,

[Source][283]§

#### type [Error][284] = <U as [TryFrom][204]<T>>::[Error][210]

The type returned in the event of a conversion error.

[Source][285]§

#### fn [try_into][286](self) -> [Result][128]<U, <U as [TryFrom][204]<T>>::[Error][210]>

Performs the conversion.

§

### impl<V, T> VZip<V> for T

where V: MultiLane<T>,

§

#### fn vzip(self) -> V

§

### impl<T> WithSubscriber for T

§

#### fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self>

where S: [Into][248]<Dispatch>,

Attaches the provided [`Subscriber`][287] to this type, returning a [`WithDispatch`] wrapper. Read more

§

#### fn with_current_subscriber(self) -> WithDispatch<Self>

Attaches the current [default][288] [`Subscriber`][287] to this type, returning a [`WithDispatch`] wrapper. Read more

   [1]: ../../../axum/index.html
   [2]: index.html
   [3]: ../../index.html
   [4]: ../index.html
   [5]: ../../../src/axum/extract/ws.rs.html#621
   [6]: struct.Utf8Bytes.html (struct axum::extract::ws::Utf8Bytes)
   [7]: ../../../src/axum/extract/ws.rs.html#623-640
   [8]: ../../../src/axum/extract/ws.rs.html#627-629
   [9]: https://doc.rust-lang.org/nightly/std/primitive.str.html
   [10]: ../../../src/axum/extract/ws.rs.html#633-635
   [11]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html (trait core::ops::deref::Deref)
   [12]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#141
   [13]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
   [14]: https://doc.rust-lang.org/nightly/std/primitive.char.html (primitive char)
   [15]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#161
   [16]: https://doc.rust-lang.org/nightly/std/primitive.bool.html
   [17]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#361
   [18]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#410
   [19]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary (method str::is_char_boundary)
   [20]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#453
   [21]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.floor_char_boundary (method str::floor_char_boundary)
   [22]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#486
   [23]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
   [24]: https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html (fn core::str::converts::from_utf8)
   [25]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#562
   [26]: https://doc.rust-lang.org/nightly/std/primitive.pointer.html
   [27]: https://doc.rust-lang.org/nightly/std/primitive.u8.html (primitive u8)
   [28]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_mut_ptr (method str::as_mut_ptr)
   [29]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#606
   [30]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html (enum core::option::Option)
   [31]: https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html (trait core::slice::index::SliceIndex)
   [32]: https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output (type core::slice::index::SliceIndex::Output)
   [33]: https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None (variant core::option::Option::None)
   [34]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#671
   [35]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#757
   [36]: https://doc.rust-lang.org/nightly/std/primitive.str.html (primitive str)
   [37]: https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html (trait core::ops::index::Index)
   [38]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_mut_unchecked (method str::slice_mut_unchecked)
   [39]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#831
   [40]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut (method str::split_at_mut)
   [41]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked (method str::split_at_checked)
   [42]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#912
   [43]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked (method str::split_at_mut_checked)
   [44]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1050
   [45]: https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html (struct core::str::iter::Chars)
   [46]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1107
   [47]: https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html (struct core::str::iter::CharIndices)
   [48]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1130
   [49]: https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html (struct core::str::iter::Bytes)
   [50]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1182
   [51]: https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html (struct core::str::iter::SplitWhitespace)
   [52]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_ascii_whitespace (method str::split_ascii_whitespace)
   [53]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1233
   [54]: https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html (struct core::str::iter::SplitAsciiWhitespace)
   [55]: https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace (method char::is_ascii_whitespace)
   [56]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace (method str::split_whitespace)
   [57]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1297
   [58]: https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html (struct core::str::iter::Lines)
   [59]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1306
   [60]: https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html (struct core::str::iter::LinesAny)
   [61]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1326
   [62]: https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html (struct core::str::iter::EncodeUtf16)
   [63]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1351
   [64]: https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html (trait core::str::pattern::Pattern)
   [65]: https://doc.rust-lang.org/nightly/core/str/pattern/index.html (mod core::str::pattern)
   [66]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1389
   [67]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1414-1416
   [68]: https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher (type core::str::pattern::Pattern::Searcher)
   [69]: https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html (trait core::str::pattern::ReverseSearcher)
   [70]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1465
   [71]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1511-1513
   [72]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1639
   [73]: https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html (struct core::str::iter::Split)
   [74]: https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html (trait core::iter::traits::double_ended::DoubleEndedIterator)
   [75]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit (method str::rsplit)
   [76]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1680
   [77]: https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html (struct core::str::iter::SplitInclusive)
   [78]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1735-1737
   [79]: https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html (struct core::str::iter::RSplit)
   [80]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split (method str::split)
   [81]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1784
   [82]: https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html (struct core::str::iter::SplitTerminator)
   [83]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit_terminator (method str::rsplit_terminator)
   [84]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1830-1832
   [85]: https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html (struct core::str::iter::RSplitTerminator)
   [86]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_terminator (method str::split_terminator)
   [87]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1885
   [88]: https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html (struct core::str::iter::SplitN)
   [89]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplitn (method str::rsplitn)
   [90]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1934-1936
   [91]: https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html (struct core::str::iter::RSplitN)
   [92]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.splitn (method str::splitn)
   [93]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1954
   [94]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1973-1975
   [95]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2013
   [96]: https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html (struct core::str::iter::Matches)
   [97]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatches (method str::rmatches)
   [98]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2047-2049
   [99]: https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html (struct core::str::iter::RMatches)
   [100]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.matches (method str::matches)
   [101]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2091
   [102]: https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html (struct core::str::iter::MatchIndices)
   [103]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatch_indices (method str::rmatch_indices)
   [104]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2131-2133
   [105]: https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html (struct core::str::iter::RMatchIndices)
   [106]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.match_indices (method str::match_indices)
   [107]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2155
   [108]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2194
   [109]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2233
   [110]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2273
   [111]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2313
   [112]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2346-2348
   [113]: https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html (trait core::str::pattern::DoubleEndedSearcher)
   [114]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2393
   [115]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2427
   [116]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches (method str::trim_start_matches)
   [117]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2455-2457
   [118]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches (method str::trim_end_matches)
   [119]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2491-2493
   [120]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2531
   [121]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_prefix (method str::strip_prefix)
   [122]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2568-2570
   [123]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_suffix (method str::strip_suffix)
   [124]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2611-2613
   [125]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2655
   [126]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2698-2700
   [127]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2749
   [128]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html (enum core::result::Result)
   [129]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html (trait core::str::traits::FromStr)
   [130]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err (type core::str::traits::FromStr::Err)
   [131]: https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err (associated type core::str::traits::FromStr::Err)
   [132]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2770
   [133]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2782
   [134]: https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html (enum core::ascii::ascii_char::AsciiChar)
   [135]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_ascii (method str::is_ascii)
   [136]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2796
   [137]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2824
   [138]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2903
   [139]: https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace (method u8::is_ascii_whitespace)
   [140]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2928
   [141]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2954
   [142]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2997
   [143]: https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html (struct core::str::iter::EscapeDebug)
   [144]: https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_debug (method char::escape_debug)
   [145]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3043
   [146]: https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html (struct core::str::iter::EscapeDefault)
   [147]: https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_default (method char::escape_default)
   [148]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3081
   [149]: https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html (struct core::str::iter::EscapeUnicode)
   [150]: https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_unicode (method char::escape_unicode)
   [151]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3114
   [152]: https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html (struct core::ops::range::Range)
   [153]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.find (method str::find)
   [154]: https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3125
   [155]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#268
   [156]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html (struct alloc::string::String)
   [157]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#323
   [158]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#380
   [159]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#465
   [160]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#529
   [161]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#559
   [162]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_uppercase (method str::make_ascii_uppercase)
   [163]: https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#591
   [164]: https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_lowercase (method str::make_ascii_lowercase)
   [165]: ../../../src/axum/extract/ws.rs.html#620
   [166]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html (trait core::clone::Clone)
   [167]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone
   [168]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247
   [169]: https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from
   [170]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html (trait core::fmt::Debug)
   [171]: https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt
   [172]: https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html (struct core::fmt::Formatter)
   [173]: https://doc.rust-lang.org/nightly/core/fmt/type.Result.html (type core::fmt::Result)
   [174]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html (trait core::default::Default)
   [175]: https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default
   [176]: ../../../src/axum/extract/ws.rs.html#642-661
   [177]: ../../../src/axum/extract/ws.rs.html#658-660
   [178]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref
   [179]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target (type core::ops::deref::Deref::Target)
   [180]: ../../../src/axum/extract/ws.rs.html#643
   [181]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target
   [182]: ../../../src/axum/extract/ws.rs.html#663-668
   [183]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html (trait core::fmt::Display)
   [184]: ../../../src/axum/extract/ws.rs.html#665-667
   [185]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt
   [186]: ../../../src/axum/extract/ws.rs.html#702-707
   [187]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html (trait core::convert::From)
   [188]: ../../../src/axum/extract/ws.rs.html#704-706
   [189]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from
   [190]: ../../../src/axum/extract/ws.rs.html#695-700
   [191]: ../../../src/axum/extract/ws.rs.html#697-699
   [192]: ../../../src/axum/extract/ws.rs.html#688-693
   [193]: ../../../src/axum/extract/ws.rs.html#690-692
   [194]: ../../../src/axum/extract/ws.rs.html#709-714
   [195]: ../../../src/axum/extract/ws.rs.html#711-713
   [196]: ../../../src/axum/extract/ws.rs.html#716-731
   [197]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html (trait core::cmp::PartialEq)
   [198]: ../../../src/axum/extract/ws.rs.html#728-730
   [199]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq
   [200]: https://doc.rust-lang.org/nightly/std/primitive.reference.html
   [201]: https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264
   [202]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne
   [203]: ../../../src/axum/extract/ws.rs.html#670-677
   [204]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html (trait core::convert::TryFrom)
   [205]: ../../../src/axum/extract/ws.rs.html#671
   [206]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error
   [207]: https://doc.rust-lang.org/nightly/core/str/error/struct.Utf8Error.html (struct core::str::error::Utf8Error)
   [208]: ../../../src/axum/extract/ws.rs.html#674-676
   [209]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from
   [210]: https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error (type core::convert::TryFrom::Error)
   [211]: ../../../src/axum/extract/ws.rs.html#679-686
   [212]: https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html (struct alloc::vec::Vec)
   [213]: ../../../src/axum/extract/ws.rs.html#680
   [214]: ../../../src/axum/extract/ws.rs.html#683-685
   [215]: https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html (trait core::cmp::Eq)
   [216]: https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html (trait core::marker::StructuralPartialEq)
   [217]: https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html (trait core::marker::Freeze)
   [218]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html (trait core::panic::unwind_safe::RefUnwindSafe)
   [219]: https://doc.rust-lang.org/nightly/core/marker/trait.Send.html (trait core::marker::Send)
   [220]: https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html (trait core::marker::Sync)
   [221]: https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html (trait core::marker::Unpin)
   [222]: https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html (trait core::panic::unwind_safe::UnwindSafe)
   [223]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#138
   [224]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html (trait core::any::Any)
   [225]: https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html (trait core::marker::Sized)
   [226]: https://doc.rust-lang.org/nightly/src/core/any.rs.html#139
   [227]: https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id
   [228]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html (struct core::any::TypeId)
   [229]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212
   [230]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html (trait core::borrow::Borrow)
   [231]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214
   [232]: https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow
   [233]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221
   [234]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html (trait core::borrow::BorrowMut)
   [235]: https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222
   [236]: https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut
   [237]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547
   [238]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html (trait core::clone::CloneToUninit)
   [239]: https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549
   [240]: https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit
   [241]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785
   [242]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788
   [243]: ../trait.FromRef.html (trait axum::extract::FromRef)
   [244]: ../trait.FromRef.html#tymethod.from_ref
   [245]: super::Span::current()
   [246]: crate::Span
   [247]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769
   [248]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html (trait core::convert::Into)
   [249]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777
   [250]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into
   [251]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380
   [252]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html (trait core::ops::deref::Receiver)
   [253]: https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382
   [254]: https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target
   [255]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#34
   [256]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html (trait typenum::type_operators::Same)
   [257]: https://docs.rs/typenum/1.19.0/src/typenum/type_operators.rs.html#35
   [258]: https://docs.rs/typenum/1.19.0/typenum/type_operators/trait.Same.html#associatedtype.Output
   [259]: https://docs.rs/http/latest/http/struct.Extensions.html
   [260]: crate::follow_redirect::policy::Standard
   [261]: https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html (trait core::iter::traits::collect::IntoIterator)
   [262]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive
   [263]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74
   [264]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html (trait alloc::borrow::ToOwned)
   [265]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76
   [266]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#associatedtype.Owned
   [267]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77
   [268]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned
   [269]: https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81
   [270]: https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into
   [271]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893
   [272]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html (trait alloc::string::ToString)
   [273]: https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2895
   [274]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string
   [275]: https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html (struct alloc::collections::TryReserveError)
   [276]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string (method alloc::string::ToString::to_string)
   [277]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829
   [278]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831
   [279]: https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html (enum core::convert::Infallible)
   [280]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834
   [281]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813
   [282]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html (trait core::convert::TryInto)
   [283]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815
   [284]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error
   [285]: https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818
   [286]: https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into
   [287]: super::Subscriber
   [288]: dispatcher#setting-the-default-subscriber

