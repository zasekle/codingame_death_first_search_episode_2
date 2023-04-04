use std::collections::HashSet;
use std::collections::HashMap;
use std::process::id;
use core::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Link {
    index: i32,
    severed: bool,
}

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
struct Node {
    links: Vec<Link>,
    gateway: bool,
    distance: i32,
    cuts: usize,
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
    let mut nodes = vec![
        Node { links: vec![Link { index: 2, severed: false }, Link { index: 1, severed: false }, Link { index: 3, severed: false }], gateway: false, distance: -1, cuts: 0 }, //0
        Node { links: vec![Link { index: 7, severed: false }, Link { index: 0, severed: false }, Link { index: 3, severed: false }], gateway: false, distance: -1, cuts: 0 }, //1
        Node { links: vec![Link { index: 6, severed: false }, Link { index: 0, severed: false }, Link { index: 3, severed: false }], gateway: false, distance: -1, cuts: 0 }, //2
        Node { links: vec![Link { index: 7, severed: false }, Link { index: 6, severed: false }, Link { index: 5, severed: false }, Link { index: 4, severed: false }, Link { index: 0, severed: false }, Link { index: 1, severed: false }, Link { index: 2, severed: false }], gateway: false, distance: -1, cuts: 0 },
        Node { links: vec![Link { index: 3, severed: false }, Link { index: 7, severed: false }], gateway: true, distance: -1, cuts: 0 }, //4
        Node { links: vec![Link { index: 3, severed: false }, Link { index: 6, severed: false }], gateway: true, distance: -1, cuts: 0 }, //5
        Node { links: vec![Link { index: 2, severed: false }, Link { index: 3, severed: false }, Link { index: 5, severed: false }], gateway: false, distance: -1, cuts: 0 }, //6
        Node { links: vec![Link { index: 3, severed: false }, Link { index: 1, severed: false }, Link { index: 4, severed: false }], gateway: false, distance: -1, cuts: 0 }, //7
    ];

    let enemy_node = 0;

    find_distance_to_nodes(
        &mut nodes,
        enemy_node,
    );

    for i in 0..nodes.len() {
        if nodes[i].gateway {
            let links = nodes[i].links.clone();
            for link in links.iter() {
                if !link.severed {
                    let link_index = link.index as usize;
                    nodes[link_index].cuts += 1;
                }
            }
        }
    }

    let mut smallest_cuts_to_distance = 999999999;
    let mut node_index = 0;
    let mut gateway_index = 0;
    for (i, node) in nodes.iter().enumerate() {
        let cuts_to_distance = node.distance - node.cuts as i32;
        if !node.gateway && node.cuts > 0 {
            if cuts_to_distance < smallest_cuts_to_distance {
                smallest_cuts_to_distance = cuts_to_distance;
                node_index = i;
            }
        }
    }

    let cloned_links = nodes[node_index].links.clone();
    for link in cloned_links.iter() {
        if !link.severed {
            let link_index = link.index as usize;
            if nodes[link_index].gateway {
                gateway_index = link_index;
                break;
            }
        }
    }

    for (i, node) in nodes.iter_mut().enumerate() {
        if i == node_index || i == gateway_index {
            for link in node.links.iter_mut() {
                if link.index as usize == node_index
                    || link.index as usize == gateway_index {
                    link.severed = true;
                }
            }
        }
    }

    //TODO: might be better to share links?
    //TODO: there is the problem that
    // it doesn't recognize when it has a 'gap' and when to use that gap, if a path constantly take moves for example, it doesn't know

    let mut output_str = String::new();

    for (i, n) in nodes.iter().enumerate() {
        if i != 0 {
            output_str.push(' ');
        }
        output_str.push_str(format!("<{i} gateway: {} distance: {} cuts: {} distance-cuts: {}>", n.gateway, n.distance, n.cuts, n.distance - n.cuts as i32).as_str());
    }
    // Example: 3 4 are the indices of the nodes you wish to sever the link between
    println!("{:?}", output_str);
    println!("{node_index} {gateway_index}")
}

fn find_distance_to_nodes(
    nodes: &mut Vec::<Node>,
    enemy_node: usize,
) {
    for node in nodes.iter_mut() {
        node.distance = -1;
        node.cuts = 0;
    }

    let mut current_idx = HashSet::<usize>::from([enemy_node]);
    let mut distance = 0;
    while !current_idx.is_empty() {
        let next_idx = current_idx;
        current_idx = HashSet::new();

        for idx in next_idx {
            if nodes[idx].distance == -1 {
                nodes[idx].distance = distance;
                for link in nodes[idx].links.iter() {
                    if !link.severed {
                        current_idx.insert(link.index as usize);
                    }
                }
            }
        }

        distance += 1;
    }
}
