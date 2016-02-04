# vkrs contribution guide

## Main conventions, workflow and project setup

### Coding conventions

- Use Unix line endings (single LF character).
- Use 4 spaces for indentation.
- Use only spaces for indentation (no tabulation).
- Preferred maximum line width is 140 characters, but try to keep lines width
  at most 120 characters (we all have large monitors these days, so no need to
  stick to old 80 columns rule).
- Try to follow Rust coding conventions in all other cases. Compiler will warn
  you when you don't follow simple rules for constants, traits and methods
  naming, just follow its recommendations.
- Use `rustfmt` to format code from time to time (preferable way is to run
  `cargo fmt`).

Also, the project has `.editorconfig` file with basic coding style definitions
readable by most IDEs and editors. Please look at [editorconfig.org][ec] for
details and find a plugin for your editor of choice in
[the download section][ec-dl]. If you install this plugin into your IDE/editor,
it will setup indentation, end-of-lines and other basic text formatting options
to conform the coding conventions described above. It will save you from many
(mis)formatting problems and help keep code clean and tidy.


### Environment setup

You will need stable Rust 1.6.0 or later. I follow stable Rust channel,
but try to check it on nightly channel to make sure the project is future
proof.

You will also need Cargo 0.7.0 or later. It comes with Rust 1.6.0.

You can download Cargo and Rust from the [Rust download page][rust-dl].

I recommend you to install [rustfmt][] for code formatting. Install it with
`cargo install rustfmt`. Make sure you have the directory Cargo installs
binaries in is in your `PATH`. After this, you can use `cargo fmt` command in
the project directory to format code according code conventions.

And of cause you will need [Git][]. No excuses here, you must now Git basics in
order to work with Github.

[rust-dl]: https://www.rust-lang.org/downloads.html
[rustfmt]: https://crates.io/crates/rustfmt/
[Git]: https://git-scm.com/

### Project setup

Fork the [main project repo][vkrs] into your Github account, then clone it and
add upstream remote to keep it in sync with upstream:

```bash
$ git clone git@github.com:$USER/vkrs.git
$ cd vkrs
$ git remote add upstream https://github.com/kstep/vkrs
```

OK, you are all set now!

[vkrs]: https://github.com/kstep/vkrs
[ec]: http://editorconfig.org/
[ec-dl]: http://editorconfig.org/#download

### Contribution workflow

To start contribution, you will need to find some task to do. Visit the
[project issues page][issues] and look for [unassigned issues with `s-help
wanted` label][search].

Find some issue you are interested in and comment on it to let me know you want
to take it. I will assign it to you and will try guide you in your first steps
to contribution. It's OK if you don't know Rust, or systems programming, or
some tricky Git workflow, whatever it is, I'll be happy to help you, answer
your questions if I can, or introduce you to forums and chats where more wise
people will be able to help you, if I can't.

Once you have your first task chosen, create a branch in your repository. You
may name it however you want, just make sure its name somehow reflect the
issue problem in one-to-three short words. It's also a good idea to place
number in the branch name, e.g. `issue-123`, as it will help everybody keep
track of your progress on the task:

```bash
$ git checkout -b issue-123-fix-bug
```

Then you are free to hack. Commit often, try to place meaningful names into
your commit messages, keep your commits small and try to place logically linked
changes into single commit, while keeping unrelated changes into different
commits.

Once you are done, it's good idea to make sure your code is formatted according
coding conventions by running `rustfmt` on it:

```bash
$ cargo fmt
$ git commit -a -m "rustfmt"
```

And then push your branch to Github:

```bash
$ git push origin -u issue-123-fix-bug
```

Then [create a Pull Request][PR] to let me know you are ready to merge your changes.
I'll review your commits if they are all OK, I'll merge them. If not, I'll try
to point you to things you may need to change. It may take several iterations
and new commits to make your changes fit the project, but once everything is
OK, PR will be merged.

After successful merge, you are safe to remove the issue branch:

```bash
$ git branch -d issue-123-fix-bug     # remove local branch
# git push origin :issue-123-fix-bug  # remove remote branch
```

Don't forget to update your fork to the upstream after this:

```bash
$ git checkout master
$ git pull --rebase upstream master
$ git push origin master
```

Then you are ready to take on the next issue. Good luck!

[issues]: https://github.com/kstep/vkrs/issues
[search]: https://github.com/kstep/vkrs/issues?utf8=%E2%9C%93&q=is%3Aopen+label%3A%22s-help+wanted%22+no%3Aassignee
[PR]: https://github.com/kstep/vkrs/compare

## Project architecture description

This section contains in depth discussion on project architecture and VK API
shape. It may seem too long and complex, so you may want to skip it and learn
the project architecture the hard way by jumping in directly to contribution,
but try at least skim it to have some common overview of the project shape.

### Basic VK API description

The project is about [VK API][vkapi] implementation in Rust. The VK API is
quite large, so one need to keep to some strict conventions to avoid confusion
(not to say "avoid to get crazy").

The API requests are all made with either GET or POST HTTP methods,
interchangeably. The current implementation always uses GET requests, but it
may change in the future. OAuthy authorization is used, and some requests
require certain permissions and an access token, while others require neither.

Request bodies are always www-form-urlencoded strings, responses are always JSON
encoded (they may be XML encoded by client request, but this feature is never
used in the client implementation).

Request bodies define response structure in a strict way, so once request
is known, we can expect some definite JSON structure in response.

The VK API is also versioned, API version is passed in a `v` argument in any
request and may affect the response structure. So each request is also marked
with some API version.

So the client API as implemented in this library revolves around two concepts:

- Requests, which define API method to call and arguments be sent to the server,
- Responses, which define server responses shape.

Each API request is represented with some `struct`, which implements `Request`
trait. Request structs keep fields to represent query to be sent. They also
define expected response type.

Each API response is represented with some `struct`, which implements
`serde::de::Deserialize` trait.

Request structures follow inline builder pattern, and define at least `new()`
constructor method without arguments, which sets up sensible defaults in the
fields.

So in order to implement some API method, one must do the following:

- create request structure type,
- implement builder pattern on it (that is an `fn new() -> Self` method and
  setter methods `fn field_name(&mut self, value: T) -> &mut Self` for each
  field),
- create response structure type,
- implement `Request` trait on request type (i.e. set `Response` associated
  type to the response type, implement `fn to_query_string(&self) -> String`
  method to build query string, implement `fn method_name() -> &'static str'`
  method which must return method name (e.g. `"audio.get"`), and optionally
  implement `fn permissions() -> Permissions` method which returns permissions
  set required by the request).
- and then implement `serde::de::Deserialize` trait for the response type.

That sounds like a lot of work, but hold on! There is some remedy to it ahead.

[vkapi]: https://vk.com/dev/methods

### Project layout and items naming conventions

This section is subject to change, as the project is still evolving, so it may
lag behind real project structure. In case of confusion, take real code layout
for guidance.

So, if you followed through previous section, you now have some high level
vision of the project's architecture. Now let's map it to actual code.

I use [serde][] and [serde_json][] to deserialize responses. To avoid
boilerplate code I use code generation with macros. I use [syntex][] to
generate code in stable Rust.

Fear not, my friend, you won't have to worry about it very much, as the
framework is ready, and all you need to do, is just follow some simple steps.

So main layout is simple:

- all common API related things are already in `api` module (`api.rs` and
  `api.rs.in` files), you won't need to touch it very much, if at all, but you
  will need to use some parts of it often;
- all main API parts go to a module with the same name, as the API part,
  e.g. [audio API][audio] go to `audio` module (`audio.rs` and `audio.rs.in`),
  [photos API][photos] go to `photos` module (`photos.rs` and `photos.rs.in`),
  etc.;

You may notice, that modules come in pairs: `*.rs` and `*.rs.in` files. The
`*.rs.in` files are processed with [serde_codegen] code generator during build
process, so all response types go to `*.rs.in` files, so serde_codgen can
generate `Deserialize` trait for them.

Request types and all helper types which doesn't come with response (and don't)
require `Deserialize` trait implementation) go to `*.rs` files.

To join up the files during compilation, you will need to place the following
snippet after `use` clauses into your `*.rs` file:

```rust
#[cfg(feature = "unstable")]
include!("filename.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/filename.rs"));
```

Use your actual `*.rs` file name instead of `filename.rs`.

Now to the naming conventions:

- API parts are put into modules with the names matching API part name
  (`audio`, `photos`, `market`, etc).
- Request types are named after API method names without API part name
  prefix (`audio.get` → `Get`, `utils.getServerTime` → `GetServerTime`, etc.).
- Response types are named after return object names (`User`, `Photo`, `Audio`).
  If response type name conflicts with request type name in the same module
  (e.g. `Post` method (as in `wall.post`) and `Post` response object),
  add module prefix to the response type name (`WallPost`).
- Some request or response fields may have names matching Rust keywords, most
  notable such name is `type`. In such a case, for response type field rename
  it to some sensible synonym (`type` is renamed to `kind` by convention) and
  use `#[serde(rename="orignal_name")]` attribute to set real response field
  name. For request fields, you will use extended `request!` or `request_ref!`
  macro with aliases (more on this later). If in doubt about good synonym,
  search project for original field name and take already used synonym to keep
  field names consistent.

[serde]: https://crates.io/crates/serde/
[serde_json]: https://crates.io/crates/serde_json/
[serde_codegen]: https://crates.io/crates/serde_codegen/
[syntex]: https://crates.io/crates/syntex/
[audio]: https://vk.com/dev/audio

### Main API request implementation workflow

So, first things first. You need to find or create a module for the API method to place.
If you read the previous section carefully, you now know where to place all these things.
Let's say you want to implement [audio.search][] method.

You need to find (or create if they're not there) the following files:

- `src/audio.rs` for requests and other main parts,
- `src/audio.rs.in` for responses to be processed with serde_codegen (so you
  don't to implement Deserialize trait manually).

In our case the files are already here. But what if they would be already in place?
Well, you will need to create them. Let's leave `audio.rs.in` empty for now. Now place
the following snippet into `audio.rs` file:

```rust
#[cfg(feature = "unstable")]
include!("audio.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/audio.rs"));
```

This snippet will join up both files into single module during compilation.
Place all imports (`use` clauses) above this code, and all other things below this code.

Now add the following line into `lib.rs` file in order to make this module visible:

```rust
pub mod audio;
```

Congratulations! You have just set up basic module structure.

Now to the fun part.

As mentioned above you will need to do the following to implement VK API method:

- create request structure type (this will be in `audio.rs` in our example),
- implement builder pattern on it (to `audio.rs`),
- create response structure type (this will go to `audio.rs.in`),
- implement `Request` trait on request type (also in `audio.rs`),
- implement `serde::de::Deserialize` trait for the response type (will be done
  automagically with code generation framework).

#### Standard library traits sidenote

Just one more thing before we continue.

Note, also, I try to implement as much standard traits on public types as
possible. Usual minimum I implement via `#[derive()]` is `Debug`, `PartialEq`
and `Clone`. Also, if possible, I implement `Copy` (if all type fields are
copyable) and `Eq` (almost always, except for some rare cases when a type
contains something like floating number fields (`f32` or `f64`) which are not
`Eq`). However it's all about putting correct `#[derive()]` clause before type
definition, so hustle is minimum. Here's a snippet:

```
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
```

Just put it before every request and response type and remove from it traits
compiler complains about.

#### Request structure implementation

Requests implementation is made really simple due to two wonderful macros I
prepared for you. Behold `request!` and `request_ref!` Both of them define
simple DSL to describe all aspects of VK API method request, that is: method
name, hardcoded fields (like version field `v`), argument fields, response type
and required permissions.

The two macros are very similar, the only difference is `request_ref!` allows
you to put fields with reference types into your generated request structure
(hence the suffix `_ref`), while the `request!` macro generates a simple
structure type without references and lifetimes.

Now to the code:

```rust
request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["audio.search"](v => 5.44) -> Collection<Audio> [Audio] {
        sized {
            auto_complete: bool = () => {bool},
            lyrics: bool = () => {bool},
            performer_only: bool = () => {bool},
            sort: Sort = (Sort::Popularity) => {AsRef},
            search_own: bool = () => {bool},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}
```

This snippet shows of almost all main aspects of the `request_ref!` macros. We
use `_ref` version here because of string field `q: &str`. We could use
`request!` macro if we chose `q` to be `String`, but we don't want to make our
users allocate strings for us, so we will just borrow the string. Now let's go
through the DSL step by step.

First we define some custom derives (`#[derive(Eq, Copy)]`). Note the macro
will add derived implementations for `PartialEq`, `Clone` and `Debug` traits
for us, these two traits a left out because some types may contain not-Copy
field types (like `String`) and/or non-Eq field types (`f32`). In our case
there are no such fields, so we just go on and out-in for these useful derives.
Remember, if we don't do it, our library users won't be able to do it for us
due to orphan trait implementation rule.

Then we define a struct in usual way. We don't mark it as `pub`lic, macro will
do it for us. We name our structure `Search` according to our conventions
(remember we are defining `audio.search` method, and `audio` part is
already encoded in our module name). We define this struct `for
["audio.search"]` with hardcoded argument `(v => 5.44)`. We could move this
`v` into struct fields and let user change version number in runtime, but
it will ruin our queries flow, because version defines response type, so if
we give user this freedom to choose API version, we will loose our freedom
to set response type structure. So we hardcode it in our implementation.
The same thing can have sense for other fields, e.g. `exteded` fields for in
requests change the available fields drastically, redefining response structure
in a very different way, so we will hard code such structure defining fields.

Now we set the response type to `-> Collection<Audio>`. The generic
`Collection<T>` type resides in `api` module so you will have to put `use
api::Collection;` to the top of your module. This type defines basic collection
response layout and contains `count` and `items` fields. You will see and use
it very often, actually almost always when a method returns some collection
of objects. If our method were to return single object, we would write
`-> Audio` here instead.

After it we see `[Audio]` slice-like thingy. It's a permission definition
block. It can actually contain a comma-separated list of permission names, as
defined in `enum api::Permission` type. In our case the `audio.search` method
requires "audio" permission, defined by `api::Permission::Audio` variant. If
our method required several permissions, e.g. both "audio" and "video", we
would write `[Audio, Video]` here. This block can be empty or omitted
altogether for public methods which doesn't require authorization and/or
special permissions, like `utils.getServerTime`.

Now we go to fields definition, and we wrap them into curly braces `{}` as we
would for simple struct type. Let's ignore `sized {}` and `unsized {}` blocks
for now and go straight to field definitions.

Fields are defined in a way very similar to usual struct fields definition,
but with some extra meta info stuck in. Let's look at it closer:

```rust
auto_complete: bool = () => {bool},
```

First comes field name and type, no surprises here. Then we have an equal sign,
very similar to variable assignment syntax. The expression in round brackets
after it defines the default for the field. It may be some value of correct
type, e.g. `(true)` or `(false)` in this case. Empty pair of brackets means
"default value" for the given type, and is a synonym to `(Default::default())`.

Then we have a fat arrow and an expression in curly braces. This syntax defines
an expression to map the field value to some `&str` value, which will go to
query string. The value in curly braces can be one of the following special
keywords, which define some common mapping pattern:

- `{}` (empty braces) — the same as `{ToString}`.
- `{ToString}` — convert value into a string with `String::to_string()` method.
  Not very efficient so not recommended to string fields, but works well for
  numeric fields.
- `{Borrow}` — convert value into a `&str` with a `Borrow::borrow()` method.
  The field type must implement `Borrow<str>` trait. Very efficient for
  `String` fields, should be the default for them.
- `{AsRef}` — convert value into a `&str` with a `AsRef::as_ref()` method. As
  efficient, as `{Borrow}` above. The field type must implement `AsRef<str>`
  trait. My usual choice for different enum types, for which I just implement
  `AsRef<str>` which returns hard coded `&'static str` for each given variant.
- `{bool}` — for boolean fields only. If boolean field value is true, produces
  `"1"` value, otherwise `"0"`.
- `{AsRef<Vec>}` — the same as `AsRef` above, but for iterable fields (of type
  `&[T]` or `Vec<T>`), where collection elements implement `AsRef<str>`. This
  mapper will iterate over the collection, converting each element into `&str`
  with `AsRef::as_ref()`, and then join up all strings into a single one with a
  comma as a glue.
- `{AsRef<Option>}` — for `Option<T: AsRef<str>>` fields. If the field value is
  `None`, it will convert it to the empty string (`""`), otherwise it works
  just like `{AsRef}` above.
- `{Vec}` — the same as `{AsRef<Vec>}` above, but uses `ToString::to_string()`
  instead of `AsRef::as_ref()` to convert collection elements into strings.
- `{Option}` — the same as `{AsRef<Option>}` above, but uses
  `ToString::to_string()` instead of `AsRef::as_ref()` to convert collection
  elements into strings.
- `{=}` — pass the field value intact. The field must already be of type `&str`.

Also it can have closure-like syntax like this:

```rust
album_id: Option<Id> = () => { |value| value.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or("") },
```

In this case the mapper block contains variable name in pipes `|value|`, which
will have `&T` type, where `T` is the type of the field (`&Option<Id>` in this
concrete case), and then an expression, which must transform this variable into
a `&str`.

Now to the `sized` and `unsized` blocks. These blocks are special to
`request_ref!` macro, the `request!` macro has field definitions list directly
in its fields block:

```rust
request! {
    #[derive(Copy, Eq)]
    struct GetLyrics for ["audio.getLyrics"](v => 5.44) -> Lyrics [Audio] {
        lyrics_id: Id = () => {}
    }
}
```

(Try to read this DSL as a home work, it shouldn't be difficult after you have
gone through all the words above.)

As block names say, they define sized and unsized request fields. The fields
definition syntax in them are the same, except for one small but important
detail: in `unsized {}` block all field types *can* (but *don't have to*) have
unsized types, like `str` or `[T]`. The macro will prepend these types with
reference prefix `&` and a lifetime specifier, bound to the parent structure.
That means the following definition:

```rust
q: str = ("") => {=},
```

Will generate the following field in our `Search` struct:

```rust
q: &str,
```

One last thing to mention. If `request_ref!` structure contains only "unsized"
fields, you can omit `sized` and `unsized` blocks and just put fields
definitions directly in fields definition block. E.g. if our `Search` struct
contained only `q` field, we could write it down this way:

```rust
request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["audio.search"](v => 5.44) -> Collection<Audio> [Audio] {
        q: str = ("") => {=},
    }
}
```

So this would generate the following structure type:

```rust
#[derive(PartialEq, Debug, Clone, Eq, Copy)]
pub struct Search<'a> {
    q: &'a str,
}
```

Now we are done with request. So, why all this hustle with custom DSL, you would ask?
Well, let's me show you the code again:

```rust
request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["audio.search"](v => 5.44) -> Collection<Audio> [Audio] {
        sized {
            auto_complete: bool = () => {bool},
            lyrics: bool = () => {bool},
            performer_only: bool = () => {bool},
            sort: Sort = (Sort::Popularity) => {AsRef},
            search_own: bool = () => {bool},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}
```

This sixteen lines have just generated the following things for us:

1. the `struct Search<'a> { ... }` with 8 fields (it would take 11 lines of
   code including derive attribute),
2. the `Struct::new()` constructor which set all fields to sensible defaults we
   just provided in the DSL (another ~12 lines of code),
3. a setter method for *each* field (at least 3 lines of code per field, which
   amounts to 24 lines),
4. and `Request` trait implementation (~7 lines + at least 1 line per field,
   that is about 15 lines of code, if you write short methods in one line, and
   more if you format them nicely).

This amounts to 62 simple lines of code, which comments and empty lines, using
one-liner methods, vs. 16 lines of macro invocation.

Yes, the DSL is very dense, but it describes exactly what request does in a
reasonable enough way, so I bet you already can read all (or almost all)
request definitions in the project, and even compose several yourself.

[audio.search]: https://vk.com/dev/audio.search

#### Response structure implementation

This one is even simpler. You just take response JSON and write it down as as
`pub struct Something {}` into `*.rs.in` file (that would be `audio.rs.in` in
our example). And you will use this `Something` after arrow `->` in some
request definition DSL clause.

Then you just put the following snippet right above your new structure type
definition:

```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
```

And you are done.

Note `Deserialize` trait (which is required for response types!). Also note that
it doesn't include `Copy` trait, as responses usually contain at least one `String`
field, which is not copyable. If you response doesn't have non-copyable fields,
add `Copy` to derived traits as well. And yes, if your response contains non-Eq
fields (like floats), remove `Eq` from derive attribute.

#### List of helpful/important predefined types

Now to the last point. This will be really simple, I promise.

There are some helpful types around this library, you will need to know about
in order to use them instead of reinventing a wheel.

Some of the types I already mentioned above, e.g. `api::Collection<T>` and
`auth::Permission`.

There are also several types and type aliases which add some semantic meaning
to response and request fields. You should use them instead of simple types
where appropriate. I.e. if you have some `user_id` field, don't just set its
type to `u64` mindlessly, use `api::Id` type instead. It will make the code
more semantic and will save some hustle in the future if I want to change type
aliases to some more complex types.

Here's the full list:

- `api::Collection<T: Deserialize>`. You already met it above. Almost all
  collections, which come from VK, are of this type. Remember, it has two
  fields: `count` (with the total number of items in the collection) and
  `items` (just a `Vec<T>`, which contains some subset of the items, cut from
  the total collection with, for example, `offset` and `count` method arguments
  (for "audio.search" and many other "*.get" and "*.search" methods)).
- `auth::Permission` — describes all available API permissions. You won't
  normally modify it, just read it to know what permissions you can put into
  request permissions definition block.
- `auth::Permissions` — just a compact set of permissions. Just know it exists,
  and don't worry about it. You will rarely use it anywhere, it's more for
  library users, than contributors.
- `api::OwnerId`, `api::Id`, `api::FullId` — different kinds of ids. `OwnerId`
  is the type for `owner_id` fields, which can be negative for group ids and
  positive for user ids (hence it's just an alias to `i64` now). All other
  `*_id` fields should have `Id` type (a.k.a. `u64`). The `FullId` type is
  usually used in requests and represents a combination of owner id and object
  id, that is it's just an alias to `(OwnerId, Id)` tuple for now.
- `api::Date` — a date-time fields type.
- `api::Duration` — a duration type.
- `api::Bool` — a boolean type, for use in responses only. It's just an alias
  to `u8` now, as VK returns 1/0 instead of true/false. Maybe it will be
  removed in the future.
- `api::ErrorCode` — this one may be important for you. It's an enum type which
  enumerates all possible API error codes. If you meet some new error codes in
  API method description, and it's missing from this enum, please add it into
  it. See below for details.
- `api::Error` — a common library errors enum. For library users. Just know it
  exists.
- `api::Result` — a convenience alias to `Result<T, Error>`, the main return
  type from the request. For library users, so just know it exists.


#### New error codes definition

The `enum api::ErrorCode` type described above contains a full list of VK API
error codes, including error descriptions in its `Display` trait implementation.

It's quite comprehensive, but some methods return very specific error codes
(e.g. [wall.post][] method). If you ever meet some method, which defines custom
error codes, please look at the `ErrorCode` type, and if the codes are missing
from it, you will need to add them into the enum.

Please try to keep enum variant names short, but concise and meaningful. The
longest error code variant name is 26 characters long, please don't try to beat
this record! Use shorter names. Use existing variant names as a guidance.

If you add new error code into `ErrorCode` type, you will also need to add it
to `From<u32>`, `Into<u32>` and `Display` traits implementations for
`ErrorCode`. Place short and meaningful text description in `Display` trait
implementation for the new codes. A good default is to just take the error
description from VK docs page and put it into `f.write_str()` in lower case.

[wall.post]: https://vk.com/dev/wall.post
