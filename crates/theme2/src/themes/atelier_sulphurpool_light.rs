
use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn atelier_sulphurpool_light() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Atelier Sulphurpool Light".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x9a9fb6ff).into(),
        border_variant: rgba(0x9a9fb6ff).into(),
        border_focused: rgba(0xc2d5efff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xc1c5d8ff).into(),
        surface: rgba(0xe5e8f5ff).into(),
        background: rgba(0xc1c5d8ff).into(),
        filled_element: rgba(0xc1c5d8ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xdde7f6ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xdde7f6ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x202646ff).into(),
        text_muted: rgba(0x5f6789ff).into(),
        text_placeholder: rgba(0xc94922ff).into(),
        text_disabled: rgba(0x767d9aff).into(),
        text_accent: rgba(0x3e8fd0ff).into(),
        icon_muted: rgba(0x5f6789ff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("string.special".into(), rgba(0x9b6279ff).into()),
                ("string.regex".into(), rgba(0x21a2c9ff).into()),
                ("embedded".into(), rgba(0x202646ff).into()),
                ("string".into(), rgba(0xac9738ff).into()),
                (
                    "function.special.definition".into(),
                    rgba(0xc08b2fff).into(),
                ),
                ("hint".into(), rgba(0x7087b2ff).into()),
                ("function.method".into(), rgba(0x3d8fd1ff).into()),
                ("punctuation.list_marker".into(), rgba(0x293256ff).into()),
                ("punctuation".into(), rgba(0x293256ff).into()),
                ("constant".into(), rgba(0xac9739ff).into()),
                ("label".into(), rgba(0x3e8fd0ff).into()),
                ("comment.doc".into(), rgba(0x5d6587ff).into()),
                ("property".into(), rgba(0xc94821ff).into()),
                ("punctuation.bracket".into(), rgba(0x5d6587ff).into()),
                ("constructor".into(), rgba(0x3e8fd0ff).into()),
                ("variable.special".into(), rgba(0x6679ccff).into()),
                ("emphasis".into(), rgba(0x3e8fd0ff).into()),
                ("link_text".into(), rgba(0xc76a29ff).into()),
                ("keyword".into(), rgba(0x6679ccff).into()),
                ("primary".into(), rgba(0x293256ff).into()),
                ("comment".into(), rgba(0x898ea4ff).into()),
                ("title".into(), rgba(0x202646ff).into()),
                ("link_uri".into(), rgba(0xac9739ff).into()),
                ("text.literal".into(), rgba(0xc76a29ff).into()),
                ("operator".into(), rgba(0x5d6587ff).into()),
                ("number".into(), rgba(0xc76a28ff).into()),
                ("preproc".into(), rgba(0x202646ff).into()),
                ("attribute".into(), rgba(0x3e8fd0ff).into()),
                ("emphasis.strong".into(), rgba(0x3e8fd0ff).into()),
                ("string.escape".into(), rgba(0x5d6587ff).into()),
                ("tag".into(), rgba(0x3e8fd0ff).into()),
                ("variable".into(), rgba(0x293256ff).into()),
                ("predictive".into(), rgba(0x8599beff).into()),
                ("enum".into(), rgba(0xc76a29ff).into()),
                ("string.special.symbol".into(), rgba(0xac9738ff).into()),
                ("punctuation.delimiter".into(), rgba(0x5d6587ff).into()),
                ("function".into(), rgba(0x3d8fd1ff).into()),
                ("type".into(), rgba(0xc08b2fff).into()),
                ("punctuation.special".into(), rgba(0x9b6279ff).into()),
                ("variant".into(), rgba(0xc08b2fff).into()),
                ("boolean".into(), rgba(0xac9739ff).into()),
            ],
        },
        status_bar: rgba(0xc1c5d8ff).into(),
        title_bar: rgba(0xc1c5d8ff).into(),
        toolbar: rgba(0xf5f7ffff).into(),
        tab_bar: rgba(0xe5e8f5ff).into(),
        editor: rgba(0xf5f7ffff).into(),
        editor_subheader: rgba(0xe5e8f5ff).into(),
        editor_active_line: rgba(0xe5e8f5ff).into(),
        terminal: rgba(0xf5f7ffff).into(),
        image_fallback_background: rgba(0xc1c5d8ff).into(),
        git_created: rgba(0xac9739ff).into(),
        git_modified: rgba(0x3e8fd0ff).into(),
        git_deleted: rgba(0xc94922ff).into(),
        git_conflict: rgba(0xc08b30ff).into(),
        git_ignored: rgba(0x767d9aff).into(),
        git_renamed: rgba(0xc08b30ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x3e8fd0ff).into(),
                selection: rgba(0x3e8fd03d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xac9739ff).into(),
                selection: rgba(0xac97393d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9b6279ff).into(),
                selection: rgba(0x9b62793d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc76a29ff).into(),
                selection: rgba(0xc76a293d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x6679cbff).into(),
                selection: rgba(0x6679cb3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x24a1c9ff).into(),
                selection: rgba(0x24a1c93d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc94922ff).into(),
                selection: rgba(0xc949223d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc08b30ff).into(),
                selection: rgba(0xc08b303d).into(),
            },
        ],
    }
}
