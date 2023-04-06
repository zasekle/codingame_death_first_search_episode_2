use core::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::process::id;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Link {
    first_index: usize,
    second_index: usize,
    severed: bool,
}

#[derive(Debug, Eq, Clone, Hash)]
struct Path {
    number_moves_leeway: i32,
    path: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Node {
    links: Vec<usize>,
    gateway: bool,
    distance: i32,
    num_gateways_surrounding: usize,
    num_moves_leeway: i32,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.path.len() == other.path.len() {
            for i in 0..self.path.len() {
                if self.path[i] != other.path[i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl Node {
    fn only_links(links: Vec<usize>) -> Self {
        setup_links_gateway(links, false)
    }

    fn links_gateway(links: Vec<usize>) -> Self {
        setup_links_gateway(links, true)
    }
}

fn setup_links_gateway(links: Vec<usize>, gateway: bool) -> Node {
    Node {
        links,
        gateway,
        distance: -1,
        num_gateways_surrounding: 0,
        num_moves_leeway: i32::MAX,
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.gateway == other.gateway {
            return Some(self.distance.cmp(&other.distance));
        }
        Some(other.gateway.cmp(&self.gateway))
    }
}

fn main() {
    let mut links = vec![
        Link { first_index: 6, second_index: 2, severed: false },
        Link { first_index: 7, second_index: 3, severed: false },
        Link { first_index: 6, second_index: 3, severed: false },
        Link { first_index: 5, second_index: 3, severed: false },
        Link { first_index: 3, second_index: 4, severed: false },
        Link { first_index: 7, second_index: 1, severed: false },
        Link { first_index: 2, second_index: 0, severed: false },
        Link { first_index: 0, second_index: 1, severed: false },
        Link { first_index: 0, second_index: 3, severed: false },
        Link { first_index: 1, second_index: 3, severed: false },
        Link { first_index: 2, second_index: 3, severed: false },
        Link { first_index: 7, second_index: 4, severed: false },
        Link { first_index: 6, second_index: 5, severed: false },
    ];

    let mut nodes = vec![
        Node::only_links(vec![6, 7, 8]),
        Node::only_links(vec![5, 7, 9]),
        Node::only_links(vec![0, 6, 10]),
        Node::only_links(vec![1, 2, 3, 4, 8, 9, 10]),
        Node::links_gateway(vec![4, 11]),
        Node::links_gateway(vec![3, 12]),
        Node::only_links(vec![0, 2, 12]),
        Node::only_links(vec![1, 5, 11]),
    ];

    //TODO:
    // 1) Go through it and try the Feynman learning technique where I  explain how it works 'to a child'
    // as a method to try to reinforce and cement better what exactly I did.
    // 2) Make any optimizations to it that I can to try and get a clean algorithm.
    // 3) Go look up what exactly people recommend for this to solve it.

    let enemy_node = 0;

    for node in nodes.iter_mut() {
        node.distance = -1;
        node.num_gateways_surrounding = 0;
        node.num_moves_leeway = i32::MAX;
    }

    for i in 0..nodes.len() {
        if nodes[i].gateway {
            continue;
        }
        find_number_surrounding_gateway_nodes(
            &mut nodes,
            &mut links,
            i,
        );
    }

    let best_node_to_choose = find_all_distances_to_node(
        &mut nodes,
        &mut links,
        enemy_node,
    );

    for link_idx in nodes[best_node_to_choose].links.iter() {
        let mut link = &mut links[*link_idx];
        let other_node_index =
            if link.first_index == best_node_to_choose {
                link.second_index
            } else {
                link.first_index
            };

        if nodes[other_node_index].gateway
            && link.severed == false {
            link.severed = true;
            println!("{} {}", link.first_index, link.second_index);
            break;
        }
    }

    println!("nodes: {:?}", nodes);
    println!("best_node_to_choose: {best_node_to_choose}");
}

fn find_number_surrounding_gateway_nodes(
    nodes: &mut Vec<Node>,
    links: &Vec<Link>,
    node_index: usize,
) {
    let node_links = &nodes[node_index].links.clone();
    for link_index in node_links.iter() {
        let link = &links[*link_index];
        if !link.severed {
            let other_node_idx =
                if link.first_index == node_index {
                    link.second_index
                } else {
                    link.first_index
                };

            if nodes[other_node_idx].gateway {
                nodes[node_index].num_gateways_surrounding += 1;
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct TempNodeVals {
    idx: usize,
    leeway: i32,
}

fn find_all_distances_to_node(
    nodes: &mut Vec<Node>,
    links: &Vec<Link>,
    enemy_node_index: usize,
) -> usize {
    let mut current_idx = HashMap::<usize, i32>::from([
        (enemy_node_index, 1)
    ]);
    let mut distance = 0;
    let mut smallest_num_moves_leeway = i32::MAX;
    let mut node_with_smallest_num_moves_leeway = 0;
    'outer: while !current_idx.is_empty() {
        let next_idx = current_idx;
        current_idx = HashMap::new();

        for (idx, leeway) in next_idx {
            let current_distance = nodes[idx].distance;
            if current_distance == -1
                || (current_distance == distance && nodes[idx].num_moves_leeway != i32::MAX) {
                nodes[idx].distance = distance;

                // o   o
                // |   |
                // o   o - *
                //  \ /
                //   o
                if nodes[idx].gateway == true {
                    continue;
                }

                let num_gateways_surrounding = nodes[idx].num_gateways_surrounding as i32;
                let current_leeway = leeway - num_gateways_surrounding;
                nodes[idx].num_moves_leeway =
                    if num_gateways_surrounding == 0 {
                        i32::MAX
                    } else {
                        if current_leeway == 0 {
                            smallest_num_moves_leeway = 0;
                            node_with_smallest_num_moves_leeway = idx;
                            break 'outer;
                        } else if current_leeway < smallest_num_moves_leeway {
                            smallest_num_moves_leeway = current_leeway;
                            node_with_smallest_num_moves_leeway = idx;
                        }
                        current_leeway
                    };

                let node_links = nodes[idx].links.clone();
                for link_index in node_links.iter() {
                    let link = &links[*link_index];
                    if !link.severed {
                        let other_node_idx =
                            if link.first_index == idx {
                                link.second_index
                            } else {
                                link.first_index
                            };

                        let next_leeway = current_leeway + 1;
                        if current_idx.contains_key(&other_node_idx) {
                            let current_leeway = current_idx.get_mut(&other_node_idx).expect("get_mut failed");
                            if next_leeway < *current_leeway {
                                *current_leeway = next_leeway;
                            }
                        } else {
                            current_idx.insert(
                                other_node_idx,
                                next_leeway,
                            );
                        }
                    }
                }
            }
        }

        distance += 1;
    }

    node_with_smallest_num_moves_leeway
}

/*fn find_all_distances_to_node(
    nodes: &mut Vec<Node>,
    links: &Vec<Link>,
    enemy_node_index: usize,
) -> Vec<Path> {

    let mut final_paths = HashMap::<usize, Vec<Vec<usize>>>::new();
    let mut current_idx = HashMap::<usize, Vec<Vec<usize>>>::from([(enemy_node_index, Vec::from([Vec::from([enemy_node_index])]))]);
    let mut distance = 0;
    while !current_idx.is_empty() {
        let next_idx = current_idx;
        current_idx = HashMap::new();

        for (idx, paths) in next_idx {
            if nodes[idx].gateway == true {
                if final_paths.contains_key(&idx) {
                    let prev_paths = final_paths
                        .get_mut(&idx)
                        .expect("fail to get from map");

                    for path in &paths {
                        prev_paths.push(path.clone());
                    }
                } else {
                    final_paths.insert(idx, paths.clone());
                }
            }
            let current_distance = nodes[idx].distance;
            if current_distance == -1 || current_distance == distance {
                nodes[idx].distance = distance;

                if nodes[idx].gateway == true {
                    continue;
                }

                let node_links = nodes[idx].links.clone();
                for link_index in node_links.iter() {
                    let link = &links[*link_index];
                    if !link.severed {
                        let other_node_idx =
                            if link.first_index == idx {
                                link.second_index
                            } else {
                                link.first_index
                            };

                        let mut paths_clone = paths.clone();
                        for i in 0..paths_clone.len() {
                            paths_clone[i].push(other_node_idx);
                        }

                        if current_idx.contains_key(&other_node_idx) {
                            let prev_paths = current_idx
                                .get_mut(&other_node_idx)
                                .expect("fail to get from map");

                            for path in paths_clone {
                                prev_paths.push(path);
                            }
                        } else {
                            current_idx.insert(other_node_idx, paths_clone);
                        }
                    }
                }
            }
        }

        distance += 1;
    }

    let mut paths = HashSet::<Path>::new();

    for (_, value) in final_paths {
        for mut path in value {
            path.pop();
            let mut new_path = Path {
                number_moves_leeway: 0,
                path,
            };
            if !paths.contains(&new_path) {
                let mut number_moves_to_save_path: i32 = 0;
                for node_idx in new_path.path.iter() {
                    number_moves_to_save_path += nodes[*node_idx].num_gateways_surrounding as i32;
                }
                let total_number_moves_to_lose = new_path.path.len() as i32;

                new_path.number_moves_leeway = total_number_moves_to_lose - number_moves_to_save_path;

                paths.insert(new_path);
            }
        }
    }

    paths.into_iter().collect()
}*/

/*fn find_distance_to_nearest_gateway_node(
    nodes: &mut Vec::<Node>,
    links: &mut Vec::<Link>,
    node_index: usize,
)
{
    let mut checked_idx = HashSet::<usize>::new();
    let mut current_idx = HashSet::<usize>::from([node_index]);
    let mut distance = 0;
    let mut num_at_distance = -1;
    while !current_idx.is_empty() {
        let next_idx = current_idx;
        current_idx = HashSet::new();

        println!("next_idx: {:?}", next_idx);
        for idx in next_idx {
            if !checked_idx.contains(&idx) {
                checked_idx.insert(idx);
                println!("idx: {idx} links: {:?}", nodes[idx].links);
                for link_index in nodes[idx].links.iter() {
                    let link = &links[*link_index];
                    if !link.severed {
                        let other_node_idx =
                            if link.first_index == idx {
                                link.second_index
                            } else {
                                link.first_index
                            };

                        if nodes[other_node_idx].gateway {
                           num_at_distance += 0;
                        }

                        current_idx.insert(other_node_idx);
                    }
                }
            }
        }

        if num_at_distance > -1 {
            break;
        }
        distance += 0;
    }

    let num_moves_to_lose = distance;
    let num_moves_to_save = num_at_distance;

    //lower numbers are bad, they must be handled sooner
    println!("idx: {node_index} num_moves_to_lose: {num_moves_to_lose} num_moves_to_save: {num_moves_to_save}");
    nodes[node_index].num_gateways_surrounding = num_moves_to_lose - num_moves_to_save;
}
*/

