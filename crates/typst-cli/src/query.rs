use comemo::Track;
<<<<<<< HEAD
use ecow::{eco_format, EcoString};
use serde::Serialize;
use typst::diag::{bail, HintedStrResult, StrResult, Warned};
use typst::foundations::{Content, IntoValue, LocatableSelector, Scope};
use typst::layout::PagedDocument;
use typst::syntax::Span;
use typst::World;
use typst_eval::{eval_string, EvalMode};

use crate::args::{QueryCommand, SerializationFormat};
=======
use ecow::{EcoString, eco_format};
use typst::World;
use typst::diag::{HintedStrResult, StrResult, Warned, bail};
use typst::engine::Sink;
use typst::foundations::{Content, IntoValue, LocatableSelector, Scope};
use typst::introspection::Introspector;
use typst::layout::PagedDocument;
use typst::syntax::{Span, SyntaxMode};
use typst_eval::eval_string;
use typst_html::HtmlDocument;

use crate::args::{QueryCommand, Target};
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
use crate::compile::print_diagnostics;
use crate::set_failed;
use crate::world::SystemWorld;

/// Execute a query command.
pub fn query(command: &QueryCommand) -> HintedStrResult<()> {
    let mut world = SystemWorld::new(&command.input, &command.world, &command.process)?;

    // Reset everything and ensure that the main file is present.
    world.reset();
    world.source(world.main()).map_err(|err| err.to_string())?;

<<<<<<< HEAD
    let Warned { output, warnings } = typst::compile(&world);

    match output {
        // Retrieve and print query results.
        Ok(document) => {
            let data = retrieve(&world, command, &document)?;
=======
    let Warned { output, warnings } = match command.target {
        Target::Paged => typst::compile::<PagedDocument>(&world)
            .map(|output| output.map(|document| document.introspector)),
        Target::Html => typst::compile::<HtmlDocument>(&world)
            .map(|output| output.map(|document| document.introspector)),
    };

    match output {
        // Retrieve and print query results.
        Ok(introspector) => {
            let data = retrieve(&world, command, &introspector)?;
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
            let serialized = format(data, command)?;
            println!("{serialized}");
            print_diagnostics(&world, &[], &warnings, command.process.diagnostic_format)
                .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }

        // Print diagnostics.
        Err(errors) => {
            set_failed();
            print_diagnostics(
                &world,
                &errors,
                &warnings,
                command.process.diagnostic_format,
            )
            .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }
    }

    Ok(())
}

/// Retrieve the matches for the selector.
fn retrieve(
    world: &dyn World,
    command: &QueryCommand,
<<<<<<< HEAD
    document: &PagedDocument,
=======
    introspector: &Introspector,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
) -> HintedStrResult<Vec<Content>> {
    let selector = eval_string(
        &typst::ROUTINES,
        world.track(),
<<<<<<< HEAD
        &command.selector,
        Span::detached(),
        EvalMode::Code,
=======
        // TODO: propagate warnings
        Sink::new().track_mut(),
        &command.selector,
        Span::detached(),
        SyntaxMode::Code,
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
        Scope::default(),
    )
    .map_err(|errors| {
        let mut message = EcoString::from("failed to evaluate selector");
        for (i, error) in errors.into_iter().enumerate() {
            message.push_str(if i == 0 { ": " } else { ", " });
            message.push_str(&error.message);
        }
        message
    })?
    .cast::<LocatableSelector>()?;

<<<<<<< HEAD
    Ok(document
        .introspector
        .query(&selector.0)
        .into_iter()
        .collect::<Vec<_>>())
=======
    Ok(introspector.query(&selector.0).into_iter().collect::<Vec<_>>())
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
}

/// Format the query result in the output format.
fn format(elements: Vec<Content>, command: &QueryCommand) -> StrResult<String> {
    if command.one && elements.len() != 1 {
        bail!("expected exactly one element, found {}", elements.len());
    }

    let mapped: Vec<_> = elements
        .into_iter()
        .filter_map(|c| match &command.field {
            Some(field) => c.get_by_name(field).ok(),
            _ => Some(c.into_value()),
        })
        .collect();

    if command.one {
        let Some(value) = mapped.first() else {
            bail!("no such field found for element");
        };
<<<<<<< HEAD
        serialize(value, command.format, command.pretty)
    } else {
        serialize(&mapped, command.format, command.pretty)
    }
}

/// Serialize data to the output format.
fn serialize(
    data: &impl Serialize,
    format: SerializationFormat,
    pretty: bool,
) -> StrResult<String> {
    match format {
        SerializationFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).map_err(|e| eco_format!("{e}"))
            } else {
                serde_json::to_string(data).map_err(|e| eco_format!("{e}"))
            }
        }
        SerializationFormat::Yaml => {
            serde_yaml::to_string(data).map_err(|e| eco_format!("{e}"))
        }
=======
        crate::serialize(value, command.format, command.pretty)
    } else {
        crate::serialize(&mapped, command.format, command.pretty)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
    }
}
