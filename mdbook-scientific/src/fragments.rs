use std::{str, usize, io::Write};
use std::path::Path;
use std::fs::{self, File};
use std::process::{Command, Stdio};

use sha2::{Digest, Sha256};

use crate::error::{Error, Result};

/// Convert input string to 24 character hash
pub fn hash(input: &str) -> String {
    let mut sh = Sha256::new();
    sh.input(input.as_bytes());
    let mut out = format!("{:x}", sh.result());
    out.truncate(24);
    out
}

/// Generate SVG file from latex file with given zoom
pub fn generate_svg_from_latex(path: &Path, zoom: f32) -> Result<()> {
    let dest_path = path.parent().unwrap();
    let file: &Path = path.file_name().unwrap().as_ref();

    // use latex to generate a dvi
    let dvi_path = path.with_extension("dvi");
    if !dvi_path.exists() {
        let latex_path = which::which("latex")
            .map_err(|err| Error::BinaryNotFound(err))?;

        let cmd = Command::new(latex_path)
            .current_dir(&dest_path)
            //.arg("--jobname").arg(&dvi_path)
            .arg(&file.with_extension("tex"))
            .output()
            .expect("Could not spawn latex");

        if !cmd.status.success() {
            let buf = String::from_utf8_lossy(&cmd.stdout);

            // latex prints error to the stdout, if this is empty, then something is fundamentally
            // wrong with the latex binary (for example shared library error). In this case just
            // exit the program
            if buf.is_empty() {
                let buf = String::from_utf8_lossy(&cmd.stderr);
                panic!("Latex exited with `{}`", buf);
            }

            let err = buf
                .split("\n")
                .filter(|x| {
                    (x.starts_with("! ") || x.starts_with("l.")) && !x.contains("Emergency stop")
                })
                .fold(("", "", usize::MAX), |mut err, elm| {
                    if elm.starts_with("! ") {
                        err.0 = elm;
                    } else if elm.starts_with("l.") {
                        let mut elms = elm[2..].splitn(2, " ").map(|x| x.trim());
                        if let Some(Ok(val)) = elms.next().map(|x| x.parse::<usize>()) {
                            err.2 = val;
                        }
                        if let Some(val) = elms.next() {
                            err.1 = val;
                        }
                    }

                    err
                });

            return Err(Error::InvalidMath(
                err.0.to_string(),
                err.1.to_string(),
                err.2,
            ));
        }
    }

    // convert the dvi to a svg file with the woff font format
    let svg_path = path.with_extension("svg");
    if !svg_path.exists() && dvi_path.exists() {
        let dvisvgm_path = which::which("dvisvgm")
            .map_err(|err| Error::BinaryNotFound(err))?;

        let cmd = Command::new(dvisvgm_path)
            .current_dir(&dest_path)
            .arg("-b")
            .arg("1")
            .arg("--font-format=woff")
            .arg(&format!("--zoom={}", zoom))
            .arg(&dvi_path)
            .output()
            .expect("Couldn't run svisvgm properly!");

        let buf = String::from_utf8_lossy(&cmd.stderr);
        if !cmd.status.success() || buf.contains("error:") {
            return Err(Error::InvalidDvisvgm(buf.to_string()));
        }
    }

    Ok(())
}

/// Generate latex file from gnuplot
///
/// This function generates a latex file with gnuplot `epslatex` backend and then source it into
/// the generate latex function
fn generate_latex_from_gnuplot(dest_path: &Path, content: &str, filename: &str) -> Result<()> {
    let gnuplot_path = which::which("gnuplot")
        .map_err(|err| Error::BinaryNotFound(err))?;

    let cmd = Command::new(gnuplot_path)
        .stdin(Stdio::piped())
        .current_dir(dest_path)
        .arg("-p")
        .spawn()
        .unwrap();
    //.expect("Could not spawn gnuplot");

    let mut stdin = cmd.stdin.unwrap();

    stdin
        .write_all(&format!("set output '{}.tex'\n", filename).as_bytes())
        .map_err(|err| Error::Io(err))?;
    stdin
        .write_all("set terminal epslatex color standalone\n".as_bytes())
        .map_err(|err| Error::Io(err))?;
    stdin
        .write_all(content.as_bytes())
        .map_err(|err| Error::Io(err))?;

    Ok(())
}

/// Parse an equation with the given zoom
pub fn parse_equation(
    dest_path: &Path,
    content: &str,
    zoom: f32,
) -> Result<String> {
    let name = hash(content);
    let path = dest_path.join(&name);

    // create a new tex file containing the equation
    if !path.with_extension("tex").exists() {
        let mut file = File::create(path.with_extension("tex")).map_err(|err| Error::Io(err))?;

        file.write_all("\\documentclass[20pt, preview]{standalone}\n\\usepackage{amsmath}\\usepackage{amsfonts}\n\\begin{document}\n$$\n".as_bytes())
            .map_err(|err| Error::Io(err))?;

        file.write_all(content.as_bytes())
            .map_err(|err| Error::Io(err))?;

        file.write_all("$$\n\\end{document}".as_bytes())
            .map_err(|err| Error::Io(err))?;
    }

    generate_svg_from_latex(&path, zoom)?;

    Ok(format!("{}.svg", name))
}

/// Parse a latex content and convert it to a SVG file
pub fn parse_latex(
    dest_path: &Path,
    content: &str,
) -> Result<String> {
    let name = hash(content);
    let path = dest_path.join(&name);

    // create a new tex file containing the equation
    if !path.with_extension("tex").exists() {
        let mut file = File::create(path.with_extension("tex")).map_err(|err| Error::Io(err))?;

        file.write_all(content.as_bytes())
            .map_err(|err| Error::Io(err))?;
    }

    generate_svg_from_latex(&path, 1.0)?;

    Ok(format!("{}.svg", name))
}

/// Parse a gnuplot file and generate a SVG file
pub fn parse_gnuplot(
    dest_path: &Path,
    content: &str,
) -> Result<String> {
    let name = hash(content);
    let path = dest_path.join(&name);

    if !path.with_extension("tex").exists() {
        //let name_plot = format!("{}_plot", name);
        generate_latex_from_gnuplot(dest_path, content, &name)?;
    }

    if !path.with_extension("svg").exists() {
        generate_svg_from_latex(&path, 1.0)?;
    }

    Ok(format!("{}.svg", name))
}

/// Parse gnuplot without using the latex backend
pub fn parse_gnuplot_only(
    dest_path: &Path,
    content: &str,
) -> Result<String> {
    let name = hash(content);
    let path = dest_path.join(&name);

    if !path.with_extension("svg").exists() {
        let gnuplot_path = which::which("gnuplot")
            .map_err(|err| Error::BinaryNotFound(err))?;
        let cmd = Command::new(gnuplot_path)
            .stdin(Stdio::piped())
            .current_dir(dest_path)
            .arg("-p")
            .spawn()
            .unwrap();
        //.expect("Could not spawn gnuplot");

        let mut stdin = cmd.stdin.unwrap();
        stdin
            .write_all(&format!("set output '{}.svg'\n", name).as_bytes())
            .map_err(|err| Error::Io(err))?;
        stdin
            .write_all("set terminal svg\n".as_bytes())
            .map_err(|err| Error::Io(err))?;
        stdin
            .write_all("set encoding utf8\n".as_bytes())
            .map_err(|err| Error::Io(err))?;
        stdin
            .write_all(content.as_bytes())
            .map_err(|err| Error::Io(err))?;
    }

    Ok(format!("{}.svg", name))
}

/// Generate html from BibTeX file using `bib2xhtml`
pub fn bib_to_html(source: &str, bib2xhtml: &str) -> Result<String> {
    let source = fs::canonicalize(source).unwrap();
    let bib2xhtml = Path::new(bib2xhtml);

    //./bib2xhtml.pl -s alpha -u -U ~/Documents/Bachelor_thesis/literature.bib
    let cmd = Command::new(bib2xhtml.join("./bib2xhtml.pl"))
        .current_dir(bib2xhtml)
        .args(&["-s", "alpha", "-u", "-U"])
        .arg(source)
        .output()
        .expect("Could not spawn bib2xhtml");

    let buf = String::from_utf8_lossy(&cmd.stdout);

    let err_str = String::from_utf8_lossy(&cmd.stderr);
    if err_str.contains("error messages)") {
        Err(Error::InvalidBibliography(err_str.to_string()))
    } else {
        let buf = buf.split("\n")
            .skip_while(|x| *x != "<dl class=\"bib2xhtml\">")
            .take_while(|x| *x != "</dl>")
            .map(|x| x.replace("<a name=\"", "<a id=\""))
            .collect();

        Ok(buf)
    }
}

/*pub fn parse_code(params: Vec<String>, content: String, url: String) -> Result<String> {
    let mut out: String = "".into();

    let mut found_first = false;
    for tag in params {
        let start_tag = format!("$ref:{}$", tag);
        let mut iter = content.split("\n");

        let mut line = 0;
        let mut found = false;
        while let Some(elm) = iter.next() {
            line += 1;
            if elm.contains(&start_tag) {
                found = true;
                break;
            }
        }

        if !found {
            return Err(Error::InvalidCode(format!("could not find the code section {}", tag)));
        } else if !found_first {
            out = format!("\n```rust, {}#L{}\n", url, line);
            found_first = true;
        }

        found = false;
        let mut prev_newline = false;
        let mut first = true;
        while let Some(elm) = iter.next() {
            if elm.contains("$endref$") {
                found = true;
                break;
            } else {
                if elm.trim().is_empty() {
                    if !first {
                        prev_newline = true;
                    }
                } else {
                    first = false;

                    if prev_newline {
                        out.push_str("\n");
                        prev_newline = false;
                    }
                    out.push_str(&elm);
                    out.push_str("\n");
                }
            }
        }

        if !found {
            return Err(Error::InvalidCode(format!("could not find the end tag for {}", tag)));
        }
    }


    out.push_str("```\n");

    Ok(out)
}*/
