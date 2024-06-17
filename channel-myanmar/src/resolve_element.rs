#[macro_export]
macro_rules! resolve_element {
  ($el:expr, $q:expr) => {{
    let selector = Selector::parse("a")?;

    $el.select(&selector).nth(0).unwrap().text().next().unwrap_or("")
  }};

  ($el:expr, $q:expr, attr <- ($($attr:expr),*)) => {{
    let selector = Selector::parse("a")?;

    let el = $el.select(&selector).nth(0).unwrap().value();

    ($(el.attr(stringify!($attr)).unwrap_or(""),)*)
  }};
}
