mod error;
mod fragments;
mod preprocess;

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::path::PathBuf;

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use nom_bibtex::*;

use preprocess::{replace_blocks, replace_inline_blocks};

pub struct Scientific;

impl Scientific {
    pub fn new() -> Scientific {
        Scientific
    }
}

impl Preprocessor for Scientific {
    fn name(&self) -> &str {
        "scientific"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
            let fragment_path = cfg
                .get("fragment_path")
                .map(|x| x.as_str().unwrap())
                .unwrap_or("fragments/");
            let fragment_path = Path::new(fragment_path);

            if !fragment_path.exists() {
                fs::create_dir(&fragment_path)?;
            }
            let fragment_path = fragment_path.canonicalize().unwrap();

            //
            // track which fragments we use to copy them into the assets folder
            let mut used_fragments = Vec::new();
            // track which references are created
            let mut references = HashMap::new();
            // if there occurs an error skip everything and return the error
            let mut error = None;

            // load all references in the bibliography and export to html
            if let (Some(bib), Some(bib2xhtml)) = (cfg.get("bibliography"), cfg.get("bib2xhtml")) {
                let bib = bib.as_str().unwrap();
                let bib2xhtml = bib2xhtml.as_str().unwrap();

                if !Path::new(bib).exists() {
                    return Err(format!("bibliography {:?} not found!", bib).into());
                }

                // read entries in bibtex file
                let bibtex = fs::read_to_string(bib).unwrap();
                let bibtex = Bibtex::parse(&bibtex).unwrap();
                for (i, entry) in bibtex.bibliographies().into_iter().enumerate() {
                    references.insert(entry.citation_key().to_string(), format!("[{}]", i+1));
                }
                //
                // create bibliography
                let content = match fragments::bib_to_html(&bib, &bib2xhtml) {
                    Ok(x) => x,
                    Err(err) => return Err(format!("{:?}", err).into())
                };

                // add final chapter for bibliography
                let bib_chapter = Chapter::new("Bibliography", format!("# Bibliography\n{}", content), PathBuf::from("bibliography.md"), Vec::new());
                book.push_item(bib_chapter);
            }

            // assets path
            let asset_path = cfg.get("assets").map(|x| x.as_str().unwrap()).unwrap_or("src/");
            let asset_path = ctx.root.join(asset_path);

            // process blocks like `$$ .. $$`
            book.for_each_mut(|item| {
                if error.is_some() {
                    return;
                }

                if let BookItem::Chapter(ref mut ch) = item {
                    let head_number = ch.number.as_ref().map(|x| format!("{}", x)).unwrap_or("".into());

                    match replace_blocks(&fragment_path, &asset_path, &ch.content, &head_number, &mut used_fragments, &mut references) {
                        Ok(x) => ch.content = x,
                        Err(err) => error = Some(format!("Error in chapter {} {:?}", head_number, err))
                    }
                }
            });

            // process inline blocks like `$ .. $`
            book.for_each_mut(|item| {
                if error.is_some() {
                    return;
                }

                if let BookItem::Chapter(ref mut ch) = item {
                    let head_number = ch.number.as_ref().map(|x| format!("{}", x)).unwrap_or("".into());

                    match replace_inline_blocks(&fragment_path, &ch.content, &references, &mut used_fragments) {
                        Ok(x) => ch.content = x,
                        Err(err) => error = Some(format!("Error in chapter {}: {:?}", head_number, err))
                    }
                }
            });

            if let Some(err) = error {
                return Err(err.into());
            }

            // the output path is `src/assets`, which get copied to the output directory
            let dest = ctx.root.join("src").join("assets");
            if !dest.exists() {
                fs::create_dir_all(&dest).unwrap();
            }

            // copy all fragments
            for fragment in used_fragments {
                fs::copy(fragment_path.join(&fragment), dest.join(&fragment)).unwrap();
            }

            Ok(book)
        } else {
            Err("Key section not found!".into())
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

