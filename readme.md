<img src="static/petal.jpeg" alt="petal logo" align="right"></img>
# Petal

> A Static Typechecker for Javascript

The project is still in its infancy, but the general goals as of typing this readme out are as follows:

1. Implement a parser for the working specification of the [proposal-type-annotations](https://github.com/tc39/proposal-type-annotations) tc39 proposal.
2. Implement a Static type checker with the following goals
   1. as-ml-as-possible type inference
   2. refinement types (and perhaps also dependent types, but as of now not really)
   3. higher kinded types
   4. focus on soundness
3. Type system that lints on a subset of javascript

### Highly flexible most-certainly-will-change RoadMap

- [ ]  Parse petal-specific grammar
   - [x] fork necessary swc crates to start work on parsing
   - [ ] implement the tentative grammar as outlined but the [type annotations proposal](https://tc39.es/proposal-type-annotations/grammar.html), 
         leveraging mostly existing Typescript parsing constructs in the swc crates
   - [ ] remove typescript syntax support from the swc crates, migrate to an ECMA type annotation proposal syntax
- [ ]  Implement basic type checking
   - [ ] everything to `unknown`
   - [ ] ... this is a far way off, no need to plan it out now :) 
   
### SWC

This project currently forks two crates from the [swc project](https://swc.rs/):

- [swc_ecma_parser](https://github.com/swc-project/swc/tree/main/crates/swc_ecma_parser) -> [swc_petal_parser](./crates/swc_petal_parser)
- [swc_ecma_ast](https://github.com/swc-project/swc/tree/main/crates/swc_ecma_ast) -> [swc_petal_asl](./crates/swc_petal_ast)

SWC is a battle-tested general purpose ecmascript compiler written in Rust. This project
forks these crates to build on top of great work.

Because this work is forked, Petal is licensed under Apache 2.0 to be in compliance of the SWC license.

The changes to the above crates are summarized below:

- swc_ecma_parser is augmented to support additional type annotation parsing
- swc_ecma_ast is augmented to support additional ast nodes associated with the additional type annotation parsing

## Musings

### Why another language / statically typed flavor of javascript?

Type systems are interesting to me, I want to learn more about type theory
and type checking with some hands-on experience.

I've written Typescript professionally for quite some time and enjoy the language 
tremendously. However, there are some gaps that I acknowledge _must_ exist
as they pertain to Typescript's goals and non-goals -- mostly having to do with
novel type constructs and soundness. These gaps have created an itch that I need to scratch.

On top of that, I've seen how slow transpiling massive javascript projects
can be. We're seeing a renaissance of native build tools and compilers to
fill this slow transpilation gap, but Typescript sticks out here.

Because there's no open specification for Typescript, anyone who wants 
to implement the type checker in something other than Typescript
has to either port the code, or conceptually reinvent the type-checker after
studying the source and docs closely. Tests have to be done via conformance suites 
that are based on the internal typescript test suites, and the cadence of features
is not stable.

None of this is the fault of Typescript. It just has an unfortunate side effect:
we're not going to see a widely-adopted native Typescript implementation anytime soon.

### Why implement the Type Annotations proposal grammar?

Currently, the cost of bootstrapping and adopting any statically typed javascript flavor is high. 
From an operational perspective, users need to adopt a pipeline that commutes the target language 
into valid javascript. From a language perspective, a compiler that emits valid javascript must 
be available and users are "locked in" to that language. They're no longer writing javascript, 
they're writing _language x_ that just happens to be a front-end for whatever JavaScript interpreter 
is going to get that compiled output.

With an ECMA specification for type annotations, users can "just write JavaScript." Typechecking 
becomes disjoint from any build pipelines: that javascript is runnable in modern browsers, and 
type annotation stripping is implemented exactly once in whatever build tool 
users have already adopted.

This makes new languages easy to try out and adopt: write javascript once, run the typechecker against it. 
It also relieves a burden from any new language author: who now at the bare minimum needs 
implement a type checker against some parsed AST that preserves type annotations. Ideally, that 
parser _already exists_.

There's a kind of beauty in this scenario, where _all valid javascript is syntactically
valid statically-typed javascript_. Petal explores that in the face of having _none_ of those
ideal scenarios.