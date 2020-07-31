use std::fs;
use std::collections::HashMap;
use std::path::Path;

use crate::fragments;
use crate::error::{Error, Result};

const BLOCK_DELIM: &str = "$$";
const INLINE_BLOCK_DELIM: &str = "$";

pub fn replace_blocks(fragment_path: &Path, asset_path: &Path, source: &str, head_num: &str, used_fragments: &mut Vec<String>, references: &mut HashMap<String, String>) -> Result<String> {
    let mut content = String::new();
    let mut start_line: Option<String> = None;
    let mut figures_counter = 0;
    let mut equations_counter = 0;

    let mut add_object = move |file: String, refer: &str, title: Option<&str>| -> String {
        used_fragments.push(file.clone());

        if let Some(title) = title {
            figures_counter += 1;
            references.insert(refer.to_string(), format!("Figure {}{}", head_num, figures_counter));

            format!("<figure id=\"{}\" class=\"figure\"><object data=\"assets/{}\" type=\"image/svg+xml\"/></object><figcaption>Figure {}{} {}</figcaption></figure>", 
                refer, file, head_num, figures_counter, title)
        } else if !refer.is_empty() {
            equations_counter += 1;
            references.insert(refer.to_string(), format!("{}{}", head_num, equations_counter));
            format!("<div id=\"{}\" class=\"equation\"><div class=\"equation_inner\"><object data=\"assets/{}\" type=\"image/svg+xml\"></object></div><span>({}{})</span></div>\n", refer, file, head_num, equations_counter)
        } else {
            format!("<div class=\"equation\"><div class=\"equation_inner\"><object data=\"assets/{}\" type=\"image/svg+xml\"></object></div></div>\n", file)
        }
    };

    source.split("\n")
    .filter_map(|line| {
        let line = line.trim();

        if !line.starts_with(BLOCK_DELIM) {
            if start_line.is_some() {
                content.push_str(line);
                content.push('\n');
                return None;
            } else {
                return Some(Ok(line.into()));
            }
        } else if line.ends_with(BLOCK_DELIM) && line.len() > 3{
            // line starts and end with BLOCK_DELIM, set content to empty
            start_line = Some(line.to_string());
            content = "".into();
        }

        if let Some(param) = start_line.take() {
            let elms = param.splitn(3, ",")
                .map(|x| x.trim())
                .map(|x| x.replace(BLOCK_DELIM, ""))
                .collect::<Vec<_>>();

            let elms = elms.iter().map(|x| x.as_str()).collect::<Vec<_>>();

            // if there is no content, try to load it from file
            if content.is_empty() {
                let path = asset_path.join(elms[1]).with_extension("tex");
                if path.exists() {
                    content = fs::read_to_string(path).unwrap();
                } else {
                    eprintln!("Block empty, but file `{}` was not found!", elms[1]);
                    return None;
                }
            }

            let generated_out = match &elms[..] {
                ["latex", refer, title] => {
                    fragments::parse_latex(fragment_path, &content)
                        .map(|file| add_object(file, refer, Some(title)))
                },
                ["gnuplot", refer, title] => {
                    fragments::parse_gnuplot(fragment_path, &content)
                        .map(|file| add_object(file, refer, Some(title)))
                },
                ["gnuplotonly", refer, title] => {
                    fragments::parse_gnuplot_only(fragment_path, &content)
                        .map(|file| add_object(file, refer, Some(title)))
                },

                ["equation", refer] | ["equ", refer] => {
                    fragments::parse_equation(fragment_path, &content, 1.6)
                        .map(|file| add_object(file, refer, None))
                }

                ["equation"] | ["equ"] | _ => {
                    fragments::parse_equation(fragment_path, &content, 1.6)
                        .map(|file| add_object(file, "", None))
                },
            };
            content = "".into();

            Some(generated_out)
        } else {
            start_line = Some(line.to_string());
            None
        }
    })
    .collect::<Result<Vec<_>>>()
    .map(|x| x.join("\n"))
}

pub fn replace_inline_blocks(fragment_path: &Path, source: &str, references: &HashMap<String, String>, used_fragments: &mut Vec<String>) -> Result<String> {
    source.split("\n").enumerate().map(|(line_num, line)| {
        if line.matches(INLINE_BLOCK_DELIM).count() % 2 != 0 {
            return Err(Error::UnevenNumberDollar);
        }

        line.split(INLINE_BLOCK_DELIM).enumerate().map(|(i, elm)| {
            if i % 2 == 0 {
                return Ok(elm.to_string());
            }

            let generated_out = if elm.starts_with("ref:") {
                let elms = elm.split(":").skip(1).collect::<Vec<&str>>();

                // we expect a type and reference name
                if elms.len() != 2 {
                    return Ok(elm.to_string());
                }

                match &elms[..] {
                    ["fig", refere] => {
                        references.get::<str>(refere)
                            .ok_or(Error::InvalidReference(format!("could not find reference to `{}` in line {}", elms[1], line_num)))
                            .map(|x| format!("<a class=\"fig_ref\" href='#{}'>{}</a>", elms[1], x))
                    },
                    ["bib", refere] => {
                        references.get::<str>(refere)
                            .ok_or(Error::InvalidReference(format!("could not find reference to `{}` in line {}", elms[1], line_num)))
                            .map(|x| format!("<a class=\"bib_ref\" href='bibliography.html#{}'>{}</a>", elms[1], x))
                    },
                    ["equ", refere] => {
                        references.get::<str>(refere)
                            .ok_or(Error::InvalidReference(format!("could not find reference to `{}` in line {}", elms[1], line_num)))
                            .map(|x| format!("<a class=\"equ_ref\" href='#{}'>Eq. ({})</a>", elms[1], x))
                    },
                    [kind, _] => Err(Error::InvalidReference(format!("unknown reference type of `{}` in line {}", kind, line_num))),
                    _ =>         Err(Error::InvalidReference(format!("reference has wrong number of arguments `{}` in line {}", elms.len(), line_num)))

                }
            } else {
                fragments::parse_equation(fragment_path, elm, 1.3)
                    .map(|filename| {
                        let res = format!("<object class=\"equation_inline\" data=\"assets/{}\" type=\"image/svg+xml\"></object>", filename);
                        used_fragments.push(filename);

                        res
                    })
            };

            generated_out
        })
        .collect::<Result<Vec<String>>>()
        .map(|x| x.join("\n"))
    })
    .collect::<Result<Vec<_>>>()
    .map(|x| x.join("\n"))
}
