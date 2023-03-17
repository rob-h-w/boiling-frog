#[macro_export]
macro_rules! set_margins {
    ($element_builder:expr, $margin:expr) => {
        $element_builder
            .margin_top($margin)
            .margin_bottom($margin)
            .margin_start($margin)
            .margin_end($margin)
    };
}

#[macro_export]
macro_rules! make_value_units_string {
    ($source:expr) => {{
        let val = $source;
        // https://docs.gtk.org/Pango/pango_markup.html
        format!(
            "<span font_size='40000'>{:.0}{}</span>",
            val.value, val.units
        )
    }};
}
