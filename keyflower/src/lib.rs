use std::result::Result;

#[derive(PartialEq, Eq)]
pub enum Keeple {
    Blue,
    Red,
    Yellow,
    Green,
}

pub struct Bag {
    pub contents: Vec<Keeple>,
}

impl Bag {
    pub fn new() -> Self {
        // TODO: the startup bag has a specific quantity of keeples
        Self { contents: vec![] }
    }

    pub fn draw(&mut self) -> Option<Keeple> {
        if !self.contents.is_empty() {
            // pick a keeple and then remove it from the `Bag`
            // we do not need the precision of an f32, but it's needed for the calculation purposes
            #[allow(clippy::cast_precision_loss)]
            let index = (rand::random::<f32>() * self.contents.len() as f32).floor() as usize;
            let value = self.contents.remove(index);
            return Some(value);
        }

        None
    }

    pub fn return_keeple(&mut self, keeple: Keeple) -> Result<(), &str> {
        if keeple == Keeple::Green {
            return Err("You can't return a Green keeple!");
        }

        self.contents.push(keeple);
        Ok(())
    }
}

impl Default for Bag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
