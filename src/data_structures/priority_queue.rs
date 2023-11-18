pub struct PriorityQueue<K, V>
where
    K: Ord,
    V: Copy,
{
    entries: Vec<Entry<K, V>>,
}

struct Entry<K, V> {
    priority: K,
    item: V,
}

impl<K, V> PriorityQueue<K, V>
where
    K: Ord,
    V: Copy,
{
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, priority: K, item: V) {
        self.entries.push(Entry { priority, item });
        self.entries.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn remove_min(&mut self) -> V {
        self.entries.remove(0).item
    }

    pub fn min(&self) -> V {
        if let Some(item) = self.entries.first() {
            return item.item.clone();
        }
        panic!("Priority queue is empty");
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_priority_queue() {
        let pq = PriorityQueue::<i32, i32>::new();
        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn single_item_priority_queue() {
        let mut pq = PriorityQueue::<i32, i32>::new();
        pq.insert(0, 15);
        assert_eq!(pq.is_empty(), false);
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.min(), 15);
        assert_eq!(pq.is_empty(), false);
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.remove_min(), 15);
        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.size(), 0);
    }
}