#[macro_use]
extern crate penrose;

use penrose_config::{gen_layout, BROWSER, HOME, TERMINAL};

use penrose::{
    core::{
        helpers::{index_selectors, spawn_with_args},
        hooks::Hook,
        layout::{bottom_stack, side_stack, Layout, LayoutConf},
    },
    draw::{dwm_bar, Color, TextStyle},
    logging_error_handler,
    xcb::{new_xcb_backed_window_manager, XcbDraw},
    Backward, Config, Forward, Less, More, Selector,
};

fn main() -> penrose::Result<()> {
    let floating_class = vec!["dmenu", "dunst", "pinentry-qt", "firefox"];

    let test = gen_layout! {
        "value", side_stack, LayoutConf::default();

    };

    let layouts: Vec<Layout> = {
        let n_main = 1;
        let ratio = 0.6;

        vec![Layout::new(
            "[]=",
            LayoutConf::default(),
            side_stack,
            n_main,
            ratio,
        )]
    };

    let config = Config::default()
        .builder()
        .workspaces(vec!["dev", "web", "other", "4", "5", "6", "7", "8", "9"])
        .layouts(layouts)
        .gap_px(3)
        .floating_classes(floating_class)
        .top_bar(true)
        .show_bar(true)
        .build()
        .map_err(|e| perror!("{}", e))
        .unwrap();

    let key_bindings = gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);

        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-j" => run_internal!(cycle_layout, Forward);
        "M-A-k" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);
        "M-p" => run_external!("dmenu_run");
        "M-Return" => run_external!("alacritty");
        "M-A-Escape" => run_internal!(exit);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})?;

    // No XCONN access after this point

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn check_keybindings() {
        penrose::core::helpers::keycodes_from_xmodmap();

        assert_eq!(2, 2);
    }
}
