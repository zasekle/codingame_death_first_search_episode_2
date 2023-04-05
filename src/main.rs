use std::collections::HashSet;
use std::collections::HashMap;
use std::process::id;
use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Link {
    first_index: usize,
    second_index: usize,
    severed: bool,
}

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Node {
    links: Vec<usize>,
    gateway: bool,
    distance: i32,
    num_gateway_nodes_on_way: i32,
    num_gateways_surrounding: usize,
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
        num_gateway_nodes_on_way: 0,
        num_gateways_surrounding: 0,
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

    let enemy_node = 0;

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

    //TODO: problems
    // 1) The same node can be counted multiple times, is this ok?
    //   *
    //  / \
    // o   o
    //  \ /
    //   *
    // 2) im not sure my leeway equation is broad enough for longer paths?

    //TODO: its about how many nodes are directly surrounding this one
    // the path length is the sum of these numbers so above would be a path of {0,1} and {0,1} so both paths are 'length' 1
    // then need to look at the 'longest' path first

    //TODO: probably run this one second, it now needs to store all paths, and it can save some processing
    // by having a running sum of the final number
    //TODO: it doesn't recognize when it has a 'gap' and when to use that gap, if a path constantly take moves for example, it doesn't know
    //A Path can be the sum of the 'num_gateways_surrounding' and a vector of node indexes.
    // Then I take the largest number relative to the path length.
    // assume 2 paths
    // *
    // |
    // 0--*  2 to lose; 2 to save
    // |
    // s
    // |
    // 0--*  2 to lose; 1 to save
    // |
    // 0--*  3 to lose; 1 to save
    // |
    // 0--*  4 to lose; 1 to save
    // a ratio of 1 (or lose - win == 0 to avoid division) means immediate attention required
    find_all_distances_to_node(
        &mut nodes,
        &mut links,
        enemy_node,
    );

    let mut output_str = String::new();

    for (i, n) in nodes.iter().enumerate() {
        if i != 0 {
            output_str.push(' ');
        }
        output_str.push_str(format!("<{i} gateway: {} distance: {} num_gateways_surrounding {}>", n.gateway, n.distance, n.num_gateways_surrounding).as_str());
    }
    // Example: 3 4 are the indices of the nodes you wish to sever the link between
    println!("{:?}", output_str);
    // println!("{node_index} {gateway_index}")
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

fn find_all_distances_to_node(
    nodes: &mut Vec<Node>,
    links: &Vec<Link>,
    enemy_node_index: usize,
) {

    //TODO: save path I suppose
    // need to make sure I handle multiple paths of same distance
    // need to look into 1 past the failing node on test cast #6
    // TODO: make sure no excess in paths

    for node in nodes.iter_mut() {
        node.distance = -1;
        node.num_gateway_nodes_on_way = 0;
    }

    let mut final_paths =  HashMap::<usize, Vec<Vec<usize>>>::new();
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
            if current_distance == -1  || current_distance == distance {
                nodes[idx].distance = distance;

                if nodes[idx].gateway == true {
                    continue;
                }

                let node_links= nodes[idx].links.clone();
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


    println!("final_paths: {:?}", final_paths);
}

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

