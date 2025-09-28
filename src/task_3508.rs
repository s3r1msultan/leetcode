use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

struct PacketList {
    items: Vec<i32>,
    ptr: usize,
}

impl PacketList {
    fn new() -> Self {
        Self {
            items: vec![],
            ptr: 0,
        }
    }

    fn forward(&mut self) {
        self.ptr += 1;
    }
}

struct Router {
    size: usize,
    packets_queue: VecDeque<Packet>,
    packets_set: HashSet<Packet>,
    destinations: HashMap<i32, PacketList>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            size: memory_limit as usize,
            packets_queue: VecDeque::new(),
            destinations: HashMap::new(),
            packets_set: HashSet::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet {
            source,
            destination,
            timestamp,
        };
        if self.packets_set.contains(&packet) {
            return false;
        }

        if self.packets_queue.len() == self.size {
            self.forward_packet();
        }

        self.packets_set.insert(packet.clone());
        self.packets_queue.push_back(packet.clone());
        self.destinations
            .entry(destination)
            .or_insert_with(PacketList::new)
            .items
            .push(timestamp);
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(packet) = self.packets_queue.pop_front() {
            self.packets_set.remove(&packet);
            self.destinations
                .entry(packet.destination)
                .or_insert_with(PacketList::new)
                .forward();
            vec![packet.source, packet.destination, packet.timestamp]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(packet_list) = self.destinations.get(&destination) {
            let i = Self::binary_search(&packet_list.items, packet_list.ptr, start_time, true);
            let j = Self::binary_search(&packet_list.items, packet_list.ptr, end_time, false);

            (j - i) as i32
        } else {
            0
        }
    }

    fn binary_search(
        timestamps: &Vec<i32>,
        mut start: usize,
        target: i32,
        is_inclusive: bool,
    ) -> usize {
        let mut end = timestamps.len();

        while start < end {
            let mid = start + (end - start) / 2;

            if timestamps[mid] == target {
                if is_inclusive {
                    end = mid;
                } else {
                    start = mid + 1;
                }
            } else if timestamps[mid] < target {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        end
    }
}
