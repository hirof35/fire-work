use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// 乱数生成の簡易関数
fn random(limit: f64) -> f64 {
    js_sys::Math::random() * limit
}

// 花火の状態
enum FireworkState {
    Rising,
    Exploded { count: u32, scale: f64 },
}

struct Firework {
    x: f64,
    y: f64,
    x_speed: f64,
    y_speed: f64,
    radius: f64,
    color: String,
    state: FireworkState,
}

impl Firework {
    fn new(color: String) -> Self {
        let mut firework = Self {
            x: 0.0,
            y: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radius: 0.0,
            color,
            state: FireworkState::Rising,
        };
        firework.initialize();
        firework
    }

    fn initialize(&mut self) {
        self.x = random(800.0);
        self.y = 600.0 + random(20.0);
        self.x_speed = -3.0 + random(6.0);
        self.y_speed = -3.0 - random(8.0);
        self.radius = random(60.0) + 60.0;
        self.state = FireworkState::Rising;
    }

    fn update(&mut self) {
        self.x += self.x_speed;
        self.y += self.y_speed;

        self.y_speed += 0.1;
        self.x_speed /= 1.01;

        if let FireworkState::Rising = self.state {
            if self.y_speed >= -1.0 {
                self.state = FireworkState::Exploded { count: 0, scale: 0.0 };
            }
        }
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str(&self.color));

        match &mut self.state {
            FireworkState::Rising => {
                ctx.begin_path();
                let _ = ctx.arc(self.x, self.y, 4.0, 0.0, std::f64::consts::TAU);
                ctx.fill();
            }
            FireworkState::Exploded { count, scale } => {
                *count += 1;
                
                for _ in 0..4 {
                    *scale += 0.06 / (*count as f64);
                    let rad = self.radius * (*scale);
                    
                    let mut angle = 0.0;
                    while angle < std::f64::consts::TAU {
                        let dx = angle.cos() * rad;
                        let dy = angle.sin() * rad;

                        ctx.begin_path();
                        let _ = ctx.arc(self.x + dx, self.y + dy, 2.0, 0.0, std::f64::consts::TAU);
                        ctx.fill();

                        angle += 0.6;
                    }
                }

                if *count > 30 {
                    self.initialize();
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let canvas = document
        .get_element_by_id("field")
        .expect("should have #field")
        .dyn_into::<HtmlCanvasElement>()?;
        
    let ctx = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let colors = vec![
        "#ff0000", "#ffff00", "#ffffff", "#ff00ff", "#00ff00", "#7F7FFF", "#00ffff",
    ];

    let mut fireworks = Vec::new();
    for i in 0..14 {
        fireworks.push(Firework::new(colors[i % colors.len()].to_string()));
    }

    // ⬇️ ここに型アノテーションを明示してエラーを解決
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let fireworks = Rc::new(RefCell::new(fireworks));

    *g.borrow_mut() = Some(Closure::new(move || {
        ctx.set_global_alpha(0.2);
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill_rect(0.0, 0.0, 800.0, 600.0);
        
        ctx.set_global_alpha(1.0);

        let mut fires = fireworks.borrow_mut();
        for firework in fires.iter_mut() {
            firework.update();
            firework.draw(&ctx);
        }

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                (f.borrow()).as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .unwrap();
    }));

    window
        .request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )?;

    Ok(())
}