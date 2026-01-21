//! Typst's code interpreter.

pub(crate) mod ops;

mod access;
mod binding;
mod call;
mod code;
mod flow;
mod import;
mod markup;
mod math;
mod methods;
mod rules;
mod vm;

<<<<<<< HEAD
pub use self::call::{eval_closure, CapturesVisitor};
pub use self::flow::FlowEvent;
pub use self::import::import;
pub use self::vm::Vm;
pub use typst_library::routines::EvalMode;
=======
pub use self::call::{CapturesVisitor, eval_closure};
pub use self::flow::FlowEvent;
pub use self::import::import;
pub use self::vm::Vm;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

use self::access::*;
use self::binding::*;
use self::methods::*;

use comemo::{Track, Tracked, TrackedMut};
<<<<<<< HEAD
use typst_library::diag::{bail, SourceResult};
=======
use typst_library::World;
use typst_library::diag::{SourceResult, bail};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use typst_library::engine::{Engine, Route, Sink, Traced};
use typst_library::foundations::{Context, Module, NativeElement, Scope, Scopes, Value};
use typst_library::introspection::Introspector;
use typst_library::math::EquationElem;
use typst_library::routines::Routines;
<<<<<<< HEAD
use typst_library::World;
use typst_syntax::{ast, parse, parse_code, parse_math, Source, Span};
=======
use typst_syntax::{Source, Span, SyntaxMode, ast, parse, parse_code, parse_math};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

/// Evaluate a source file and return the resulting module.
#[comemo::memoize]
#[typst_macros::time(name = "eval", span = source.root().span())]
pub fn eval(
    routines: &Routines,
    world: Tracked<dyn World + '_>,
    traced: Tracked<Traced>,
    sink: TrackedMut<Sink>,
    route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    // Prevent cyclic evaluation.
    let id = source.id();
    if route.contains(id) {
        panic!("Tried to cyclicly evaluate {:?}", id.vpath());
    }

    // Prepare the engine.
    let introspector = Introspector::default();
    let engine = Engine {
        routines,
        world,
        introspector: introspector.track(),
        traced,
        sink,
        route: Route::extend(route).with_id(id),
    };

    // Prepare VM.
    let context = Context::none();
    let scopes = Scopes::new(Some(world.library()));
    let root = source.root();
    let mut vm = Vm::new(engine, context.track(), scopes, root.span());

    // Check for well-formedness unless we are in trace mode.
    let errors = root.errors();
    if !errors.is_empty() && vm.inspected.is_none() {
        return Err(errors.into_iter().map(Into::into).collect());
    }

    // Evaluate the module.
    let markup = root.cast::<ast::Markup>().unwrap();
    let output = markup.eval(&mut vm)?;

    // Handle control flow.
    if let Some(flow) = vm.flow {
        bail!(flow.forbidden());
    }

    // Assemble the module.
    let name = id
        .vpath()
        .as_rootless_path()
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();

    Ok(Module::new(name, vm.scopes.top).with_content(output).with_file_id(id))
}

/// Evaluate a string as code and return the resulting value.
///
/// Everything in the output is associated with the given `span`.
#[comemo::memoize]
pub fn eval_string(
    routines: &Routines,
    world: Tracked<dyn World + '_>,
<<<<<<< HEAD
    string: &str,
    span: Span,
    mode: EvalMode,
    scope: Scope,
) -> SourceResult<Value> {
    let mut root = match mode {
        EvalMode::Code => parse_code(string),
        EvalMode::Markup => parse(string),
        EvalMode::Math => parse_math(string),
=======
    sink: TrackedMut<Sink>,
    string: &str,
    span: Span,
    mode: SyntaxMode,
    scope: Scope,
) -> SourceResult<Value> {
    let mut root = match mode {
        SyntaxMode::Code => parse_code(string),
        SyntaxMode::Markup => parse(string),
        SyntaxMode::Math => parse_math(string),
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    };

    root.synthesize(span);

    // Check for well-formedness.
    let errors = root.errors();
    if !errors.is_empty() {
        return Err(errors.into_iter().map(Into::into).collect());
    }

    // Prepare the engine.
<<<<<<< HEAD
    let mut sink = Sink::new();
=======
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    let introspector = Introspector::default();
    let traced = Traced::default();
    let engine = Engine {
        routines,
        world,
        introspector: introspector.track(),
        traced: traced.track(),
<<<<<<< HEAD
        sink: sink.track_mut(),
=======
        sink,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        route: Route::default(),
    };

    // Prepare VM.
    let context = Context::none();
    let scopes = Scopes::new(Some(world.library()));
    let mut vm = Vm::new(engine, context.track(), scopes, root.span());
    vm.scopes.scopes.push(scope);

    // Evaluate the code.
    let output = match mode {
<<<<<<< HEAD
        EvalMode::Code => root.cast::<ast::Code>().unwrap().eval(&mut vm)?,
        EvalMode::Markup => {
            Value::Content(root.cast::<ast::Markup>().unwrap().eval(&mut vm)?)
        }
        EvalMode::Math => Value::Content(
=======
        SyntaxMode::Code => root.cast::<ast::Code>().unwrap().eval(&mut vm)?,
        SyntaxMode::Markup => {
            Value::Content(root.cast::<ast::Markup>().unwrap().eval(&mut vm)?)
        }
        SyntaxMode::Math => Value::Content(
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            EquationElem::new(root.cast::<ast::Math>().unwrap().eval(&mut vm)?)
                .with_block(false)
                .pack()
                .spanned(span),
        ),
    };

    // Handle control flow.
    if let Some(flow) = vm.flow {
        bail!(flow.forbidden());
    }

    Ok(output)
}

/// Evaluate an expression.
pub trait Eval {
    /// The output of evaluating the expression.
    type Output;

    /// Evaluate the expression to the output value.
    fn eval(self, vm: &mut Vm) -> SourceResult<Self::Output>;
}
