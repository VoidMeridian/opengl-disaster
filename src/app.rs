use glium::glutin::event_loop::EventLoop;
use glium::{glutin, Program, Surface, Frame, DepthTest};
use crate::shape::Shape;
use crate::shader::Shader;
use crate::camera::Camera;
use glium::glutin::event::{KeyboardInput, VirtualKeyCode};
use std::time::Instant;
use glium::glutin::dpi::PhysicalPosition;

const STEP: f32 = 100.0;
fn mat4(val: f32) -> glm::TMat4<f32> {
    glm::mat4(val, 0.0, 0.0, 0.0,
                0.0, val, 0.0, 0.0,
              0.0, 0.0, val, 0.0,
                    0.0, 0.0, 0.0, val)
}
#[derive(Debug)]
pub struct App {
    pub display: glium::Display,
    pub program: Program,
    pub camera: Camera,
    start_time: Instant,
    last_frame: f32,
    current_frame: f32,
    pub mouse_pos: PhysicalPosition<f32>,
    x_bound: bool
}
impl App {

    pub fn new<T: Into<String>>(ev: &EventLoop<()>, title: T) -> App {
        let wb = glutin::window::WindowBuilder::new().with_title(title);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, ev).unwrap();
        let default_shader = Shader::default();
        let program = glium::Program::from_source(&display, &default_shader.get_vertex(), &default_shader.get_fragment(), None).unwrap();
        let viewport = display.get_framebuffer_dimensions();
        let time = std::time::Instant::now();
        //display.gl_window().window().set_cursor_visible(false);
        display.gl_window().window().set_cursor_grab(true);
        display.gl_window().window().set_cursor_position(PhysicalPosition::new(viewport.0 as f32/2 as f32, viewport.1 as f32/2 as f32));
        App {
            display,
            program,
            camera: Camera::new(viewport),
            start_time: time,
            current_frame: 0.0,
            last_frame: 0.0,
            x_bound: false,
            mouse_pos: PhysicalPosition::new(viewport.0 as f32/2 as f32, viewport.1 as f32/2 as f32),
        }
    }
    pub fn set_shaders(&mut self, shader: Shader) {
        let program = glium::Program::from_source(&self.display, &shader.get_vertex(), &shader.get_fragment(), None).unwrap();
        self.program = program;

    }

    pub fn draw(&self, shape: Shape, target: &mut Frame) {
        let light_color: [f32;3] = [1.0, 1.0, 1.0];
        let light_pos: [f32;3] = [3.0, 0.0, 0.0];
        let uniforms = uniform! {
            model: *shape.model_matrix().as_ref(),
            view: *self.camera.view().as_ref(),
            projection: *self.camera.projection().as_ref(),
            lightColor: light_color,
            lightPos: light_pos,
            objectColor: shape.get_color(),

        };
        let params = glium::draw_parameters::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: false,
                .. Default::default()
            },
            .. Default::default()

        };
        target.draw(shape.get_vbuffer(), shape.get_ibuffer(), &self.program, &uniforms, &params).unwrap();
    }
    pub fn render(&mut self) {
        self.current_frame = self.start_time.elapsed().as_secs_f32();
        self.camera.set_time(self.current_frame - self.last_frame);
        self.last_frame = self.current_frame;
        let mut target = self.display.draw();
        target.clear_color(100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0);
        let mut model = glm::translate(&mat4(1.0), &glm::vec3(1.0, 0.0, 0.0));
        self.draw(Shape::square(&self.display, [1.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], model), &mut target);
        let cube = glm::translate(&mat4(1.0), &glm::vec3(0.0, -1.0, 0.0));
        self.draw(Shape::cube(&self.display, [0.5, 0.5, 0.5], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], cube), &mut target);
        target.finish().unwrap();
    }
    pub fn keyboard_input(&mut self, input: KeyboardInput) -> glutin::event_loop::ControlFlow {
        let mut cf = glutin::event_loop::ControlFlow::Poll;
        if let Some(key) = input.virtual_keycode {
            match key {
                VirtualKeyCode::Up => {
                    self.camera.forward(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Down => {
                    self.camera.backward(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Left => {
                    self.camera.left(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Right => {
                    self.camera.right(STEP);
                    cf = glutin::event_loop::ControlFlow::Poll;
                }
                VirtualKeyCode::Escape => {
                    cf = glutin::event_loop::ControlFlow::Exit;
                }
                _ => {}
            }
        }
        cf
    }
    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) -> glutin::event_loop::ControlFlow {
        //println!("Got input");
        let mut xoff = position.x as f32 - self.mouse_pos.x;
        let mut yoff = self.mouse_pos.y - position.y as f32;
        if self.x_bound {
            xoff = 0.0;
            self.x_bound = false;
        }
        self.mouse_pos = position.cast();
        let sensitivity = 0.05;
        xoff *= sensitivity;
        yoff *= sensitivity;
        //println!("{} {}", xoff, yoff);
        self.camera.pitch(yoff);
        self.camera.yaw(xoff);
        if self.camera.pitch > 89.0 {
            self.camera.pitch = 89.0;
        }
        if self.camera.pitch < -89.0 {
            self.camera.pitch = -89.0;
        }
        self.camera.transform();
        self.check_mouse_x();
        glutin::event_loop::ControlFlow::Poll
    }
    fn check_mouse_x(&mut self) {
        let (x, _) = self.display.get_framebuffer_dimensions();
        let mut lower_x = x / 8;
        let mut upper_x = (x / 2) + lower_x;
        if self.mouse_pos.x >= (upper_x) as f32 && !self.x_bound {
            self.mouse_pos.x = lower_x as f32;
            self.x_bound = true;
            self.display.gl_window().window().set_cursor_position(self.mouse_pos);
        }
        if self.mouse_pos.x <= (lower_x) as f32 && !self.x_bound {
            self.mouse_pos.x = upper_x as f32;
            self.x_bound = true;
            self.display.gl_window().window().set_cursor_position(self.mouse_pos);
        }
    }
}