use std::error::Error;

/// Usage:
///```
/// # use utils::string_util_functions::get_tuple_list_from_string;
/// let s = "a:1,b:2,c:3";
/// assert_eq!(
///     get_tuple_list_from_string(s).filter_map(|v| v.ok()).collect::<Vec<_>>(),
///     vec![("a", "1"), ("b", "2"), ("c", "3")]
/// )
///```
pub fn get_tuple_list_from_string(
    string: &str,
) -> impl Iterator<Item = Result<(&str, &str), Box<dyn Error>>> {
    string.split(',').map(|s| {
        s.split_once(':').ok_or(
            format!("Data incorrectly formatted. Needs to be formatted as such: \"a:1,b:2,dtd:345...\". Current failing format: {:#?}", s)
            .into(),
        )
    })
}

/// Usage:
///```
/// # use utils::string_util_functions::get_list_of_tuple_lists_from_string;
/// let s = "(x:1,y:1),(x:2,y:2),(x:3,y:3),(x:4,y:4)";
/// assert_eq!(
///     get_list_of_tuple_lists_from_string(s)
///         .map(|v| v.filter_map(|v| v.ok()).collect::<Vec<_>>())
///         .collect::<Vec<_>>(),
///     vec![
///         vec![("x", "1"), ("y", "1")],
///         vec![("x", "2"), ("y", "2")],
///         vec![("x", "3"), ("y", "3")],
///         vec![("x", "4"), ("y", "4")]
///     ]
/// )
///```

pub fn get_list_of_tuple_lists_from_string(
    string: &str,
) -> impl Iterator<Item = impl Iterator<Item = Result<(&str, &str), Box<dyn Error>>>> {
    string.split("),").map(|s| {
        let mut s = s.trim();
        if s.starts_with('(') {
            s = &s[1..s.len()];
        }
        if s.chars().nth_back(0) == Some(',') {
            s = &s[0..(s.len() - 1)];
        }
        if s.chars().nth_back(0) == Some(')') {
            s = &s[0..(s.len() - 1)];
        }
        get_tuple_list_from_string(s)
    })
}
