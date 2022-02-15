#[allow(dead_code)]
pub struct Logger{
    color: Color
}

enum Color{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    BackgroundRed,
    BackgroundGreen,
    BackgroundYellow,
    BackgroundBlue,
    BackgroundMagenta,
    BackgroundCyan,
    BackgroundWhite,
}

#[allow(dead_code)]
impl Logger{
    pub fn black(&mut self) -> &mut Logger {
        self.color = Color::Black;

        self
    }

    pub fn bright_black(&mut self) -> &mut Logger {
        self.color = Color::BrightBlack;
        
        self
    }

    pub fn red(&mut self) -> &mut Logger {
        self.color = Color::Red;

        self
    }

    pub fn bright_red(&mut self) -> &mut Logger {
        self.color = Color::BrightRed;

        self
    }

    pub fn green(&mut self) -> &mut Logger {
        self.color = Color::Green;

        self
    }

    pub fn bright_green(&mut self) -> &mut Logger {
        self.color = Color::BrightGreen;

        self
    }
    
    pub fn yellow(&mut self) -> &mut Logger {
        self.color = Color::Yellow;

        self
    }

    pub fn bright_yellow(&mut self) -> &mut Logger {
        self.color = Color::BrightYellow;

        self
    }

    pub fn blue(&mut self) -> &mut Logger {
        self.color = Color::Blue;

        self
    }
    
    pub fn birhgt_blue(&mut self) -> &mut Logger {
        self.color = Color::BrightBlue;

        self
    }

    pub fn magenta(&mut self) -> &mut Logger {
        self.color = Color::Magenta;

        self
    }

    pub fn bright_magenta(&mut self) -> &mut Logger {
        self.color = Color::BrightMagenta;

        self
    }
    
    pub fn cyan(&mut self) -> &mut Logger {
        self.color = Color::Cyan;

        self
    }

    pub fn bright_cyan(&mut self) -> &mut Logger {
        self.color = Color::BrightCyan;

        self
    }

    pub fn white(&mut self) -> &mut Logger {
        self.color = Color::White;

        self
    }

    pub fn bright_white(&mut self) -> &mut Logger {
        self.color = Color::BrightWhite;

        self
    }

    // pub fn bgRed(&mut self) -> &mut Logger {
    //     self.color = "\x1b[40m$TEXT\x1b[0m".to_string();

    //     self
    // }

    pub fn bg_red(&mut self) -> &mut Logger {
        self.color = Color::BackgroundRed;

        self
    }

    pub fn bg_green(&mut self) -> &mut Logger {
        self.color = Color::BackgroundGreen;

        self
    }

    pub fn bg_yellow(&mut self) -> &mut Logger {
        self.color = Color::BackgroundYellow;

        self
    }

    pub fn bg_blue(&mut self) -> &mut Logger {
        self.color = Color::BackgroundBlue;

        self
    }

    pub fn bg_magenta(&mut self) -> &mut Logger {
        self.color = Color::BackgroundMagenta;

        self
    }

    pub fn bg_cyan(&mut self) -> &mut Logger {
        self.color = Color::BackgroundCyan;

        self
    }

    pub fn bg_white(&mut self) -> &mut Logger {
        self.color = Color::BackgroundWhite;

        self
    }

    pub fn log(&mut self, text: &str) {
        let text = format!("{}{}m{}{}", "\x1b[", &self.get_color(), text, "\x1b[0m");
        println!("{}", text);
    }

    pub fn warn(&mut self, text: &str) {
        self.bg_red().log(text);
    }

    fn get_color(&mut self) -> String {
        let color = match self.color {
            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),
            Color::BackgroundRed => "41".into(),
            Color::BackgroundGreen => "42".into(),
            Color::BackgroundYellow => "43".into(),
            Color::BackgroundBlue => "44".into(),
            Color::BackgroundMagenta => "45".into(),
            Color::BackgroundCyan => "46".into(),
            Color::BackgroundWhite => "47".into(),
            // Color::TrueColor { r, g, b } => format!("38;2;{};{};{}", r, g, b).into(),
        };

        color
    }

    pub fn get_color_text(&mut self, text: &str) -> String {
        format!("{}{}m{}{}", "\x1b[", &self.get_color(), text, "\x1b[0m")
    }
}

pub fn new() -> (Logger, Logger) {
    (
        Logger{
            color: Color::Yellow
        },
        Logger{
            color: Color::Red
        }
    )
}