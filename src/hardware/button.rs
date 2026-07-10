use ch32_hal::{
    Peri,
    gpio::{AnyPin, Input, Level, Pull},
};

use crate::utils::{Debounce, DebounceResult};

pub struct Button {
    input: Input<'static>,
    debounce: Debounce<Level>,
}

pub struct ButtonState(DebounceResult<Level>);

impl Button {
    pub fn new_pulldown(pin: Peri<'static, AnyPin>) -> Self {
        let input = Input::new(pin, Pull::Down);

        Self {
            debounce: Debounce::new(input.get_level()),
            input,
        }
    }

    pub fn poll(&mut self) -> ButtonState {
        let level = self.input.get_level();
        let result = self.debounce.tick(level);
        ButtonState(result)
    }
}

#[allow(unused)]
impl ButtonState {
    pub fn is_up(&self) -> bool {
        *self.0.current() == Level::Low
    }

    pub fn is_down(&self) -> bool {
        *self.0.current() == Level::High
    }

    pub fn is_press(&self) -> bool {
        matches!(self.0, DebounceResult::Changed(_, Level::High))
    }

    pub fn is_release(&self) -> bool {
        matches!(self.0, DebounceResult::Changed(_, Level::Low))
    }
}
