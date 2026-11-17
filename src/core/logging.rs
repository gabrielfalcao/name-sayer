use chrono::{DateTime, Local};
use fern::colors::{Color, ColoredLevelConfig, WithFgColor};
use iocore::Path;
use log::{Level, debug, error, info, log_enabled, trace, warn};
use owo_colors::{OwoColorize, Style,Rgb};

pub fn color_for_level(level: log::Level) -> Color {
    use log::Level::*;
    match level {
        Off => (Color::TrueColor { r: 171, g: 148, b: 122 }),
        Info => (Color::TrueColor { r: 77, g: 155, b: 230 }),
        Warn => (Color::TrueColor { r: 247, g: 150, b: 23 }),
        Error => (Color::TrueColor { r: 232, g: 59, b: 59 }),
        Debug => (Color::TrueColor { r: 199, g: 220, b: 208 }),
        Trace => (Color::TrueColor { r: 240, g: 79, b: 120 }),
    }
}
pub fn rgb_from_fern_color(color: Color) -> Rgb {
    let Color::TrueColor {r,g,b} = color else {unreachable!()};
    Rgb(r, g, b)

}
pub fn style_from_fern_color(color: Color) -> Style {
    let Color::TrueColor {r,g,b} = color else {unreachable!()};
    Style::new().truecolor(r, g, b)
}

pub fn setup_logger(
    level: log::Level,
    output_file: Option<Path>,
) -> Result<(), fern::InitError> {
    // let mut colors = ColoredLevelConfig::new()
    //     .info(COLOR_INFO)
    //     .warn(COLOR_WARN)
    //     .error(COLOR_ERROR)
    //     .debug(COLOR_DEBUG)
    //     .trace(COLOR_TRACE);

    let mut dsp = fern::Dispatch::new()
        .format(move |out, message, record| {
            let now = Local::now();
            let timestamp = now.format("%Y/%m/%dT%H:%M:%S.%f");
            let color = color_for_level(level);
            let style = style_from_fern_color(color);
            let level = level.as_str().to_string();

            let target = record.target();
            let message = message.style(style);

            out.finish(format_args!("\x1b[0m[{level}] [{timestamp}] {target}: {message}", level=level.style(style)))
        })
        .level(level.to_level_filter())
        .chain(std::io::stderr());
    if let Some(output_file) = output_file {
        dsp = dsp.chain(match fern::log_file(&output_file) {
            Ok(output) => output,
            Err(error) => {
                log::error!("failed to setup log output to {output_file}: {error}");
                std::process::exit(1);
            }
        })
    }
    dsp.apply()?;
    Ok(())
}

// #2E222F  \x1b[1;38;2;46;34;47m      r:46, g: 34, b: 47,
// #3E3546  \x1b[1;38;2;62;53;70m      r:62, g: 53, b: 70,
// #625565  \x1b[1;38;2;98;85;101m     r:98, g: 85, b: 101,
// #966C6C  \x1b[1;38;2;150;108;108m  r:150, g: 108, b: 108,
// #AB947A  \x1b[1;38;2;171;148;122m  r:171, g: 148, b: 122,
// #694F62  \x1b[1;38;2;105;79;98m    r:105, g: 79, b: 98,
// #7F708A  \x1b[1;38;2;127;112;138m  r:127, g: 112, b: 138,
// #9BABB2  \x1b[1;38;2;155;171;178m  r:155, g: 171, b: 178,
// #C7DCD0  \x1b[1;38;2;199;220;208m  r:199, g: 220, b: 208,
// #FFFFFF  \x1b[1;38;2;255;255;255m  r:255, g: 255, b: 255,
// #6E2727  \x1b[1;38;2;110;39;39m    r:110, g: 39, b: 39,
// #B33831  \x1b[1;38;2;179;56;49m    r:179, g: 56, b: 49,
// #EA4F36  \x1b[1;38;2;234;79;54m    r:234, g: 79, b: 54,
// #F57D4A  \x1b[1;38;2;245;125;74m   r:245, g: 125, b: 74,
// #AE2334  \x1b[1;38;2;174;35;52m    r:174, g: 35, b: 52,
// #E83B3B  \x1b[1;38;2;232;59;59m    r:232, g: 59, b: 59,
// #FB6B1D  \x1b[1;38;2;251;107;29m   r:251, g: 107, b: 29,
// #F79617  \x1b[1;38;2;247;150;23m   r:247, g: 150, b: 23,
// #F9C22B  \x1b[1;38;2;249;194;43m   r:249, g: 194, b: 43,
// #7A3045  \x1b[1;38;2;122;48;69m    r:122, g: 48, b: 69,
// #9E4539  \x1b[1;38;2;158;69;57m    r:158, g: 69, b: 57,
// #CD683D  \x1b[1;38;2;205;104;61m   r:205, g: 104, b: 61,
// #E6904E  \x1b[1;38;2;230;144;78m   r:230, g: 144, b: 78,
// #FBB954  \x1b[1;38;2;251;185;84m   r:251, g: 185, b: 84,
// #4C3E24  \x1b[1;38;2;76;62;36m      r:76, g: 62, b: 36,
// #676633  \x1b[1;38;2;103;102;51m   r:103, g: 102, b: 51,
// #A2A947  \x1b[1;38;2;162;169;71m   r:162, g: 169, b: 71,
// #D5E04B  \x1b[1;38;2;213;224;75m   r:213, g: 224, b: 75,
// #FBFF86  \x1b[1;38;2;251;255;134m  r:251, g: 255, b: 134,
// #165A4C  \x1b[1;38;2;22;90;76m      r:22, g: 90, b: 76,
// #239063  \x1b[1;38;2;35;144;99m     r:35, g: 144, b: 99,
// #1EBC73  \x1b[1;38;2;30;188;115m    r:30, g: 188, b: 115,
// #91DB69  \x1b[1;38;2;145;219;105m  r:145, g: 219, b: 105,
// #CDDF6C  \x1b[1;38;2;205;223;108m  r:205, g: 223, b: 108,
// #313638  \x1b[1;38;2;49;54;56m      r:49, g: 54, b: 56,
// #374E4A  \x1b[1;38;2;55;78;74m      r:55, g: 78, b: 74,
// #547E64  \x1b[1;38;2;84;126;100m    r:84, g: 126, b: 100,
// #92A984  \x1b[1;38;2;146;169;132m  r:146, g: 169, b: 132,
// #B2BA90  \x1b[1;38;2;178;186;144m  r:178, g: 186, b: 144,
// #0B5E65  \x1b[1;38;2;11;94;101m     r:11, g: 94, b: 101,
// #0B8A8F  \x1b[1;38;2;11;138;143m    r:11, g: 138, b: 143,
// #0EAF9B  \x1b[1;38;2;14;175;155m    r:14, g: 175, b: 155,
// #30E1B9  \x1b[1;38;2;48;225;185m    r:48, g: 225, b: 185,
// #8FF8E2  \x1b[1;38;2;143;248;226m  r:143, g: 248, b: 226,
// #323353  \x1b[1;38;2;50;51;83m      r:50, g: 51, b: 83,
// #484A77  \x1b[1;38;2;72;74;119m     r:72, g: 74, b: 119,
// #4D65B4  \x1b[1;38;2;77;101;180m    r:77, g: 101, b: 180,
// #4D9BE6  \x1b[1;38;2;77;155;230m    r:77, g: 155, b: 230,
// #8FD3FF  \x1b[1;38;2;143;211;255m  r:143, g: 211, b: 255,
// #45293F  \x1b[1;38;2;69;41;63m      r:69, g: 41, b: 63,
// #6B3E75  \x1b[1;38;2;107;62;117m   r:107, g: 62, b: 117,
// #905EA9  \x1b[1;38;2;144;94;169m   r:144, g: 94, b: 169,
// #A884F3  \x1b[1;38;2;168;132;243m  r:168, g: 132, b: 243,
// #EAADED  \x1b[1;38;2;234;173;237m  r:234, g: 173, b: 237,
// #753C54  \x1b[1;38;2;117;60;84m    r:117, g: 60, b: 84,
// #A24B6F  \x1b[1;38;2;162;75;111m   r:162, g: 75, b: 111,
// #CF657F  \x1b[1;38;2;207;101;127m  r:207, g: 101, b: 127,
// #ED8099  \x1b[1;38;2;237;128;153m  r:237, g: 128, b: 153,
// #831C5D  \x1b[1;38;2;131;28;93m    r:131, g: 28, b: 93,
// #C32454  \x1b[1;38;2;195;36;84m    r:195, g: 36, b: 84,
// #F04F78  \x1b[1;38;2;240;79;120m   r:240, g: 79, b: 120,
// #F68181  \x1b[1;38;2;246;129;129m  r:246, g: 129, b: 129,
// #FCA790  \x1b[1;38;2;252;167;144m  r:252, g: 167, b: 144,
// #FDCBB0  \x1b[1;38;2;253;203;176m  r:253, g: 203, b: 176,
