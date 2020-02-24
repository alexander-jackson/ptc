   Compiling ptc v0.1.0 (/Users/alexander/Documents/University/Programming/rust/ptc)
warning: Constants have by default a `'static` lifetime
  --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:51:22
   |
51 |     const __ACTION: &'static [i16] = &[
   |                     -^^^^^^^------ help: consider removing `'static`: `&[i16]`
   |
   = note: `#[warn(clippy::redundant_static_lifetimes)]` on by default
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes

warning: Constants have by default a `'static` lifetime
   --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:379:26
    |
379 |     const __EOF_ACTION: &'static [i16] = &[
    |                         -^^^^^^^------ help: consider removing `'static`: `&[i16]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes

warning: Constants have by default a `'static` lifetime
   --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:707:20
    |
707 |     const __GOTO: &'static [i16] = &[
    |                   -^^^^^^^------ help: consider removing `'static`: `&[i16]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes

warning: Constants have by default a `'static` lifetime
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1036:28
     |
1036 |         const __TERMINAL: &'static [&'static str] = &[
     |                           -^^^^^^^--------------- help: consider removing `'static`: `&[&'static str]`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes

warning: Constants have by default a `'static` lifetime
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1036:38
     |
1036 |         const __TERMINAL: &'static [&'static str] = &[
     |                                     -^^^^^^^---- help: consider removing `'static`: `&str`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_static_lifetimes

warning: All the struct fields are matched to a wildcard pattern, consider using `..`.
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1218:13
     |
1218 |             lexer::Tok::Float { value: _ } if true => Some(29),
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(clippy::unneeded_field_pattern)]` on by default
     = help: Try with `Float { .. }` instead
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unneeded_field_pattern

warning: All the struct fields are matched to a wildcard pattern, consider using `..`.
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1220:13
     |
1220 |             lexer::Tok::Identifier { name: _ } if true => Some(31),
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: Try with `Identifier { .. }` instead
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unneeded_field_pattern

warning: All the struct fields are matched to a wildcard pattern, consider using `..`.
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1223:13
     |
1223 |             lexer::Tok::Integer { value: _ } if true => Some(34),
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
     = help: Try with `Integer { .. }` instead
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unneeded_field_pattern

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1242:67
     |
1242 |                 __tok @ lexer::Tok::Newline => __Symbol::Variant0((__tok)),
     |                                                                   ^^^^^^^
     |
     = note: `#[warn(clippy::double_parens)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1246:68
     |
1246 |                 __tok @ lexer::Tok::NotEqual => __Symbol::Variant0((__tok)),
     |                                                                    ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1250:66
     |
1250 |                 __tok @ lexer::Tok::Modulo => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1254:72
     |
1254 |                 __tok @ lexer::Tok::ModuloEquals => __Symbol::Variant0((__tok)),
     |                                                                        ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1258:64
     |
1258 |                 __tok @ lexer::Tok::LPar => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1262:64
     |
1262 |                 __tok @ lexer::Tok::RPar => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1266:68
     |
1266 |                 __tok @ lexer::Tok::Multiply => __Symbol::Variant0((__tok)),
     |                                                                    ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1270:74
     |
1270 |                 __tok @ lexer::Tok::MultiplyEquals => __Symbol::Variant0((__tok)),
     |                                                                          ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1274:64
     |
1274 |                 __tok @ lexer::Tok::Plus => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1278:70
     |
1278 |                 __tok @ lexer::Tok::PlusEquals => __Symbol::Variant0((__tok)),
     |                                                                      ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1282:65
     |
1282 |                 __tok @ lexer::Tok::Comma => __Symbol::Variant0((__tok)),
     |                                                                 ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1286:65
     |
1286 |                 __tok @ lexer::Tok::Minus => __Symbol::Variant0((__tok)),
     |                                                                 ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1290:71
     |
1290 |                 __tok @ lexer::Tok::MinusEquals => __Symbol::Variant0((__tok)),
     |                                                                       ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1294:63
     |
1294 |                 __tok @ lexer::Tok::Dot => __Symbol::Variant0((__tok)),
     |                                                               ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1298:66
     |
1298 |                 __tok @ lexer::Tok::Divide => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1302:72
     |
1302 |                 __tok @ lexer::Tok::DivideEquals => __Symbol::Variant0((__tok)),
     |                                                                        ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1306:65
     |
1306 |                 __tok @ lexer::Tok::Colon => __Symbol::Variant0((__tok)),
     |                                                                 ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1310:69
     |
1310 |                 __tok @ lexer::Tok::Semicolon => __Symbol::Variant0((__tok)),
     |                                                                     ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1314:64
     |
1314 |                 __tok @ lexer::Tok::Less => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1318:71
     |
1318 |                 __tok @ lexer::Tok::LessOrEqual => __Symbol::Variant0((__tok)),
     |                                                                       ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1322:66
     |
1322 |                 __tok @ lexer::Tok::Assign => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1326:65
     |
1326 |                 __tok @ lexer::Tok::Equal => __Symbol::Variant0((__tok)),
     |                                                                 ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1330:67
     |
1330 |                 __tok @ lexer::Tok::Greater => __Symbol::Variant0((__tok)),
     |                                                                   ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1334:74
     |
1334 |                 __tok @ lexer::Tok::GreaterOrEqual => __Symbol::Variant0((__tok)),
     |                                                                          ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1338:67
     |
1338 |                 __tok @ lexer::Tok::LSquare => __Symbol::Variant0((__tok)),
     |                                                                   ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1342:67
     |
1342 |                 __tok @ lexer::Tok::RSquare => __Symbol::Variant0((__tok)),
     |                                                                   ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1346:70
     |
1346 |                 __tok @ lexer::Tok::LogicalAnd => __Symbol::Variant0((__tok)),
     |                                                                      ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1350:63
     |
1350 |                 __tok @ lexer::Tok::Def => __Symbol::Variant0((__tok)),
     |                                                               ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1354:64
     |
1354 |                 __tok @ lexer::Tok::Else => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1358:75
     |
1358 |                 lexer::Tok::Float { value: __tok0 } => __Symbol::Variant1((__tok0)),
     |                                                                           ^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1362:66
     |
1362 |                 __tok @ lexer::Tok::Global => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1366:79
     |
1366 |                 lexer::Tok::Identifier { name: __tok0 } => __Symbol::Variant2((__tok0)),
     |                                                                               ^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1370:62
     |
1370 |                 __tok @ lexer::Tok::If => __Symbol::Variant0((__tok)),
     |                                                              ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1374:66
     |
1374 |                 __tok @ lexer::Tok::Indent => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1378:77
     |
1378 |                 lexer::Tok::Integer { value: __tok0 } => __Symbol::Variant3((__tok0)),
     |                                                                             ^^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1382:70
     |
1382 |                 __tok @ lexer::Tok::LogicalNot => __Symbol::Variant0((__tok)),
     |                                                                      ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1386:69
     |
1386 |                 __tok @ lexer::Tok::LogicalOr => __Symbol::Variant0((__tok)),
     |                                                                     ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1390:64
     |
1390 |                 __tok @ lexer::Tok::Pass => __Symbol::Variant0((__tok)),
     |                                                                ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1394:66
     |
1394 |                 __tok @ lexer::Tok::Return => __Symbol::Variant0((__tok)),
     |                                                                  ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1398:68
     |
1398 |                 __tok @ lexer::Tok::Unindent => __Symbol::Variant0((__tok)),
     |                                                                    ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: Consider removing unnecessary double parentheses
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1402:65
     |
1402 |                 __tok @ lexer::Tok::While => __Symbol::Variant0((__tok)),
     |                                                                 ^^^^^^^
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: returning the result of a `let` binding from a block
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2251:13
     |
2245 | /             let __r = __state_machine::Parser::drive(
2246 | |                 __StateMachine {
2247 | |                     __phantom: ::std::marker::PhantomData::<()>,
2248 | |                 },
2249 | |                 __tokens,
2250 | |             );
     | |______________- unnecessary `let` binding
2251 |               __r
     |               ^^^
     |
     = note: `#[warn(clippy::let_and_return)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#let_and_return
help: return the expression directly
     |
2245 |             
2246 |             __state_machine::Parser::drive(
2247 |                 __StateMachine {
2248 |                     __phantom: ::std::marker::PhantomData::<()>,
2249 |                 },
2250 |                 __tokens,
   ...

warning: calling `to_string` on `&&str`
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1083:22
     |
1083 |                 Some(terminal.to_string())
     |                      ^^^^^^^^^^^^^^^^^^^^ help: try dereferencing the receiver: `(*terminal).to_string()`
     |
     = note: `#[warn(clippy::inefficient_to_string)]` on by default
     = help: `&str` implements `ToString` through a slower blanket impl, but `str` has a fast specialization of `ToString`
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#inefficient_to_string

warning: the function has a cognitive complexity of (43/25)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1182:8
     |
1182 |     fn __token_to_integer<
     |        ^^^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(clippy::cognitive_complexity)]` on by default
     = help: you could split it up into multiple smaller functions
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#cognitive_complexity

warning: the function has a cognitive complexity of (43/25)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:1233:8
     |
1233 |     fn __token_to_symbol<
     |        ^^^^^^^^^^^^^^^^^
     |
     = help: you could split it up into multiple smaller functions
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#cognitive_complexity

warning: you should consider deriving a `Default` implementation for `parser::__parse__Program::ProgramParser`
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2228:9
     |
2228 | /         pub fn new() -> ProgramParser {
2229 | |             ProgramParser {
2230 | |                 _priv: (),
2231 | |             }
2232 | |         }
     | |_________^
     |
     = note: `#[warn(clippy::new_without_default)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#new_without_default
help: try this
     |
2223 |     #[derive(Default)]
2224 |     pub struct ProgramParser {
     |

warning: redundant closure found
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2244:45
     |
2244 |             let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove closure as shown: `__ToTriple::to_triple`
     |
     = note: `#[warn(clippy::redundant_closure)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_closure

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2669:31
     |
2669 |                 let __start = __sym0.0.clone();
     |                               ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = note: `#[warn(clippy::clone_on_copy)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2670:29
     |
2670 |                 let __end = __sym0.2.clone();
     |                             ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2934:23
     |
2934 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2935:21
     |
2935 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2950:48
     |
2950 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2951:66
     |
2951 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2968:23
     |
2968 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2969:21
     |
2969 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2984:48
     |
2984 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:2985:66
     |
2985 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3001:23
     |
3001 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3002:21
     |
3002 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3019:23
     |
3019 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3020:21
     |
3020 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3038:23
     |
3038 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3039:21
     |
3039 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3057:23
     |
3057 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3058:21
     |
3058 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3076:23
     |
3076 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3077:21
     |
3077 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3092:48
     |
3092 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3093:66
     |
3093 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3110:23
     |
3110 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3111:21
     |
3111 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3126:48
     |
3126 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3127:66
     |
3127 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3143:23
     |
3143 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3144:21
     |
3144 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3161:23
     |
3161 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3162:21
     |
3162 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3180:23
     |
3180 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3181:21
     |
3181 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3198:23
     |
3198 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3199:21
     |
3199 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3214:48
     |
3214 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3215:66
     |
3215 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3231:23
     |
3231 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3232:21
     |
3232 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3249:23
     |
3249 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3250:21
     |
3250 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3268:23
     |
3268 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3269:21
     |
3269 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3285:23
     |
3285 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3286:21
     |
3286 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3302:23
     |
3302 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3303:21
     |
3303 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3321:23
     |
3321 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3322:21
     |
3322 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3338:23
     |
3338 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3339:21
     |
3339 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3356:23
     |
3356 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3357:21
     |
3357 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3373:23
     |
3373 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3374:21
     |
3374 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3389:48
     |
3389 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3390:66
     |
3390 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3406:23
     |
3406 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3407:21
     |
3407 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3425:23
     |
3425 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3426:21
     |
3426 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3442:23
     |
3442 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3443:21
     |
3443 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3459:23
     |
3459 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3460:21
     |
3460 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3478:23
     |
3478 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3479:21
     |
3479 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3497:23
     |
3497 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3498:21
     |
3498 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3516:23
     |
3516 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3517:21
     |
3517 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3533:23
     |
3533 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3534:21
     |
3534 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3550:23
     |
3550 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3551:21
     |
3551 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3567:23
     |
3567 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3568:21
     |
3568 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3586:23
     |
3586 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3587:21
     |
3587 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3603:23
     |
3603 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3604:21
     |
3604 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3620:23
     |
3620 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3621:21
     |
3621 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3637:23
     |
3637 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3638:21
     |
3638 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3654:23
     |
3654 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3655:21
     |
3655 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3671:23
     |
3671 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3672:21
     |
3672 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3690:23
     |
3690 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3691:21
     |
3691 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3709:23
     |
3709 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3710:21
     |
3710 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3729:23
     |
3729 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3730:21
     |
3730 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3748:23
     |
3748 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3749:21
     |
3749 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3765:23
     |
3765 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3766:21
     |
3766 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3783:23
     |
3783 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3784:21
     |
3784 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3800:23
     |
3800 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3801:21
     |
3801 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3816:48
     |
3816 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3817:66
     |
3817 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3833:23
     |
3833 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3834:21
     |
3834 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3852:23
     |
3852 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3853:21
     |
3853 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3869:23
     |
3869 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3870:21
     |
3870 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3886:23
     |
3886 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3887:21
     |
3887 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3903:23
     |
3903 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3904:21
     |
3904 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3920:23
     |
3920 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3921:21
     |
3921 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3937:23
     |
3937 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3938:21
     |
3938 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3954:23
     |
3954 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3955:21
     |
3955 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3971:23
     |
3971 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3972:21
     |
3972 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3988:23
     |
3988 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:3989:21
     |
3989 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4005:23
     |
4005 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4006:21
     |
4006 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4022:23
     |
4022 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4023:21
     |
4023 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4042:23
     |
4042 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4043:21
     |
4043 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4059:23
     |
4059 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4060:21
     |
4060 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4076:23
     |
4076 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4077:21
     |
4077 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4093:23
     |
4093 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4094:21
     |
4094 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4110:23
     |
4110 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4111:21
     |
4111 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4127:23
     |
4127 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4128:21
     |
4128 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4143:48
     |
4143 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4144:66
     |
4144 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4160:23
     |
4160 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4161:21
     |
4161 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4183:23
     |
4183 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4184:21
     |
4184 |         let __end = __sym6.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym6.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4205:23
     |
4205 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4206:21
     |
4206 |         let __end = __sym5.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym5.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4223:23
     |
4223 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4224:21
     |
4224 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4240:23
     |
4240 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4241:21
     |
4241 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4263:23
     |
4263 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4264:21
     |
4264 |         let __end = __sym6.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym6.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4283:23
     |
4283 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4284:21
     |
4284 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4300:23
     |
4300 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4301:21
     |
4301 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4317:23
     |
4317 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4318:21
     |
4318 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4333:48
     |
4333 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4334:66
     |
4334 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4350:23
     |
4350 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4351:21
     |
4351 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4367:23
     |
4367 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4368:21
     |
4368 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4385:23
     |
4385 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4386:21
     |
4386 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4403:23
     |
4403 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4404:21
     |
4404 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4420:23
     |
4420 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4421:21
     |
4421 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4437:23
     |
4437 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4438:21
     |
4438 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4454:23
     |
4454 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4455:21
     |
4455 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4473:23
     |
4473 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4474:21
     |
4474 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4490:23
     |
4490 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4491:21
     |
4491 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4507:23
     |
4507 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4508:21
     |
4508 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4524:23
     |
4524 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4525:21
     |
4525 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4541:23
     |
4541 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4542:21
     |
4542 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4559:23
     |
4559 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4560:21
     |
4560 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4576:23
     |
4576 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4577:21
     |
4577 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4593:23
     |
4593 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4594:21
     |
4594 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4610:23
     |
4610 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4611:21
     |
4611 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4627:23
     |
4627 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4628:21
     |
4628 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4644:23
     |
4644 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4645:21
     |
4645 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4661:23
     |
4661 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4662:21
     |
4662 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4680:23
     |
4680 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4681:21
     |
4681 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4699:23
     |
4699 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4700:21
     |
4700 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4716:23
     |
4716 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4717:21
     |
4717 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4733:23
     |
4733 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4734:21
     |
4734 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4750:23
     |
4750 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4751:21
     |
4751 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4767:23
     |
4767 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4768:21
     |
4768 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4784:23
     |
4784 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4785:21
     |
4785 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4801:23
     |
4801 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4802:21
     |
4802 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4817:48
     |
4817 |         let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
     |                                                ^^^^^^^^^^^ help: try removing the `clone` call: `s.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4818:66
     |
4818 |         let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
     |                                                                  ^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__start`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4834:23
     |
4834 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4835:21
     |
4835 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4852:23
     |
4852 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4853:21
     |
4853 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4869:23
     |
4869 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4870:21
     |
4870 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4886:23
     |
4886 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4887:21
     |
4887 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4903:23
     |
4903 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4904:21
     |
4904 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4920:23
     |
4920 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4921:21
     |
4921 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4937:23
     |
4937 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4938:21
     |
4938 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4954:23
     |
4954 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4955:21
     |
4955 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4971:23
     |
4971 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4972:21
     |
4972 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4988:23
     |
4988 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:4989:21
     |
4989 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5005:23
     |
5005 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5006:21
     |
5006 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5022:23
     |
5022 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5023:21
     |
5023 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5041:23
     |
5041 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5042:21
     |
5042 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5061:23
     |
5061 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5062:21
     |
5062 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5079:23
     |
5079 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5080:21
     |
5080 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5098:23
     |
5098 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5099:21
     |
5099 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5118:23
     |
5118 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5119:21
     |
5119 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5138:23
     |
5138 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5139:21
     |
5139 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5156:23
     |
5156 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5157:21
     |
5157 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5175:23
     |
5175 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5176:21
     |
5176 |         let __end = __sym2.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5192:23
     |
5192 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5193:21
     |
5193 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5210:23
     |
5210 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5211:21
     |
5211 |         let __end = __sym1.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5227:23
     |
5227 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5228:21
     |
5228 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5244:23
     |
5244 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5245:21
     |
5245 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5264:23
     |
5264 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5265:21
     |
5265 |         let __end = __sym3.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5281:23
     |
5281 |         let __start = __sym0.0.clone();
     |                       ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:5282:21
     |
5282 |         let __end = __sym0.2.clone();
     |                     ^^^^^^^^^^^^^^^^ help: try removing the `clone` call: `__sym0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6180:19
     |
6180 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = note: `#[warn(clippy::trivially_copy_pass_by_ref)]` on by default
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6181:18
     |
6181 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6214:19
     |
6214 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6215:18
     |
6215 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6231:19
     |
6231 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6232:18
     |
6232 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6240:19
     |
6240 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6241:18
     |
6241 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6274:19
     |
6274 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6275:18
     |
6275 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6301:19
     |
6301 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6302:18
     |
6302 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6318:19
     |
6318 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6319:18
     |
6319 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6327:19
     |
6327 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6328:18
     |
6328 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6370:19
     |
6370 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6371:18
     |
6371 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6444:20
     |
6444 |     let __start0 = __2.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6445:18
     |
6445 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6465:20
     |
6465 |     let __start0 = __1.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6466:18
     |
6466 |     let __end0 = __2.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6486:20
     |
6486 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6487:18
     |
6487 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6505:20
     |
6505 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6506:18
     |
6506 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6525:20
     |
6525 |     let __start0 = __0.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6526:18
     |
6526 |     let __end0 = __1.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6548:20
     |
6548 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6549:18
     |
6549 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6568:20
     |
6568 |     let __start0 = __0.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6569:18
     |
6569 |     let __end0 = __1.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6589:20
     |
6589 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6590:18
     |
6590 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6609:20
     |
6609 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6610:18
     |
6610 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6633:20
     |
6633 |     let __start0 = __4.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__4.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6634:18
     |
6634 |     let __end0 = __6.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__6.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6658:20
     |
6658 |     let __start0 = __3.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6659:18
     |
6659 |     let __end0 = __3.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6680:20
     |
6680 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6681:18
     |
6681 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6699:20
     |
6699 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6700:18
     |
6700 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6717:20
     |
6717 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6718:18
     |
6718 |     let __end0 = __0.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6736:20
     |
6736 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6737:18
     |
6737 |     let __end0 = __0.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6754:20
     |
6754 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6755:18
     |
6755 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6773:20
     |
6773 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6774:18
     |
6774 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6791:20
     |
6791 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6792:18
     |
6792 |     let __end0 = __0.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6810:20
     |
6810 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6811:18
     |
6811 |     let __end0 = __0.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6833:20
     |
6833 |     let __start0 = __3.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__3.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6834:18
     |
6834 |     let __end0 = __3.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__3.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6860:20
     |
6860 |     let __start0 = __2.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6861:18
     |
6861 |     let __end0 = __3.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__3.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6886:20
     |
6886 |     let __start0 = __2.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6887:18
     |
6887 |     let __end0 = __2.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6907:20
     |
6907 |     let __start0 = __1.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6908:18
     |
6908 |     let __end0 = __2.0.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__2.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6928:20
     |
6928 |     let __start0 = __1.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6929:18
     |
6929 |     let __end0 = __1.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__1.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6945:20
     |
6945 |     let __start0 = __0.2.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6946:18
     |
6946 |     let __end0 = __0.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6960:19
     |
6960 |     __lookbehind: &usize,
     |                   ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: this argument (8 byte) is passed by reference, but would be more efficient if passed by value (limit: 8 byte)
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6961:18
     |
6961 |     __lookahead: &usize,
     |                  ^^^^^^ help: consider passing by value instead: `usize`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trivially_copy_pass_by_ref

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6964:20
     |
6964 |     let __start0 = __lookbehind.clone();
     |                    ^^^^^^^^^^^^^^^^^^^^ help: try dereferencing it: `*__lookbehind`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6965:18
     |
6965 |     let __end0 = __lookahead.clone();
     |                  ^^^^^^^^^^^^^^^^^^^ help: try dereferencing it: `*__lookahead`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6981:20
     |
6981 |     let __start0 = __0.0.clone();
     |                    ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.0`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

warning: using `clone` on a `Copy` type
    --> /Users/alexander/Documents/University/Programming/rust/ptc/target/debug/build/ptc-ce27892519482e93/out/parser.rs:6982:18
     |
6982 |     let __end0 = __0.2.clone();
     |                  ^^^^^^^^^^^^^ help: try removing the `clone` call: `__0.2`
     |
     = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#clone_on_copy

    Finished dev [unoptimized + debuginfo] target(s) in 1.13s
