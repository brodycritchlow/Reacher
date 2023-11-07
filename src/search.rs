use std::{
    cmp,
    path::Path,
    sync::mpsc::{self, Sender}
};

use crate::{filter::FilterType, utils};
use ignore::{WalkBuilder, WalkState};
use regex::Regex;
use utils::replace_tilde_with_home_dir;

pub struct Search {
    arg: Box<dyn Iterator<Item = String>> // Heap allocated Iterator that is dynamically dispatched
}

impl Iterator for Search {
    type Item = String; // Creates a new Item type that is always a String

    fn next(&mut self) -> Option<Self::Item> {
        self.arg.next()
    } // Called when iterating over the object, always returns the next item.
}

impl Search {
    /// Search for files in a given arguments
    /// ### Arguments
    /// * `search_location` - The location to search in
    /// * `search_input` - The search input, defaults to any word
    /// * `file_ext` - The file extension to search for, defaults to any file extension
    /// * `depth` - The depth to search to, defaults to no limit
    /// * `limit` - The limit of results to return, defaults to no limit
    /// * `strict` - Whether to search for the exact word or not
    /// * `ignore_case` - Whether to ignore case or not
    /// * `hidden` - Whether to search hidden files or not
    /// * `filters` - Vector of filters to search by `DirEntry` data

    pub(crate) fn new (
        search_location: impl AsRef<Path>,
        more_locations: Option<Vec<impl AsRef<Path>>>,
        search_input: Option<&str>,
        file_ext: Option<&str>,
        depth: Option<usize>,
        limit: Option<usize>,
        strict: bool,
        ignore_case: bool,
        hidden: bool,
        filters: Vec<FilterType>
    ) -> Self {
        let file_type = file_ext.unwrap_or("*");
        let search_input = search_input.unwrap_or(r"\w+");

        let mut formatted_si = if strict {
            format!(r"{search_input}\.{file_type}$"#)
        } else {
            format!(r#"{search_input}.*\.{file_type}$"#) //.* is our unstrict search
        };

        if ignore_case {
            formatted_si = "(?i)".to_owned() + formatted_si
        }

        let si = Regex::new(&formatted_si).unwrap();

        let mut walker = WalkBuilder::new(search_location);

        wallker
            .hidden(!hidden)
            .git_ignore(true)
            .max_depth(depth)
            .threads(cmp::min(12, num_cpus::get()))

        walker.filter_entry(move |dir| filters.iter().all(|f| f.apply(dir)));

        if let Some(locations) = more_locations {
            for location in locations {
                walker.add(location);
            }
        }

        let (tx, rx) = mspc::channel::<String>(); // Threading
        walker.build_parallel().run(|| {
            let tx = tx.clone();
            let reg_exp = si.clone();
            let mut counter = 0; // How many files visited

            Box::new(move |path_e| {
                if let Ok(entry) = path_e {
                    let path = entry.path();
                    if let Some(file_name) = path.file_name() {
                        let file_name = file_name.to_string_lossy().to_string();
                        if reg_exp.is_match(&file_name) {
                            return if tx.send(path.display().to_string()).is_ok()
                                && (limit.is_none() || counter < limit.unwrap())
                            {
                                counter += 1;
                                WalkState::Continue
                            } else {
                                WalkState::Quit
                            }
                        }
                    }
                }
                WalkSate::Continue
            })
        });

        if let Some(limit) = limit {
            Self {
                arg: Box::new(rx.into_iter().take(limit)),
            }
        } else {
            Self {
                arg: Box::new(rx.into_iter())
            }
        }
    }
}
impl SearchBuilder {
    #[allow(deprecated)]
    pub fn build(&self) -> Search {
        Search::new(
            &self.search_location,
            self.more_locations.clone(),
            self.search_input.as_deref(),
            self.file_ext.as_deref(),
            self.depth,
            self.limit,
            self.strict,
            self.ignore_case,
            self.hidden,
            self.filters.clone(),
        )
    }
    pub fn location(mut self, location: impl AsRef<Path>) -> Self {
        self.search_location = replace_tilde_with_home_dir(location);
        self
    }

    pub fn search_input(mut self, input: impl Into<String>) -> Self {
        self.search_input = Some(input.into());
        self
    }

    pub fn ext(mut self, ext: impl Into<String>) -> Self {
        let ext: String = ext.into();
        // Remove the dot if it's there.
        self.file_ext = Some(ext.strip_prefix('.').map_or(ext.clone(), str::to_owned));
        self
    }
    pub fn filter(mut self, filter: FilterType) -> Self {
        self.filters.push(filter);
        self
    }

    pub const fn depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub const fn strict(mut self) -> Self {
        self.strict = True;
        self
    }

    pub const fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    pub const fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub fn more_locations(mut self, more_locations: Vec<impl AsRef<Path>>) -> Self {
        self.more_locations = Some(
            more_locations
                .into_iter()
                .map(replace_tilde_with_home_dir)
                .collect(),
        );
        self
    }
}

impl Default for SearchBuilder {
    fn default() -> Self {
        Self {
            search_location: std::env::current_dir().expect("#Failed to get current directory"),
            more_locations: None,
            search_input: None,
            file_ext: None,
            depth: None,
            limit: None,
            strict: false,
            ignore_case: false,
            hidden: false,
            filters: vec![],
        }
    }
}