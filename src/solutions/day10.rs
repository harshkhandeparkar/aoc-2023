use crate::inputs::day10::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> i128 {
    let (graph, start) = get_input(custom_input);

    let mut distances = Graph::<i128> {
        map: graph.map.iter().map(|row| vec![-1; row.len()]).collect(),
        rows: graph.rows,
        cols: graph.cols,
    };

    distances.set(start, 0);

    // BFS
    let mut queue: Vec<Coord> = Vec::new();
    queue.push(start);

    // Keeping track of all the nodes in the loop
    let mut in_loop = Graph::<bool> {
        map: graph.map.iter().map(|row| vec![false; row.len()]).collect(),
        rows: graph.rows,
        cols: graph.cols,
    };

    in_loop.set(start, true);

    while let Some(u) = queue.pop() {
        let adj_nodes = graph.get_reachable_adj_nodes(u);

        for v in adj_nodes {
            let old_dist = distances.get(v).unwrap();
            let new_dist = distances.get(u).unwrap() + 1;

            if old_dist == -1 || new_dist < old_dist {
                queue.push(v);
                if old_dist == -1 {
                    in_loop.set(v, true);
                }
                distances.set(v, new_dist);
            }
        }
    }

    if part == 1 {
        *distances
            .map
            .iter()
            .map(|row| row.iter().max().unwrap_or(&0))
            .max()
            .unwrap()
    } else {
        let mut num_enclosed: i128 = 0;

        // Explore every node that is not in_loop
        // If you hit a wall, the nodes are outside: exclude the nodes
        // If no wall was hit, add the nodes to the list of enclosed nodes

        let mut nodes_under_consideration: Vec<(i32, i32)> = Vec::new();
        let mut visited = Graph {
            map: in_loop.map.clone(),
            rows: in_loop.rows,
            cols: in_loop.cols
        };

        for i in 0..(in_loop.rows as i32) {
            for j in 0..(in_loop.cols as i32) {
                let node = (i, j);

                if !in_loop.get(node).unwrap() && !visited.get(node).unwrap() {
                    // If not in loop and not visited explore it
                    let mut queue = vec![node];
                    let mut consider_nodes = true;

                    while let Some(node_) = queue.pop() {
                        if !visited.get(node_).unwrap() {
                            nodes_under_consideration.push(node_);
                            visited.set(node_, true);

                            let mut adj_nodes = graph.get_adj_nodes_diag(node_);

                            if adj_nodes.len() < 4 {
                                // Hit a wall
                                consider_nodes = false;
                            }
                            queue.append(&mut adj_nodes);
                        }
                    }

                    if consider_nodes {
                        println!("{:?}", nodes_under_consideration);
                        num_enclosed += nodes_under_consideration.len() as i128;
                    }
                    nodes_under_consideration.clear();
                }
            }
        }

        num_enclosed
    }
}

type Coord = (i32, i32);

struct Graph<T: Clone + Copy> {
    map: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

impl<T: Clone + Copy> Graph<T> {
    fn get(&self, (x, y): Coord) -> Option<T> {
        if x >= 0 && y >= 0 && x < self.rows as i32 && y < self.cols as i32 {
            Some(self.map[x as usize][y as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, (x, y): Coord) -> Option<&mut T> {
        if x >= 0 && y >= 0 && x < self.rows as i32 && y < self.cols as i32 {
            Some(&mut self.map[x as usize][y as usize])
        } else {
            None
        }
    }

    fn set(&mut self, (x, y): Coord, value: T) {
        if let Some(loc) = self.get_mut((x, y)) {
            *loc = value;
        }
    }

    fn get_adj_nodes(&self, (x, y): Coord) -> Vec<Coord> {
        let mut adj_nodes = Vec::new();

        for (i, j) in vec![(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Some(_) = self.get((x + i, y + j)) {
                adj_nodes.push((x + i, y + j));
            }
        }

        adj_nodes
    }

    fn get_adj_nodes_diag(&self, (x, y): Coord) -> Vec<Coord> {
        let mut adj_nodes = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if let Some(_) = self.get((x + i, y + j)) {
                    adj_nodes.push((x + i, y + j));
                }
            }
        }

        adj_nodes
    }
}

impl Graph<char> {
    fn get_reachable_adj_nodes(&self, coord: Coord) -> Vec<Coord> {
        let mut adj_nodes: Vec<Coord> = Vec::new();
        let tile = self.get(coord).unwrap();

        let (x, y) = coord;

        let adj_e = (x + 1, y);
        let adj_w = (x - 1, y);
        let adj_n = (x, y - 1);
        let adj_s = (x, y + 1);

        if let Some(adj_tile) = self.get(adj_e) {
            if ((adj_tile == '-' || adj_tile == '7' || adj_tile == 'J') && tile == 'S')
                || tile == 'F'
                || tile == '-'
                || tile == 'L'
            {
                adj_nodes.push(adj_e);
            }
        }

        if let Some(adj_tile) = self.get(adj_w) {
            if ((adj_tile == '-' || adj_tile == 'F' || adj_tile == 'L') && tile == 'S')
                || tile == 'J'
                || tile == '-'
                || tile == '7'
            {
                adj_nodes.push(adj_w);
            }
        }

        if let Some(adj_tile) = self.get(adj_s) {
            if ((adj_tile == '|' || adj_tile == 'L' || adj_tile == 'J') && tile == 'S')
                || tile == '|'
                || tile == 'F'
                || tile == '7'
            {
                adj_nodes.push(adj_s);
            }
        }

        if let Some(adj_tile) = self.get(adj_n) {
            if ((adj_tile == '|' || adj_tile == '7' || adj_tile == 'F') && tile == 'S')
                || tile == '|'
                || tile == 'J'
                || tile == 'L'
            {
                adj_nodes.push(adj_n);
            }
        }

        adj_nodes
    }
}

fn get_input(custom_input: Option<&str>) -> (Graph<char>, Coord) {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);
    let mut start = (0, 0);

    let map: Vec<Vec<char>> = raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // transpose to put the <-> direction on the x-axis
    let rows = map.len();
    let cols = map[0].len();

    let map: Vec<Vec<char>> = (0..cols)
        .map(|col| (0..rows).map(|row| map[row][col]).collect())
        .collect();

    for (x, row) in map.iter().enumerate() {
        for (y, &tile) in row.iter().enumerate() {
            if tile == 'S' {
                start = (x as i32, y as i32);
                break;
            }
        }
    }

    (
        Graph {
            map,
            rows: cols,
            cols: rows,
        },
        start,
    )
}
