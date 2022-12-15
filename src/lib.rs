#![feature(is_some_and)]

mod calc;
mod simulator;
mod states;
mod settings;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =  glam::Vec3A::splat(0.0);

        let mut res1 = result;

        res1.x = res1.x + 1.0;

        println!("{result} {res1}");

        // assert_eq!(result, res1);
    }
}
