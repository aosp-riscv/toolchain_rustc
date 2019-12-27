<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use clap::{crate_version};
=======
use clap::crate_version;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

use std::env;
use std::path::{Path, PathBuf};

use clap::{App, AppSettings, ArgMatches, SubCommand};

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use mdbook::MDBook;
use mdbook::errors::{Result as Result3};
=======
use mdbook::errors::Result as Result3;
use mdbook::MDBook;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

#[cfg(feature = "linkcheck")]
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
use mdbook::renderer::RenderContext;
#[cfg(feature = "linkcheck")]
use mdbook_linkcheck::{self, errors::BrokenLinks};
use failure::Error;
=======
use failure::Error;
#[cfg(feature = "linkcheck")]
use mdbook::renderer::RenderContext;
#[cfg(feature = "linkcheck")]
use mdbook_linkcheck::{self, errors::BrokenLinks};
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

fn main() {
    let d_message = "-d, --dest-dir=[dest-dir]
'The output directory for your book{n}(Defaults to ./book when omitted)'";
    let dir_message = "[dir]
'A directory for your book{n}(Defaults to Current Directory when omitted)'";

    let matches = App::new("rustbook")
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
                    .about("Build a book with mdBook")
                    .author("Steve Klabnik <steve@steveklabnik.com>")
                    .version(&*format!("v{}", crate_version!()))
                    .setting(AppSettings::SubcommandRequired)
                    .subcommand(SubCommand::with_name("build")
                        .about("Build the book from the markdown files")
                        .arg_from_usage(d_message)
                        .arg_from_usage(dir_message))
                    .subcommand(SubCommand::with_name("linkcheck")
                        .about("Run linkcheck with mdBook 3")
                        .arg_from_usage(dir_message))
                    .get_matches();
=======
        .about("Build a book with mdBook")
        .author("Steve Klabnik <steve@steveklabnik.com>")
        .version(&*format!("v{}", crate_version!()))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the book from the markdown files")
                .arg_from_usage(d_message)
                .arg_from_usage(dir_message),
        )
        .subcommand(
            SubCommand::with_name("linkcheck")
                .about("Run linkcheck with mdBook 3")
                .arg_from_usage(dir_message),
        )
        .get_matches();
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

    // Check which subcomamnd the user ran...
    match matches.subcommand() {
        ("build", Some(sub_matches)) => {
            if let Err(e) = build(sub_matches) {
                eprintln!("Error: {}", e);

                for cause in e.iter().skip(1) {
                    eprintln!("\tCaused By: {}", cause);
                }

                ::std::process::exit(101);
            }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        },
        ("linkcheck", Some(sub_matches)) => {
            if let Err(err) = linkcheck(sub_matches) {
                eprintln!("Error: {}", err);

                #[cfg(feature = "linkcheck")]
                {
                    if let Ok(broken_links) = err.downcast::<BrokenLinks>() {
                        for cause in broken_links.links().iter() {
                            eprintln!("\tCaused By: {}", cause);
                        }
=======
        }
        ("linkcheck", Some(sub_matches)) => {
            #[cfg(feature = "linkcheck")]
            {
                if let Err(err) = linkcheck(sub_matches) {
                    eprintln!("Error: {}", err);

                    // HACK: ignore timeouts
                    let actually_broken = err
                        .downcast::<BrokenLinks>()
                        .map(|broken_links| {
                            broken_links
                                .links()
                                .iter()
                                .inspect(|cause| eprintln!("\tCaused By: {}", cause))
                                .fold(false, |already_broken, cause| {
                                    already_broken || !format!("{}", cause).contains("timed out")
                                })
                        })
                        .unwrap_or(false);

                    if actually_broken {
                        std::process::exit(101);
                    } else {
                        std::process::exit(0);
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                    }
                }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
            }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
                ::std::process::exit(101);
            }
        },
=======
            #[cfg(not(feature = "linkcheck"))]
            {
                // This avoids the `unused_binding` lint.
                println!(
                    "mdbook-linkcheck is disabled, but arguments were passed: {:?}",
                    sub_matches
                );
            }
        }
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        (_, _) => unreachable!(),
    };
}

#[cfg(feature = "linkcheck")]
pub fn linkcheck(args: &ArgMatches<'_>) -> Result<(), Error> {
    let book_dir = get_book_dir(args);
    let book = MDBook::load(&book_dir).unwrap();
    let cfg = book.config;
    let render_ctx = RenderContext::new(&book_dir, book.book, cfg, &book_dir);

    mdbook_linkcheck::check_links(&render_ctx)
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
}

#[cfg(not(feature = "linkcheck"))]
pub fn linkcheck(_args: &ArgMatches<'_>) -> Result<(), Error> {
    println!("mdbook-linkcheck is disabled.");
    Ok(())
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

// Build command implementation
pub fn build(args: &ArgMatches<'_>) -> Result3<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    // Set this to allow us to catch bugs in advance.
    book.config.build.create_missing = false;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = PathBuf::from(dest_dir);
    }

    book.build()?;

    Ok(())
}

fn get_book_dir(args: &ArgMatches<'_>) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().unwrap()
    }
}
