use glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta};

pub struct EventHandler {
    pub mouse_buttons: [ElementState; 16],
    pub mouse_position: (i32, i32),
    pub mouse_position_prev: (i32, i32),
    pub keys: [ElementState; 256],
    pub closed: bool,
    pub scroll_amount: f32,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            mouse_buttons: [ElementState::Released; 16],
            mouse_position: (0, 0),
            mouse_position_prev: (0, 0),
            keys: [ElementState::Released; 256],
            closed: false,
            scroll_amount: 0f32,
        }
    }

    pub fn handle_event(&mut self, e: Event) {
        self.scroll_amount = 0f32;
        match e {
            Event::MouseMoved(x, y) => {
                self.mouse_position_prev = self.mouse_position;
                self.mouse_position = (x, y);
            },
            Event::MouseInput(state, button) => self.mouse_buttons[ match button {
                MouseButton::Left => 0,
                MouseButton::Right => 1,
                MouseButton::Middle => 2,
                MouseButton::Other(b) => b,
            } as usize] = state,
            Event::MouseWheel(delta, _) => { match delta {
                MouseScrollDelta::LineDelta(h, v) => self.scroll_amount = h + v,
                MouseScrollDelta::PixelDelta(h, v) => self.scroll_amount = h + v,
            }},
            Event::KeyboardInput(state, k, _) => self.keys[k as usize] = state,
            Event::Closed => self.closed = true,
            _ => {},
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn is_mouse_pressed(&self, mouse: u8) -> bool {
        match self.mouse_buttons[mouse as usize] {
            ElementState::Pressed => true,
            ElementState::Released => false,
        }
    }
}
