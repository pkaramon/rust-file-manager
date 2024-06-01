use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct SortEntry {
    pub name: &'static str,
    pub func: fn(&mut Vec<PathBuf>) -> Result<bool>,
}

fn sort_by_name(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort();
    Ok(true)
}

fn sort_by_size(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort_by(|a, b| {
        let a_size = fs::metadata(a).unwrap().len();
        let b_size = fs::metadata(b).unwrap().len();
        b_size.cmp(&a_size)
    });
    Ok(true)
}

fn sort_by_modified_date(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort_by(|a, b| {
        let a_time = fs::metadata(a).unwrap().modified().unwrap();
        let b_time = fs::metadata(b).unwrap().modified().unwrap();
        b_time.cmp(&a_time)
    });
    Ok(true)
}

pub const SORT_ENTRIES: [SortEntry; 3] = [
    SortEntry {
        name: "Name",
        func: sort_by_name,
    },
    SortEntry {
        name: "Size",
        func: sort_by_size,
    },
    SortEntry {
        name: "Modified Date",
        func: sort_by_modified_date,
    },
];
