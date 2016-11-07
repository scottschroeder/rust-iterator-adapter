
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


struct PointBuilder<I> {
    iter: I,
}

impl<I: Iterator<Item = T>, T> Iterator for PointBuilder<I> {
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Point<T>> {
        let x = match self.iter.next() {
            Some(x) => x,
            None => return None,
        };
        let y = match self.iter.next() {
            Some(y) => y,
            None => return None,
        };
        Some(Point { x: x, y: y })
    }
}


trait Pointable<T>: Iterator {
    fn collect_points(self) -> PointBuilder<Self>
        where Self: Iterator<Item = T>,
              Self: Sized
    {
        PointBuilder { iter: self }
    }
}

impl<I, T> Pointable<T> for I where I: Iterator<Item = T> {}

fn main() {
    let numbers: Vec<f64> = vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64];
    let points: Vec<Point<f64>> = numbers.into_iter().collect_points().collect();
    println!("{:?}", points);

    let words: Vec<String> = vec![
        "cat".to_string(),
        "dog".to_string(),
        "mouse".to_string(),
        "bird".to_string(),
    ];
    let enemies: Vec<Point<String>> = words.into_iter().collect_points().collect();
    println!("{:?}", enemies);
}
