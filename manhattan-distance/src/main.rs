/*
https://codingdojo.org/kata/manhattan-distance/

Manhattan distance is the distance between two points in a grid (like the grid-like street geography of the New York borough of Manhattan) calculated by only taking a vertical and/or horizontal path.

Write a function int manhattanDistance(Point, Point) that returns the Manhattan Distance between the two points.

Suggested tests
manhattanDistance( Point(1, 1), Point(1, 1) ) should returns 0
manhattanDistance( Point(5, 4), Point(3, 2) ) should returns 4
manhattanDistance( Point(1, 1), Point(0, 3) ) should returns 3
*/

struct Point {
    x: i32,
    y: i32,
}

fn manhattan_distance(p: Point, q : Point) -> i32 {
    return (p.x - q.x).abs() + (p.y - q.y).abs();
}

fn userinput(text : &str) -> Point {
    let mut valid = false;
    let mut coords: [i32; 2] = [0; 2];
    while !valid {
        println!("{}", text);
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to get input");
        let result = buffer.trim().split(", ");
        let mut i = 0;
        for coord in result {
            if i >= 2 {
                println!("Too many arguments !\n");
                valid = false; 
                break;
            }
            let coord_result = coord.parse::<i32>();
            match coord_result {
                Ok(ok) => { 
                    coords[i] = ok; 
                    valid = true; 
                }
                Err(e) => { 
                    println!("Wrong number format ! {}\n", e); 
                    valid = false; 
                }
            }
            i += 1;
        }
        if i <= 1 {
            println!("Too few arguments !\n");
            valid = false; 
        }
    }
    return Point { x: coords[0], y: coords[1] }
}

fn main() {
    // assert!(manhattan_distance(Point{x:1, y:1}, Point{x:1, y:1}) == 0);
    // assert!(manhattan_distance(Point{x:5, y:4}, Point{x:3, y:2}) == 4);
    // assert!(manhattan_distance(Point{x:1, y:1}, Point{x:0, y:3}) == 3);

    loop {
        println!("  >  The distance between the two points is {}.\n", 
            manhattan_distance(
                userinput("Please enter the first point's coordinates in <x, y> format :"),
                userinput("Please enter the second point's coordinates :")
            )
        );
    }
}
