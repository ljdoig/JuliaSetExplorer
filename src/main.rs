use explorer::simulation::State;
use mandelbruhst_cli::palette::{ColorPalette, ConfigRGB};

fn main() {
    let palette = ColorPalette::new(vec![
        ConfigRGB {
            value: 0.0,
            red: 0,
            green: 18,
            blue: 25,
        },
        ConfigRGB {
            value: 0.1,
            red: 20,
            green: 33,
            blue: 61,
        },
        ConfigRGB {
            value: 0.25,
            red: 252,
            green: 163,
            blue: 17,
        },
        ConfigRGB {
            value: 0.5,
            red: 229,
            green: 229,
            blue: 229,
        },
        ConfigRGB {
            value: 1.0,
            red: 255,
            green: 255,
            blue: 255,
        },
    ])
    .unwrap();

    State::new(palette).run();
}
