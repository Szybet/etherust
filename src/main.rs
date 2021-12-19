//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::conf::FullscreenType;
use ggez::conf::NumSamples;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics::Mesh;
use ggez::graphics::{self, Color, DrawMode, Rect};
use ggez::input::mouse::position;
use ggez::input::mouse::*;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use glam::*;

pub struct static_rect_data {
    object_grabbed: bool,
    diffrence_y: f32,
    diffrence_x: f32,
    color: Color,
    rect_cord: Rect,
    object_id: i32,
}

struct MainState {
    static_objects: Vec<static_rect_data>,
    object_grabbed: i32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let pos = MainState {
            static_objects: Vec::new(),
            object_grabbed: 1000,
        };
        Ok(pos)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.5, 0.2, 0.3, 1.0].into());

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

        // 0
        let mut rec_0 = self.static_objects.iter_mut().nth(0).unwrap();
        draw_rec_func(ctx, rec_0);

        move_object(rect_cord_mouse, ctx, rec_0, &mut self.object_grabbed);

        // 1
        let mut rec_1 = self.static_objects.iter_mut().nth(1).unwrap();
        draw_rec_func(ctx, rec_1);

        move_object(rect_cord_mouse, ctx, rec_1, &mut self.object_grabbed);

        // 2
        let mut rec_2 = self.static_objects.iter_mut().nth(2).unwrap();
        draw_rec_func(ctx, rec_2);

        move_object(rect_cord_mouse, ctx, rec_2, &mut self.object_grabbed);

        // 3
        let mut rec_3 = self.static_objects.iter_mut().nth(3).unwrap();
        draw_rec_func(ctx, rec_3);

        move_object(rect_cord_mouse, ctx, rec_3, &mut self.object_grabbed);

        // 4
        let mut rec_4 = self.static_objects.iter_mut().nth(4).unwrap();
        draw_rec_func(ctx, rec_4);

        move_object(rect_cord_mouse, ctx, rec_4, &mut self.object_grabbed);

        manage_objects(
            rect_cord_mouse,
            ctx,
            &mut self.static_objects,
            &mut self.object_grabbed,
        );

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn move_object(
    rect_cord_mouse: Rect,
    ctx: &mut Context,
    static_rect_data: &mut static_rect_data,
    object_id: &mut i32,
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

pub fn draw_rec_func(ctx: &mut Context, static_rect_data: &mut static_rect_data) {
    let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        static_rect_data.rect_cord,
        static_rect_data.color,
    )
    .unwrap();

    graphics::draw(ctx, &rectangle, (Vec2::new(0.0, 0.0),)).unwrap();
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
        .window_mode(windowmode);
    let (ctx, event_loop) = cb.build()?;
    let mut state = MainState::new()?;

    // Here are defined objects
    let mut rec_0 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
        rect_cord: Rect::new(0.0, 0.0, 30.0, 30.0),
        object_id: 0,
    };
    state.static_objects.push(rec_0);

    let mut rec_1 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: Color::new(1.0, 0.0, 1.0, 1.0),
        rect_cord: Rect::new(200.0, 200.0, 60.0, 60.0),
        object_id: 1,
    };
    state.static_objects.push(rec_1);

    let mut rec_2 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: Color::new(0.3, 0.1, 1.0, 1.0),
        rect_cord: Rect::new(300.0, 300.0, 100.0, 100.0),
        object_id: 2,
    };
    state.static_objects.push(rec_2);

    let mut rec_3 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: Color::new(1.0, 0.3, 1.0, 1.0),
        rect_cord: Rect::new(400.0, 400.0, 120.0, 120.0),
        object_id: 3,
    };
    state.static_objects.push(rec_3);

    let mut rec_4 = static_rect_data {
        object_grabbed: false,
        diffrence_y: 0.0,
        diffrence_x: 0.0,
        color: Color::new(0.0, 1.0, 1.0, 1.0),
        rect_cord: Rect::new(500.0, 500.0, 150.0, 150.0),
        object_id: 4,
    };
    state.static_objects.push(rec_4);

    event::run(ctx, event_loop, state)
}
