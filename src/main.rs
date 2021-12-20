//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::conf::FullscreenType;
use ggez::conf::NumSamples;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics::BlendMode;
use ggez::graphics::DrawParam;
use ggez::graphics::FilterMode;
use ggez::graphics::Font;
use ggez::graphics::GlBackendSpec;
use ggez::graphics::ImageGeneric;
use ggez::graphics::Mesh;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::mouse::position;
use ggez::input::mouse::*;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use glam::*;
use rand::Rng;
use std::env;
use std::path;
use std::path::PathBuf;

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

pub struct static_rect_data_destination {
    color: Color,
    rect_cord: Rect,
    object_id: i32,
    connected: i32,
}

pub struct MainState {
    static_objects: Vec<static_rect_data>,
    static_objects_destination: Vec<static_rect_data_destination>,
    static_buttons: Vec<static_rect_button>,
    object_grabbed: i32,
    main: graphics::Image,
    true_image: graphics::Image,
    false_image: graphics::Image,
    points: i32,
    pause: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let main = graphics::Image::new(ctx, "/Main.png")?;
        let true_image = graphics::Image::new(ctx, "/true.png")?;
        let false_image = graphics::Image::new(ctx, "/false.png")?;

        let pos = MainState {
            static_objects: Vec::new(),
            static_objects_destination: Vec::new(),
            static_buttons: Vec::new(),
            object_grabbed: 1000,
            main,
            true_image,
            false_image,
            points: 0,
            pause: false,
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

        let buttons_clicked = manage_all_buttons(ctx, &mut self.static_buttons, rect_cord_mouse);
        if buttons_clicked.contains(&0) {
            declare_variables(self, ctx);
        }
        if buttons_clicked.contains(&1) {
            if self.pause == false {
                let x_sum: f32 = 44.0;
                let mut x_cord: f32 = 860.0;
                let mut points = 0;

                for connects in &self.static_objects_destination {
                    let dst = glam::Vec2::new(x_cord, 723.0);
                    if connects.object_id == connects.connected {
                        graphics::draw(ctx, &self.true_image, (dst,));
                        points = points + 1;
                    } else {
                        graphics::draw(ctx, &self.false_image, (dst,));
                        points = points - 1;
                    }
                    x_cord = x_cord + x_sum;
                }
                self.pause = true;
                self.points = points;
            }
        }

        let font = Font::new(ctx, "/font.ttf").unwrap();
        let uniform_scale_24px = PxScale::from(24.0);

        let string_text = self.points.to_string();
        let mut text_fragment = TextFragment::new(string_text)
            .font(font)
            .scale(uniform_scale_24px)
            .color(Color::new(0.0, 1.0, 1.0, 1.0));

        let mut text: Text = Text::new(text_fragment);
        let dst = glam::Vec2::new(300.0, 300.0);

        graphics::queue_text(ctx, &text, dst, Some(Color::new(0.0, 1.0, 1.0, 1.0)));
        // check cos nie dziala
        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            Some(BlendMode::from(BlendMode::Add)),
            FilterMode::Nearest,
        )
        .unwrap();

        graphics::present(ctx)?;
        Ok(())
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
            if button.rect_cord.overlaps(&rect_cord_mouse) == true {
                if button_pressed(ctx, MouseButton::Left) == true {
                    clicked_buttons.push(button.button_id);
                    button.clicked_frames = 120;
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

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("etherust", "szybet")
        .window_setup(window_settings)
        .window_mode(windowmode)
        .add_resource_path(resource_dir);

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

    // Here are defined static objects ( this could be done in for in )
    let mut rec_0 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 0,
        texture: graphics::Image::new(ctx, "/T568B/0.png").unwrap(),
    };
    state.static_objects.push(rec_0);

    let mut rec_1 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 1,
        texture: graphics::Image::new(ctx, "/T568B/1.png").unwrap(),
    };
    state.static_objects.push(rec_1);

    let mut rec_2 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 2,
        texture: graphics::Image::new(ctx, "/T568B/2.png").unwrap(),
    };
    state.static_objects.push(rec_2);

    let mut rec_3 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 3,
        texture: graphics::Image::new(ctx, "/T568B/3.png").unwrap(),
    };
    state.static_objects.push(rec_3);

    let mut rec_4 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 4,
        texture: graphics::Image::new(ctx, "/T568B/4.png").unwrap(),
    };
    state.static_objects.push(rec_4);

    let mut rec_5 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 5,
        texture: graphics::Image::new(ctx, "/T568B/5.png").unwrap(),
    };
    state.static_objects.push(rec_5);

    let mut rec_6 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 6,
        texture: graphics::Image::new(ctx, "/T568B/6.png").unwrap(),
    };
    state.static_objects.push(rec_6);

    let mut rec_7 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: color_transparent,
        rect_cord: Rect::new(0.0, location.y, 25.0, 450.0),
        object_id: 7,
        texture: graphics::Image::new(ctx, "/T568B/7.png").unwrap(),
    };
    state.static_objects.push(rec_7);

    for looping in 0..12 {
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
        button_image: graphics::Image::new(ctx, "/Reset.png").unwrap(),
        button_image_clicked: graphics::Image::new(ctx, "/Reset-clicked.png").unwrap(),
        rect_cord: Rect::new(30.0, 30.0, 250.0, 100.0),
        button_id: 0,
        clicked_frames: 119,
    };
    state.static_buttons.push(button_reset);

    let mut check_button = static_rect_button {
        color: color_transparent,
        button_image: graphics::Image::new(ctx, "/Check.png").unwrap(),
        button_image_clicked: graphics::Image::new(ctx, "/Check-clicked.png").unwrap(),
        rect_cord: Rect::new(30.0, 150.0, 235.0, 100.0),
        button_id: 1,
        clicked_frames: 0,
    };
    state.static_buttons.push(check_button);
}
