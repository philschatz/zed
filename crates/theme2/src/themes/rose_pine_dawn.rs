
use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn rose_pine_dawn() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Rosé Pine Dawn".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0xdcd6d5ff).into(),
        border_variant: rgba(0xdcd6d5ff).into(),
        border_focused: rgba(0xc3d7dbff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xdcd8d8ff).into(),
        surface: rgba(0xfef9f2ff).into(),
        background: rgba(0xdcd8d8ff).into(),
        filled_element: rgba(0xdcd8d8ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xdde9ebff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xdde9ebff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x575279ff).into(),
        text_muted: rgba(0x706c8cff).into(),
        text_placeholder: rgba(0xb4647aff).into(),
        text_disabled: rgba(0x938fa3ff).into(),
        text_accent: rgba(0x57949fff).into(),
        icon_muted: rgba(0x706c8cff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("primary".into(), rgba(0x575279ff).into()),
                ("attribute".into(), rgba(0x57949fff).into()),
                ("operator".into(), rgba(0x276983ff).into()),
                ("boolean".into(), rgba(0xd7827dff).into()),
                ("tag".into(), rgba(0x55949fff).into()),
                ("enum".into(), rgba(0x9079a9ff).into()),
                ("embedded".into(), rgba(0x575279ff).into()),
                ("label".into(), rgba(0x57949fff).into()),
                ("function.method".into(), rgba(0xd7827dff).into()),
                ("punctuation.list_marker".into(), rgba(0x635e82ff).into()),
                ("punctuation.delimiter".into(), rgba(0x635e82ff).into()),
                ("string".into(), rgba(0xea9d34ff).into()),
                ("type".into(), rgba(0x55949fff).into()),
                ("string.regex".into(), rgba(0x9079a9ff).into()),
                ("variable".into(), rgba(0x575279ff).into()),
                ("constructor".into(), rgba(0x57949fff).into()),
                ("punctuation.bracket".into(), rgba(0x635e82ff).into()),
                ("emphasis".into(), rgba(0x57949fff).into()),
                ("comment.doc".into(), rgba(0x6e6a8bff).into()),
                ("comment".into(), rgba(0x9893a5ff).into()),
                ("keyword".into(), rgba(0x276983ff).into()),
                ("preproc".into(), rgba(0x575279ff).into()),
                ("string.special".into(), rgba(0x9079a9ff).into()),
                ("string.escape".into(), rgba(0x6e6a8bff).into()),
                ("constant".into(), rgba(0x3daa8eff).into()),
                ("property".into(), rgba(0x57949fff).into()),
                ("punctuation.special".into(), rgba(0x635e82ff).into()),
                ("text.literal".into(), rgba(0x9079a9ff).into()),
                ("type.builtin".into(), rgba(0x55949fff).into()),
                ("string.special.symbol".into(), rgba(0x9079a9ff).into()),
                ("link_uri".into(), rgba(0xd7827dff).into()),
                ("number".into(), rgba(0x3daa8eff).into()),
                ("emphasis.strong".into(), rgba(0x57949fff).into()),
                ("function".into(), rgba(0xd7827dff).into()),
                ("title".into(), rgba(0xea9d34ff).into()),
                ("punctuation".into(), rgba(0x797593ff).into()),
                ("link_text".into(), rgba(0x55949fff).into()),
                ("variant".into(), rgba(0x57949fff).into()),
                ("predictive".into(), rgba(0xa2acbeff).into()),
                ("hint".into(), rgba(0x7a92aaff).into()),
            ],
        },
        status_bar: rgba(0xdcd8d8ff).into(),
        title_bar: rgba(0xdcd8d8ff).into(),
        toolbar: rgba(0xfaf4edff).into(),
        tab_bar: rgba(0xfef9f2ff).into(),
        editor: rgba(0xfaf4edff).into(),
        editor_subheader: rgba(0xfef9f2ff).into(),
        editor_active_line: rgba(0xfef9f2ff).into(),
        terminal: rgba(0xfaf4edff).into(),
        image_fallback_background: rgba(0xdcd8d8ff).into(),
        git_created: rgba(0x3daa8eff).into(),
        git_modified: rgba(0x57949fff).into(),
        git_deleted: rgba(0xb4647aff).into(),
        git_conflict: rgba(0xe99d35ff).into(),
        git_ignored: rgba(0x938fa3ff).into(),
        git_renamed: rgba(0xe99d35ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x57949fff).into(),
                selection: rgba(0x57949f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x3daa8eff).into(),
                selection: rgba(0x3daa8e3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x7c697fff).into(),
                selection: rgba(0x7c697f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9079a9ff).into(),
                selection: rgba(0x9079a93d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9079a9ff).into(),
                selection: rgba(0x9079a93d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x296983ff).into(),
                selection: rgba(0x2969833d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb4647aff).into(),
                selection: rgba(0xb4647a3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xe99d35ff).into(),
                selection: rgba(0xe99d353d).into(),
            },
        ],
    }
}
