use std::error::Error;

/// Usage:
///```no_run
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
