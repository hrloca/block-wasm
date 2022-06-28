/*

let first = task.register(|ctx| {
    Some(Particles::Change(Point, Point))
});

let second = task.then(first, |ctx| {
    Some(Particles::Delete(Point, Point))
});

let third = task.then(second, |ctx| {
    Some(Particles::Fall(Point, Point))
});

task.then_from(third, second);

task.run(first);

relation {
    1: 2,
    2: 3,
    3: 2,
}

tasks [
   1: () => Particles::Change(Point, Point),
   2: () => Particles::Delete(Point, Point),
   3: () => Particles::Fall(Point, Point),
]

comp [a]

*/

use rand::prelude::*;

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    #[test]
    fn playground() {}
}
