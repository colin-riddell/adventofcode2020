use std::fs;

struct Grid {
    taboggan_map: Vec<Vec<char>>

}

impl Grid {

    fn get_pos(&self, x: i32, y: i32, sx: i32, sy:i32) -> char{
        let pos_x : i32 = sx  + x.abs();
        let pos_y : i32 = sy  + y.abs();

        println!("{} and {}", pos_x, pos_y);
        return self.taboggan_map[pos_y as usize][pos_x as usize];
    }

    pub fn new(lines: Vec<&str>) -> Self{ 
        let width = lines[0].len();
        let height = lines.len();

        let mut taboggan_map: Vec<Vec<char>> = vec![vec!['.'; width]; height];
        for (i, line) in lines.iter().enumerate() {
            let line_split: Vec<char>= line.chars().collect::<Vec<_>>();
            for (j, cell) in line_split.iter().enumerate() {
                print!("{}", cell);
                taboggan_map[i][j] = *cell;
            }
            println!("");
        }

        return Self {taboggan_map: taboggan_map}
    }

}

pub fn run() {
    let input = fs::read_to_string("day3.txt").expect("Something went wrong reading the file");
    let lines : Vec<&str> = input.split("\n").collect();

    let taboggan_map : Grid = Grid::new(lines);
    println!("{}",taboggan_map.get_pos(8, -1, 0, 0));

}

#[test]
fn get_point_from_grid_small() {
    let input: &str = "1234\n5678";
    /*
        1234
        5678
    */
    let lines : Vec<&str> = input.split("\n").collect();

    let taboggan_map : Grid = Grid::new(lines);
    println!("{}",taboggan_map.get_pos(3, -1, 0, 0));
    assert_eq!(taboggan_map.get_pos(3, -1, 0, 0), '8')
}

#[test]
fn get_point_from_grid_med() {
    let input: &str = "1234*vkmx3\n5678qazx~p";
    /*
        1234*vkmx3
        5678qazx~p
    */
    let lines : Vec<&str> = input.split("\n").collect();

    let taboggan_map : Grid = Grid::new(lines);
    println!("{}",taboggan_map.get_pos(3, -1, 0, 0));
    assert_eq!(taboggan_map.get_pos(7, -1, 0, 0), 'x')
}

#[test]
fn get_point_from_grid_large() {
    let input: &str = "1234*vkmx3paseufnsGPFIU\n5678qazx~pQ9873RY9Q73RD\n123456&*(@£$%^&*@£$%^&)\nPOIN08N0909j124bcxbzxbA";
    /*
        1234*vkmx3paseufnsePFIU
        5678qazx~pQ9873RY9Q73RD
        123456&*(@£$%^&*@£$%^&)
        POIN08N0909j124bcxbzxbA
    */
    let lines : Vec<&str> = input.split("\n").collect();

    let taboggan_map : Grid = Grid::new(lines);

    assert_eq!(taboggan_map.get_pos(4, -1, 14, -1), '$')
}