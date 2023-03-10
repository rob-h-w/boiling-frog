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
