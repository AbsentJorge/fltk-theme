use super::*;
use fltk::enums::FrameType;

const ACCENT: u8 = 16;

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x, y, w, h, c);
    set_draw_color(Color::by_index(ACCENT));
    draw_rect(x, y, w, h)
}

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x, y, w, h, c);
    up_frame(x, y, w, h, c);
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, c);
    down_frame(x, y, w, h, c);
}

fn up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::by_index(ACCENT));
    draw_xyline2(x + w - 1, y, x, y + h - 1); // Outer line
    draw_xyline2(x + w - 1, y + 1, x + 1, y + h - 1); // Inner line

    set_draw_color(Color::Dark2);
    draw_xyline2(x + 2, y + h - 1, x + w - 1, y); // Outer line
    draw_xyline2(x + 2, y + h - 2, x + w - 2, y); // Inner line
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::Dark2);
    draw_xyline2(x + w - 1, y, x, y + h - 1); // Outer line
    draw_xyline2(x + w - 2, y + 1, x + 1, y + h - 1); // Inner line

    set_draw_color(Color::Light1);
    draw_xyline2(x + 2, y + h - 1, x + w - 1, y); // Outer line
    draw_xyline2(x + 2, y + h - 2, x + w - 2, y); // Inner line
}

fn thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, c);
    thin_up_frame(x, y, w, h, c);
}

fn thin_down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_rect_fill(x + 1, y + 1, w - 2, h - 2, c);
    thin_down_frame(x, y, w, h, c);
}

fn thin_up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::by_index(ACCENT));
    draw_xyline2(x + w - 1, y, x, y + h - 1); // Outer line

    set_draw_color(Color::Dark2);
    draw_xyline2(x, y + h - 1, x + w - 1, y + 1); // Outer line
}

fn thin_down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::Dark2);
    draw_xyline2(x + w - 1, y, x, y + h - 1); // Top line

    set_draw_color(Color::Light1);
    draw_xyline2(x, y + h - 1, x + w - 1, y + 1); // Bottom line
}

fn embossed_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::by_index(ACCENT));
    draw_rect(x + 1, y + 1, w - 1, h - 1);

    set_draw_color(Color::Dark3);

    draw_xyline2(x + w - 1, y, x, y + h - 1);
    draw_xyline2(x + 2, y + h - 2, x + w - 2, y + 2);
}

fn embossed_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(Color::Background);
    draw_rectf(x, y, w, h);
    embossed_frame(x, y, w, h, c);
}

fn round_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    draw_box(FrameType::OFlatBox, x, y, w, h, Color::Background);
    draw_box(FrameType::OvalFrame, x, y, w, h, Color::Selection);
}

pub(crate) fn use_jewel_scheme() {
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Plastic);

    app::set_visible_focus(false);
    app::set_font_size(12);

    app::set_frame_type_cb(FrameType::UpBox, up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::DownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::UpFrame, up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::DownFrame, down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::ThinUpBox, thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::ThinDownBox, thin_down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::ThinUpFrame, thin_up_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::ThinDownFrame, thin_down_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::BorderBox, border_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::EmbossedFrame, embossed_frame, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::EmbossedBox, embossed_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::RoundUpBox, round_box, 1, 1, 2, 2);
    app::set_frame_type_cb(FrameType::RoundDownBox, round_box, 2, 2, 4, 4);
}
