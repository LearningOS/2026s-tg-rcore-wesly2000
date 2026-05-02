//! 七巧板图案显示模块
//!
//! 本模块使用 ANSI 转义序列在终端显示彩色的七巧板"OS"图案。
//! 由于 ch1 是最简单的裸机程序，这里采用串口输出的方式，
//! 而不是实现完整的 VirtIO-GPU 驱动。

use tg_sbi::console_putchar;

/// 颜色代码（ANSI 256 色）
const COLOR_RED: u8 = 196;
const COLOR_ORANGE: u8 = 208;
const COLOR_YELLOW: u8 = 226;
const COLOR_GREEN: u8 = 46;
const COLOR_CYAN: u8 = 51;
const COLOR_BLUE: u8 = 21;
const COLOR_PURPLE: u8 = 129;

/// 七巧板"OS"图案数据
/// 使用 ASCII 艺术表示，每个字符代表一个颜色块
const TANGRAM_PATTERN: &[&[u8]] = &[
    b"                                                                                ",
    b"                                                                                ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"  ########################  ########################  ################          ",
    b"  ########################  ########################  ################          ",
    b"  ########################  ########################  ################          ",
    b"  ########################  ########################  ################          ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"      ################      ################        ################            ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"          ########              ########                ########                ",
    b"                                                                                ",
    b"                                                                                ",
];

/// 颜色映射：根据位置返回对应的颜色
fn get_color_at(x: usize, y: usize) -> u8 {
    // 简化的颜色映射逻辑
    // 根据位置返回不同的颜色，模拟七巧板的不同块

    if y < 6 {
        if x < 20 { COLOR_RED }
        else if x < 40 { COLOR_ORANGE }
        else if x < 60 { COLOR_YELLOW }
        else { COLOR_GREEN }
    } else if y < 12 {
        if x < 20 { COLOR_ORANGE }
        else if x < 40 { COLOR_YELLOW }
        else if x < 60 { COLOR_GREEN }
        else { COLOR_CYAN }
    } else if y < 18 {
        if x < 20 { COLOR_YELLOW }
        else if x < 40 { COLOR_GREEN }
        else if x < 60 { COLOR_CYAN }
        else { COLOR_BLUE }
    } else {
        if x < 20 { COLOR_GREEN }
        else if x < 40 { COLOR_CYAN }
        else if x < 60 { COLOR_BLUE }
        else { COLOR_PURPLE }
    }
}

/// 输出 ANSI 颜色代码
fn set_color(color: u8) {
    // ANSI 256 色模式：\x1b[38;5;Nm
    let seq = b"\x1b[38;5;";
    for &byte in seq {
        console_putchar(byte);
    }

    // 输出颜色代码
    if color >= 100 {
        console_putchar(b'0' + color / 100);
        console_putchar(b'0' + (color % 100) / 10);
        console_putchar(b'0' + color % 10);
    } else if color >= 10 {
        console_putchar(b'0' + color / 10);
        console_putchar(b'0' + color % 10);
    } else {
        console_putchar(b'0' + color);
    }

    console_putchar(b'm');
}

/// 重置颜色
fn reset_color() {
    let seq = b"\x1b[0m";
    for &byte in seq {
        console_putchar(byte);
    }
}

/// 输出字符串
fn print_str(s: &[u8]) {
    for &c in s {
        console_putchar(c);
    }
}

/// 显示七巧板"OS"图案
pub fn display_tangram() {
    print_str(b"\n");
    print_str(b"=== Tangram OS Pattern ===\n");
    print_str(b"\n");

    // 逐行输出图案
    for (y, line) in TANGRAM_PATTERN.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == b'#' {
                // 根据位置设置颜色
                let color = get_color_at(x, y);
                set_color(color);
                console_putchar(b'#');
                reset_color();
            } else {
                console_putchar(ch);
            }
        }
        console_putchar(b'\n');
    }

    print_str(b"\n");
    print_str(b"=========================\n");
}
