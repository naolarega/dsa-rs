pub struct PriorityQueue<T>
where
    T: Copy,
{
    entries: Vec<Entry<T>>,
}

struct Entry<T> {
    priority: i32,
    item: T,
}

impl<T> PriorityQueue<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, priority: i32, item: T) {
        self.entries.push(Entry { priority, item });
        self.entries.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn remove_min(&mut self) -> Option<T> {
        if let Some(item) = self.entries.pop() {
            return Some(item.item);
        }
        None
    }

    pub fn min(&self) -> Option<T> {
        if let Some(item) = self.entries.last() {
            return Some(item.item.clone());
        }
        None
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}