pub const TERMINAL: &str = "alacritty";
pub const BROWSER: &str = "qutebrowser";
pub const HOME: &str = env!("HOME");
pub const DMENU_EXEC: &str = "dmenu_run";

type Color = &'static str;

pub const GREEN: Color = "";
pub const BG: Color = "";

const N_MAIN: u32 = 1;
const RATIO: f32 = 0.6;

#[macro_export]
macro_rules! gen_layout {
    () => {};
    {$( $symbol: literal, $func: item); *} => {{
        vec![$(
            penrose::layout::Layout::new($symbol, , $func, N_MAIN, RATIO)
        )*]
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
