use nih_plug_iced::Font;

pub const EB_GARAMOND_REGULAR: Font = Font::External {
    name: "EB Garamond Regular",
    bytes: include_bytes!("../../assets/fonts/EBGaramond-Regular.ttf"),
};
pub const EB_GARAMOND_BOLD: Font = Font::External {
    name: "EB Garamond Bold",
    bytes: include_bytes!("../../assets/fonts/EBGaramond-Bold.ttf"),
};
pub const EB_GARAMOND_ITALIC: Font = Font::External {
    name: "EB Garamond Italic",
    bytes: include_bytes!("../../assets/fonts/EBGaramond-Italic.ttf"),
};
pub const EB_GARAMOND_MEDIUM: Font = Font::External {
    name: "EB Garamond Medium",
    bytes: include_bytes!("../../assets/fonts/EBGaramond-Medium.ttf"),
};

pub const ROBOTO_MONO_REGULAR: Font = Font::External {
    name: "Roboto Mono Regular",
    bytes: include_bytes!("../../assets/fonts/RobotoMono-Regular.ttf"),
};
