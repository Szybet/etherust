//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]
#![windows_subsystem = "windows"]

use ggez::conf::Backend;
use ggez::conf::FullscreenType;
use ggez::conf::ModuleConf;
use ggez::conf::NumSamples;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics::BlendMode;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawMode::Stroke;
use ggez::graphics::DrawParam;
use ggez::graphics::FilterMode;
use ggez::graphics::Font;
use ggez::graphics::GlBackendSpec;
use ggez::graphics::ImageGeneric;
use ggez::graphics::LineCap;
use ggez::graphics::LineCap::Butt;
use ggez::graphics::LineJoin;
use ggez::graphics::LineJoin::Miter;
use ggez::graphics::Mesh;
use ggez::graphics::PxScale;
use ggez::graphics::StrokeOptions;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::graphics::{self, Color, Rect};
use ggez::input::mouse::position;
use ggez::input::mouse::*;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use glam::*;
use rand::Rng;
use std::env;
use std::include_bytes;
use std::path;
use std::process;
use std::{thread, time};

#[derive(Clone, Debug)]
pub struct static_rect_data {
    object_grabbed: bool,
    diffrence_y: f32,
    diffrence_x: f32,
    color: Color,
    rect_cord: Rect,
    object_id: i32,
    texture: ImageGeneric<GlBackendSpec>,
}

pub struct static_rect_button {
    color: Color,
    rect_cord: Rect,
    button_image: graphics::Image,
    button_image_clicked: graphics::Image,
    button_id: i32,
    clicked_frames: i32,
}

pub struct static_rect_slider {
    cord: Vec2,
    slider_id: i32,
    frames: f32, // 0 means the button is false and the max digits means the slider is true
    going_mode_on: bool, // false = off, true = on
    clicked_wait: i32,
    text: String,
}

pub struct static_rect_data_destination {
    color: Color,
    rect_cord: Rect,
    object_id: i32,
    connected: i32,
}

pub struct text_time {
    text: Text,
    cord: Vec2,
    color: Color,
    text_id: i32,
    time_static: i32,
    time_count: i32,
}

pub struct MainState {
    static_objects: Vec<static_rect_data>,
    static_objects_destination: Vec<static_rect_data_destination>,
    static_buttons: Vec<static_rect_button>,
    timed_text: Vec<text_time>,
    object_grabbed: i32,
    main: graphics::Image,
    true_image: graphics::Image,
    false_image: graphics::Image,
    points: i32,
    pause: bool,
    main_font: Font,
    requested_text: Vec<i32>,
    mode: String, // its a or b
    settings: Settings,
    show_mode: i32, // 0 - main, 1 - settings
}

pub struct Settings {
    clear_mode: bool, // true means clear all, false means clear only false
    sliders: Vec<static_rect_slider>,
    buttons: Vec<static_rect_button>,
}

impl Settings {
    fn new(ctx: &mut Context) -> GameResult<Settings> {
        let pos = Settings {
            clear_mode: true,
            sliders: Vec::new(),
            buttons: Vec::new(),
        };
        Ok(pos)
    }
}

pub fn draw_sliders(
    mut ctx: &mut Context,
    rect_cord_mouse: Rect,
    self_main: &mut MainState,
) -> Vec<i32> {
    let mut vec_push: Vec<i32> = Vec::new();

    let weight_x_rect_round: f32 = 200.0;
    let weight_y_rect_round: f32 = 100.0;

    for slider in &mut self_main.settings.sliders {
        let rect = Rect::new(
            slider.cord.x,
            slider.cord.y,
            weight_x_rect_round,
            weight_y_rect_round,
        );

        let drawmode_width: f32 = 15.0;

        let mut drawmode = DrawMode::Stroke(
            StrokeOptions::default()
                .with_start_cap(LineCap::Round)
                .with_end_cap(LineCap::Round)
                .with_line_join(LineJoin::Round)
                .with_line_width(15.0)
                .with_miter_limit(1.0)
                .with_tolerance(1.0),
        );

        let radius_rect: f32 = 50.0;

        let rect_round =
            graphics::Mesh::new_rounded_rectangle(ctx, drawmode, rect, radius_rect, Color::WHITE)
                .unwrap();
        graphics::draw(ctx, &rect_round, (Vec2::new(0.0, 0.0),)).unwrap();

        let radius_circle: f32 = radius_rect - drawmode_width + 9.0;
        // x 148 min, max 252, cord = 100
        let min_x = slider.cord.x + radius_circle + 7.0;
        let max_x = slider.cord.x + weight_x_rect_round - radius_circle - 7.0;
        let max_frames: f32 = 300.0;
        let percentage_from_frames: f32 = ((slider.frames * 100.0) / max_frames) / 100.0;
        let add_min_x: f32 = (max_x - min_x) * percentage_from_frames;
        let circle_cord_x_calc = min_x + add_min_x;
        let circle_cord_y = slider.cord.y + (weight_y_rect_round / 2.0);

        let mut color_circle = Color::new(0.0, 0.0, 0.0, 0.0);

        if slider.going_mode_on == false {
            color_circle = Color::RED;
            if slider.frames != 0.0 {
                slider.frames = slider.frames - 1.0;
            }
        }
        if slider.going_mode_on == true {
            color_circle = Color::GREEN;
            if slider.frames != max_frames {
                slider.frames = slider.frames + 1.0;
            }
        }

        let circle = graphics::Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::from([circle_cord_x_calc, circle_cord_y]),
            radius_circle,
            1.0,
            color_circle,
        )
        .unwrap();

        graphics::draw(ctx, &circle, (Vec2::new(0.0, 0.0),)).unwrap();

        if slider.clicked_wait == 0 {
            if rect_cord_mouse.overlaps(&rect) {
                if button_pressed(ctx, MouseButton::Left) == true {
                    vec_push.push(slider.slider_id);
                    slider.clicked_wait = 300;
                }
            }
        } else {
            slider.clicked_wait = slider.clicked_wait - 1;
        }

        let mut slider_text = Text::new(slider.text.clone());
        slider_text.set_font(self_main.main_font, PxScale::from(30.0));
        let slider_text_cord_x = slider.cord.x + weight_x_rect_round + 30.0;
        let slider_text_cord_y = slider.cord.y + (weight_y_rect_round / 2.0) - 20.0;
        graphics::draw(
            ctx,
            &slider_text,
            (
                Vec2::new(slider_text_cord_x, slider_text_cord_y),
                Color::WHITE,
            ),
        );
    }

    return vec_push;
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let main = graphics::Image::from_bytes(ctx, include_bytes!("../resources/Main.png"))?;
        let true_image = graphics::Image::from_bytes(ctx, include_bytes!("../resources/true.png"))?;
        let false_image =
            graphics::Image::from_bytes(ctx, include_bytes!("../resources/false.png"))?;
        let font =
            Font::new_glyph_font_bytes(ctx, include_bytes!("../resources/font.ttf")).unwrap();

        let pos = MainState {
            static_objects: Vec::new(),
            static_objects_destination: Vec::new(),
            static_buttons: Vec::new(),
            timed_text: Vec::new(),
            object_grabbed: 1000,
            main,
            true_image,
            false_image,
            points: 0,
            pause: false,
            main_font: font,
            requested_text: Vec::new(),
            mode: String::new(),
            settings: Settings::new(ctx).unwrap(),
            show_mode: 0,
        };
        Ok(pos)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.170, 0.169, 0.182, 1.0].into());

        // The mouse rectangle
        let mouse_position = position(ctx);
        let rect_cord_mouse = graphics::Rect::new(mouse_position.x, mouse_position.y, 0.0, 0.0);
        let rectangle_mouse = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect_cord_mouse,
            Color::new(0.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &rectangle_mouse, (Vec2::new(0.0, 0.0),))?;

        if self.show_mode == 0 {
            // Draw Main
            let dst = glam::Vec2::new(825.0, 50.0);
            graphics::draw(ctx, &self.main, (dst,))?;

            // destination object
            for cab in &mut self.static_objects_destination {
                draw_rec_func(ctx, &mut cab.rect_cord, cab.color);
                for rec in &mut self.static_objects {
                    draw_rec_func(&mut ctx, &mut rec.rect_cord, rec.color);
                    let dst = glam::Vec2::new(rec.rect_cord.x, rec.rect_cord.y);
                    graphics::draw(ctx, &rec.texture, (dst,))?;

                    if self.pause == false {
                        move_object(
                            rect_cord_mouse,
                            ctx,
                            rec,
                            &mut self.object_grabbed,
                            &mut cab.connected,
                        );
                    }
                }
            }
            // Rect objects

            manage_objects(
                rect_cord_mouse,
                ctx,
                &mut self.static_objects,
                &mut self.object_grabbed,
            );

            for cab_static in &mut self.static_objects_destination {
                for rec_connect in &mut self.static_objects {
                    grab_overlaps_connect(
                        &ctx,
                        &cab_static.rect_cord,
                        &mut rec_connect.rect_cord,
                        self.object_grabbed,
                        &mut cab_static.connected,
                        rec_connect.object_id,
                    )
                }
            }

            let buttons_clicked =
                manage_all_buttons(ctx, &mut self.static_buttons, rect_cord_mouse);
            if buttons_clicked.contains(&0) {
                declare_variables(self, ctx); // here it is, and oh no
            }
            if buttons_clicked.contains(&1) {
                if self.pause == false {
                    let mut correctly_connected: bool = true;
                    for connects in &self.static_objects_destination {
                        if connects.connected == 1000 {
                            correctly_connected = false;
                        }
                    }
                    if correctly_connected == true {
                        if self.pause == false {
                            let mut points = 0;
                            for connects in &self.static_objects_destination {
                                if connects.object_id == connects.connected {
                                    points = points + 1;
                                } else {
                                    points = points - 1;
                                }
                            }
                            self.pause = true;
                            self.points = self.points + points;
                        }
                    } else {
                        if self.requested_text.contains(&0) == false {
                            self.requested_text.push(0);
                        }
                    }
                } else {
                    if self.requested_text.contains(&1) == false {
                        self.requested_text.push(1);
                    }
                }
            }
            if buttons_clicked.contains(&2) {
                for settings_buttons in &mut self.settings.buttons {
                    if settings_buttons.button_id == 2 {
                        settings_buttons.clicked_frames = 120;
                    }
                }
                self.show_mode = 1;
            }

            if self.pause == true {
                let x_sum: f32 = 44.0;
                let mut x_cord: f32 = 860.0;
                for connects in &self.static_objects_destination {
                    let dst = glam::Vec2::new(x_cord, 723.0);
                    if connects.object_id == connects.connected {
                        graphics::draw(ctx, &self.true_image, (dst,));
                    } else {
                        graphics::draw(ctx, &self.false_image, (dst,));
                    }
                    x_cord = x_cord + x_sum;
                }
            }

            let uniform_scale_24px = PxScale::from(70.0);
            let mut points_text = Text::new(format!("Points: {}", self.points));
            points_text.set_font(self.main_font, uniform_scale_24px);

            graphics::draw(ctx, &points_text, (Vec2::new(30.0, 300.0), Color::WHITE))?;

            if self.requested_text.is_empty() == false {
                manage_requested_text(ctx, &mut self.timed_text, &mut self.requested_text)
            }

            let mut mode_text = Text::new(format!("{}", self.mode));
            mode_text.set_font(self.main_font, PxScale::from(140.0));

            graphics::draw(ctx, &mode_text, (Vec2::new(425.0, 30.0), Color::WHITE))?;
        }
        if self.show_mode == 1 {
            // settings
            let clicked_sliders = draw_sliders(ctx, rect_cord_mouse, self);

            if clicked_sliders.contains(&0) {
                for slider in &mut self.settings.sliders {
                    if slider.slider_id == 0 {
                        if slider.going_mode_on == true {
                            slider.going_mode_on = false;
                            self.settings.clear_mode = false;
                        } else {
                            slider.going_mode_on = true;
                            self.settings.clear_mode = true;
                        }
                    }
                }
            }

            let clicked_buttons =
                manage_all_buttons(ctx, &mut self.settings.buttons, rect_cord_mouse);
            if clicked_buttons.contains(&2) {
                for main_buttons in &mut self.static_buttons {
                    if main_buttons.button_id == 2 {
                        main_buttons.clicked_frames = 120;
                    }
                }
                self.show_mode = 0;
            }

            let mut tittle_text = Text::new("Settings");
            tittle_text.set_font(self.main_font, PxScale::from(100.0));
            graphics::draw(ctx, &tittle_text, (Vec2::new(475.0, 30.0), Color::WHITE))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn manage_requested_text(ctx: &mut Context, text_vec: &mut Vec<text_time>, ids: &mut Vec<i32>) {
    for id in ids.clone() {
        for text in &mut *text_vec {
            if text.text_id == id {
                graphics::draw(ctx, &text.text, (text.cord, text.color)).unwrap();
                text.time_count = text.time_count - 1;
                if text.time_count == 0 {
                    text.time_count = text.time_static;

                    let index = ids.iter().position(|&r| r == id);
                    match index {
                        Some(x) => {
                            ids.remove(x);
                        }
                        None => (),
                    }

                    /*
                    if ids.contains(&id) == true {
                        // thats the worst part of code in this file
                        let index = ids.iter().position(|&r| r == id).unwrap();
                        ids.remove(index);
                    }
                    */
                }
            }
        }
    }
}

pub fn manage_all_buttons(
    ctx: &mut Context,
    mut vector: &mut Vec<static_rect_button>,
    rect_cord_mouse: Rect,
) -> Vec<i32> {
    let mut clicked_buttons: Vec<i32> = Vec::new();
    for mut button in vector {
        draw_rec_func(ctx, &mut button.rect_cord, button.color);

        let dst = glam::Vec2::new(button.rect_cord.x, button.rect_cord.y);
        if button.clicked_frames > 0 {
            graphics::draw(ctx, &button.button_image_clicked, (dst,));
            button.clicked_frames = button.clicked_frames - 1;
        } else {
            graphics::draw(ctx, &button.button_image, (dst,));
        }

        if cursor_type(ctx) == CursorIcon::Grabbing {
        } else {
            if button.clicked_frames == 0 {
                if button.rect_cord.overlaps(&rect_cord_mouse) == true {
                    if button_pressed(ctx, MouseButton::Left) == true {
                        clicked_buttons.push(button.button_id);
                        button.clicked_frames = 120;
                    }
                }
            }
        }
    }

    return clicked_buttons;
}

pub fn move_object(
    rect_cord_mouse: Rect,
    ctx: &mut Context,
    static_rect_data: &mut static_rect_data,
    object_id: &mut i32,
    connected: &mut i32,
) {
    if object_id == &mut static_rect_data.object_id {
        if rect_cord_mouse.overlaps(&static_rect_data.rect_cord) == true {
            if static_rect_data.object_grabbed == false {
                if button_pressed(ctx, MouseButton::Left) == true {
                    static_rect_data.diffrence_y = rect_cord_mouse.y - static_rect_data.rect_cord.y;
                    static_rect_data.diffrence_x = rect_cord_mouse.x - static_rect_data.rect_cord.x;
                    static_rect_data.object_grabbed = true;
                }
            }
        }

        if static_rect_data.object_grabbed == true {
            if button_pressed(ctx, MouseButton::Left) == true {
                let width: f32 = 1280.0;
                let height: f32 = 800.0;
                let new_rect_coed_x: f32 = rect_cord_mouse.x - static_rect_data.diffrence_x;
                let new_rect_coed_y: f32 = rect_cord_mouse.y - static_rect_data.diffrence_y;

                if new_rect_coed_x + static_rect_data.rect_cord.w < width && new_rect_coed_x > 0.0 {
                    static_rect_data.rect_cord.x = new_rect_coed_x;
                }
                if new_rect_coed_y + static_rect_data.rect_cord.h < height && new_rect_coed_y > 0.0
                {
                    static_rect_data.rect_cord.y = new_rect_coed_y;
                }
                if connected == object_id {
                    *connected = 1000;
                }
            } else {
                static_rect_data.object_grabbed = false;
            }
        }
    }
}

pub fn manage_objects(
    rect_cord_mouse: Rect,
    ctx: &mut Context,
    static_rect_data: &mut Vec<static_rect_data>,
    object_grabbed: &mut i32,
) {
    let mut is_overlapping = false;

    for static_rect_data_loop in static_rect_data {
        if static_rect_data_loop.rect_cord.overlaps(&rect_cord_mouse) == true {
            is_overlapping = true;
            if object_grabbed == &mut 1000 {
                *object_grabbed = static_rect_data_loop.object_id;
            }
        }
    }

    if is_overlapping == true {
        if button_pressed(ctx, MouseButton::Left) == true {
            set_cursor_type(ctx, CursorIcon::Grabbing);
        } else {
            set_cursor_type(ctx, CursorIcon::Grab);
            *object_grabbed = 1000;
        }
    } else {
        set_cursor_type(ctx, CursorIcon::Default);
        *object_grabbed = 1000;
    }
}

pub fn draw_rec_func(ctx: &mut Context, rect: &Rect, color: Color) {
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), *rect, color).unwrap();

    graphics::draw(ctx, &rectangle, (Vec2::new(0.0, 0.0),)).unwrap();
}

pub fn grab_overlaps_connect(
    ctx: &Context,
    rect0_static: &Rect,
    rect1_to_connect: &mut Rect,
    object_grabbed: i32,
    connected: &mut i32,
    object_id: i32,
) {
    if object_grabbed == 1000 {
        if connected == &mut 1000 {
            if cursor_type(ctx) == CursorIcon::Grab {
                if rect0_static.overlaps(rect1_to_connect) {
                    rect1_to_connect.x = rect0_static.x;
                    rect1_to_connect.y = rect0_static.y;
                    *connected = object_id;
                }
            }
        }
    }
}

pub fn main() -> GameResult {
    let window_settings = WindowSetup {
        title: "etherust".to_owned(),
        samples: NumSamples::One,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    };

    let windowmode = WindowMode {
        width: 1280.0,
        height: 800.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1280.0,
        max_width: 1280.0,
        min_height: 800.0,
        max_height: 800.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false,
    };

    let cb = ggez::ContextBuilder::new("etherust", "szybet")
        .window_setup(window_settings)
        .window_mode(windowmode)
        //.modules(ModuleConf {
         //   audio: false,
         //   gamepad: false,
        //})
        .backend(Backend::OpenGL { major: 3, minor: 2 });

    let (mut ctx, event_loop) = cb.build()?;
    let mut state = MainState::new(&mut ctx)?;

    declare_variables(&mut state, &mut ctx);

    event::run(ctx, event_loop, state)
}

pub fn declare_variables(mut state: &mut MainState, ctx: &mut Context) {
    let color_transparent = Color::new(0.255, 0.255, 0.255, 0.0);
    let mut location: Point2<f32> = Point2::from([400.0, 300.0]);

    state.static_objects = Vec::new();
    state.static_objects_destination = Vec::new();
    state.static_buttons = Vec::new();
    state.object_grabbed = 1000;
    state.pause = false;

    let mut cables_vec = Vec::new();
    let cable_0_byt = include_bytes!("../resources/cables/0.png");
    let cable_1_byt = include_bytes!("../resources/cables/1.png");
    let cable_2_byt = include_bytes!("../resources/cables/2.png");
    let cable_3_byt = include_bytes!("../resources/cables/3.png");
    let cable_4_byt = include_bytes!("../resources/cables/4.png");
    let cable_5_byt = include_bytes!("../resources/cables/5.png");
    let cable_6_byt = include_bytes!("../resources/cables/6.png");
    let cable_7_byt = include_bytes!("../resources/cables/7.png");

    let mut rng = rand::thread_rng();
    if rng.gen_range(0..2) == 0 {
        // here is type B
        cables_vec = vec![
            &cable_0_byt[..],
            &cable_1_byt[..],
            &cable_2_byt[..],
            &cable_3_byt[..],
            &cable_4_byt[..],
            &cable_5_byt[..],
            &cable_6_byt[..],
            &cable_7_byt[..],
        ];
        state.mode = String::from("T568B");
    } else {
        // here is type A
        cables_vec = vec![
            &cable_2_byt[..],
            &cable_5_byt[..],
            &cable_0_byt[..],
            &cable_3_byt[..],
            &cable_4_byt[..],
            &cable_1_byt[..],
            &cable_6_byt[..],
            &cable_7_byt[..],
        ];
        state.mode = String::from("T568A");
    }

    let mut count_i32: i32 = 0;
    for num in 0..8 {
        let cable_file = cables_vec.iter().nth(num).unwrap();

        let mut rec = static_rect_data {
            object_grabbed: false,
            diffrence_y: 0.0,
            diffrence_x: 0.0,
            color: color_transparent,
            rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
            object_id: count_i32,
            texture: graphics::Image::from_bytes(ctx, cable_file).unwrap(),
        };
        state.static_objects.push(rec);
        count_i32 = count_i32 + 1;
    }

    for looping in 0..20 {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..8);
        let copy = state.static_objects.remove(random);
        state.static_objects.push(copy);
    }

    for rec in &mut state.static_objects {
        rec.rect_cord.x = location.x;
        location.x = location.x + 50.0;
    }

    // Here are defined static_rect_data_destination
    let y_cord_cab: f32 = 325.0;
    let mut x_cord_cab: f32 = 859.0;
    let mut x_cord_cab_sum: f32 = 44.0;
    let mut object_id_cab: i32 = 0;
    let color_cub: Color = color_transparent;

    for looping in 0..8 {
        let mut cab = static_rect_data_destination {
            //color: color_transparent,
            color: color_cub,
            rect_cord: Rect::new(x_cord_cab, y_cord_cab, 22.0, 396.0),
            object_id: object_id_cab,
            connected: 1000,
        };
        state.static_objects_destination.push(cab);
        x_cord_cab = x_cord_cab + x_cord_cab_sum;
        object_id_cab = object_id_cab + 1;
    }

    let mut button_reset = static_rect_button {
        color: color_transparent,
        button_image: graphics::Image::from_bytes(ctx, include_bytes!("../resources/Reset.png"))
            .unwrap(),
        button_image_clicked: graphics::Image::from_bytes(
            ctx,
            include_bytes!("../resources/Reset-clicked.png"),
        )
        .unwrap(),
        rect_cord: Rect::new(30.0, 30.0, 250.0, 100.0),
        button_id: 0,
        clicked_frames: 120,
    };
    state.static_buttons.push(button_reset);

    let mut check_button = static_rect_button {
        color: color_transparent,
        button_image: graphics::Image::from_bytes(ctx, include_bytes!("../resources/Check.png"))
            .unwrap(),
        button_image_clicked: graphics::Image::from_bytes(
            ctx,
            include_bytes!("../resources/Check-clicked.png"),
        )
        .unwrap(),
        rect_cord: Rect::new(30.0, 150.0, 235.0, 100.0),
        button_id: 1,
        clicked_frames: 0,
    };
    state.static_buttons.push(check_button);

    let mut settings_button = static_rect_button {
        color: color_transparent,
        button_image: graphics::Image::from_bytes(ctx, include_bytes!("../resources/settings.png"))
            .unwrap(),
        button_image_clicked: graphics::Image::from_bytes(
            ctx,
            include_bytes!("../resources/settings-clicked.png"),
        )
        .unwrap(),
        rect_cord: Rect::new(30.0, 720.0, 40.0, 40.0),
        button_id: 2,
        clicked_frames: 0,
    };
    state.static_buttons.push(settings_button);

    // Here is timed text
    let mut check_error_text = Text::new(format!("Connect all cables first!"));
    check_error_text.set_font(state.main_font, PxScale::from(30.0));

    let mut check_error_cables = text_time {
        text: check_error_text,
        cord: Vec2::new(20.0, 250.0),
        color: Color::RED,
        text_id: 0,
        time_static: 300,
        time_count: 300,
    };
    state.timed_text.push(check_error_cables);

    let mut check_error_text = Text::new(format!("Now click Reset"));
    check_error_text.set_font(state.main_font, PxScale::from(45.0));

    let mut check_error_cables = text_time {
        text: check_error_text,
        cord: Vec2::new(30.0, 250.0),
        color: Color::RED,
        text_id: 1,
        time_static: 300,
        time_count: 300,
    };
    state.timed_text.push(check_error_cables);

    // Sliders for settings
    let mut clearmode = static_rect_slider {
        cord: Vec2::new(100.0, 200.0),
        slider_id: 0,
        frames: 0.0,
        going_mode_on: false,
        clicked_wait: 0,
        text: String::from("If True, reset clears only not guessed cables"),
    };
    state.settings.sliders.push(clearmode);

    let mut settings_button = static_rect_button {
        color: color_transparent,
        button_image: graphics::Image::from_bytes(ctx, include_bytes!("../resources/settings.png"))
            .unwrap(),
        button_image_clicked: graphics::Image::from_bytes(
            ctx,
            include_bytes!("../resources/settings-clicked.png"),
        )
        .unwrap(),
        rect_cord: Rect::new(30.0, 720.0, 40.0, 40.0),
        button_id: 2, // becouse the same like in main
        clicked_frames: 0,
    };
    state.settings.buttons.push(settings_button);
}
