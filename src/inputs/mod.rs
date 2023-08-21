use std::{collections::HashMap, error::Error, fmt::Display, hash::Hash};

use sdl2::{event::Event, EventPump};

pub type Scancode = sdl2::keyboard::Scancode;
pub type GamepadButton = sdl2::controller::Button;
pub type GamepadAxis = sdl2::controller::Axis;

pub trait InputScheme: Hash + Eq + std::fmt::Debug + Display + Copy {}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Control {
    Button(ButtonControl),
    Axis(AxisControl),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ButtonControl {
    Keyboard(Scancode),
    Gamepad(GamepadButton),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AxisControl {
    Keyboard(Scancode, Scancode),
    Gamepad(GamepadAxis),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Down,
    Up,
}

pub enum Input {
    Button(ButtonInputData),
    Axis(AxisInputData),
}

pub struct ButtonInputData {
    pub value: ButtonState,
    pub changed_this_frame: bool,
    controls: Vec<ButtonControl>,
}

pub struct AxisInputData {
    value: f64,
    controls: Vec<AxisControl>,
}

#[derive(Debug)]
pub enum InputRegistrationError<T>
where
    T: InputScheme,
{
    ControlBusy(T),
}

pub struct InputsPipeline<T>
where
    T: InputScheme,
{
    event_pump: EventPump,
    controls_input: HashMap<Control, T>,
    inputs: HashMap<T, Input>,
}

impl<T> Error for InputRegistrationError<T> where T: InputScheme {}

impl<T> Display for InputRegistrationError<T>
where
    T: InputScheme,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputRegistrationError::ControlBusy(id) => {
                write!(f, "Control already assigned to {}", id)
            }
        }
    }
}

impl<T> InputsPipeline<T>
where
    T: InputScheme,
{
    pub(crate) fn new(event_pump: EventPump) -> Self {
        let controller_inputs = HashMap::new();
        let inputs = HashMap::new();

        InputsPipeline {
            event_pump,
            controls_input: controller_inputs,
            inputs,
        }
    }

    pub fn register(
        &mut self,
        input_id: T,
        input: Input,
        controls: &Vec<Control>,
    ) -> Result<(), InputRegistrationError<T>> {
        for c in controls {
            if let Some(i) = self.controls_input.get(c) {
                return Err(InputRegistrationError::ControlBusy(*i));
            }
        }

        Ok(())
    }

    pub fn read(&self, key: &T) -> Option<&Input> {
        self.inputs.get(key)
    }

    pub(crate) fn process_events(&mut self) {
        let inputs: HashMap<Control, Event> = self
            .event_pump
            .poll_iter()
            .filter_map(|e| match e {
                Event::KeyDown {
                    scancode, repeat, ..
                } => Some((Control::Button(ButtonControl::Keyboard(scancode?)), e)),
                Event::KeyUp {
                    scancode, repeat, ..
                } => Some((Control::Button(ButtonControl::Keyboard(scancode?)), e)),
                _ => None,
            })
            .collect();

        for i in self.inputs.values_mut() {
            match i {
                Input::Axis(a) => {
                    for control in &a.controls {
                        match control {
                            AxisControl::Gamepad(_) => {
                                if let Some(event) = inputs.get(&Control::Axis(*control)) {
                                    a.value = match event {
                                        Event::ControllerAxisMotion { value, .. } => {
                                            *value as f64 / i16::MAX as f64
                                        }
                                        _ => 0.,
                                    }
                                }
                            },
                            AxisControl::Keyboard(min, max) => {
                                let mut v = 0.;
                                if let Some(event) =
                                    inputs.get(&Control::Button(ButtonControl::Keyboard(*min)))
                                {
                                    match event {
                                        Event::KeyDown { .. } => v -= 1.,
                                        _ => {}
                                    }
                                }

                                if let Some(event) =
                                    inputs.get(&Control::Button(ButtonControl::Keyboard(*max)))
                                {
                                    match event {
                                        Event::KeyDown { .. } => v += 1.,
                                        _ => {}
                                    }
                                }

                                a.value = v;
                            }
                        }
                    }
                }
                Input::Button(b) => {
                    for control in &b.controls {
                        match control {
                            ButtonControl::Gamepad(_) => {
                                if let Some(event) = inputs.get(&Control::Button(*control)) {
                                    match event {
                                        Event::ControllerButtonDown { .. } => {
                                            b.changed_this_frame = b.value != ButtonState::Down;
                                            b.value = ButtonState::Down;
                                        }
                                        Event::ControllerButtonUp { .. } => {
                                            b.changed_this_frame = b.value != ButtonState::Up;
                                            b.value = ButtonState::Up;
                                        }
                                        _ => {
                                            b.changed_this_frame = false;
                                            b.value = ButtonState::Up;
                                        }
                                    };
                                }
                            }
                            ButtonControl::Keyboard(_) => {
                                if let Some(event) = inputs.get(&Control::Button(*control)) {
                                    match event {
                                        Event::KeyDown { .. } => {
                                            b.changed_this_frame = b.value != ButtonState::Down;
                                            b.value = ButtonState::Down;
                                        }
                                        Event::KeyUp { .. } => {
                                            b.changed_this_frame = b.value != ButtonState::Up;
                                            b.value = ButtonState::Up;
                                        }
                                        _ => {
                                            b.changed_this_frame = false;
                                            b.value = ButtonState::Up;
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
