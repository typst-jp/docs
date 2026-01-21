use typst::text::FontVariant;
use typst_kit::fonts::Fonts;

use crate::args::FontsCommand;

/// Execute a font listing command.
pub fn fonts(command: &FontsCommand) {
<<<<<<< HEAD
    let fonts = Fonts::searcher()
        .include_system_fonts(!command.font.ignore_system_fonts)
        .search_with(&command.font.font_paths);
=======
    let mut fonts = Fonts::searcher();
    fonts.include_system_fonts(!command.font.ignore_system_fonts);
    #[cfg(feature = "embed-fonts")]
    fonts.include_embedded_fonts(!command.font.ignore_embedded_fonts);
    let fonts = fonts.search_with(&command.font.font_paths);
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534

    for (name, infos) in fonts.book.families() {
        println!("{name}");
        if command.variants {
            for info in infos {
                let FontVariant { style, weight, stretch } = info.variant;
                println!("- Style: {style:?}, Weight: {weight:?}, Stretch: {stretch:?}");
            }
        }
    }
}
