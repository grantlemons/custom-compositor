#![allow(dead_code)]

static BORDER_PX: u8 = 1; // border pixel of windows
static SNAP: u8 = 32; // snap pixel
static SHOW_BAR: u8 = 1; // 0 means no bar
static TOP_BAR: u8 = 1; // 0 means bottom bar
static FONTS: &[&str] = &["monospace:size=10"];
static DMENUFONT: &str = "monospace:size=10";

type Color = &'static str;
enum ColorScheme {
    Normal(Color, Color, Color),
    Selected(Color, Color, Color),
}

static GRAY_1: Color = "#222222"; // background color
static GRAY_2: Color = "#444444"; // inactive window border color
static GRAY_3: Color = "#bbbbbb"; // font color
static GRAY_4: Color = "#eeeeee"; // current tag and current window font color
static CYAN: Color = "#006080"; // top bar second color(blue) and active window border color

static COLORS: [ColorScheme; 2] = [
    //                  fg      bg      border
    ColorScheme::Normal(GRAY_3, GRAY_1, GRAY_2),
    ColorScheme::Selected(GRAY_4, CYAN, CYAN),
];

// tagging
static TAGS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

// layout(s)
static MFACT: f32 = 0.55; // factor of master area size [0.05..0.95]
static NMASTER: u8 = 1; // number of clients in master area
static RESIZEHINTS: u8 = 0; // 1 means respect size hints in tiled resizals
static ATTACHBELOW: u8 = 1; // 1 means attach after the currently active window
static LOCKFULLSCREEN: u8 = 1; // 1 will force focus on the fullscreen window

enum Layout {
    Floating,
    Tiling,
    Monocle,
}

static LAYOUTS: [(&str, Layout); 3] = [
    ("><>", Layout::Floating),
    ("[]=", Layout::Tiling),
    ("[M]", Layout::Monocle),
];
