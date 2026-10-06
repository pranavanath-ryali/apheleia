pub struct SlotMap<T> {
    slots: Vec<Option<T>>,
    index_offset: u8,
}
impl<T> SlotMap<T> {
    pub fn new(index_offset: u8) -> Self {
        Self {
            slots: Default::default(),
            index_offset,
        }
    }
}

impl<T> SlotMap<T> {
    pub fn insert(&mut self, index: u32, value: T) {
        while (self.slots.len() as u32) <= (index - self.index_offset as u32) {
            self.slots.push(None);
        }
        self.slots[index as usize - self.index_offset as usize] = Some(value);
    }

    pub fn remove(&mut self, index: u32) {
        if let Some(slot) = self.slots.get_mut(index as usize) {
            *slot = None;
        }
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        if let Some(data) = self
            .slots
            .get(index as usize - self.index_offset as usize)?
        {
            return Some(data);
        }
        None
    }

    pub fn get_mut(&mut self, index: u32) -> Option<&mut T> {
        if let Some(data) = self
            .slots
            .get_mut(index as usize - self.index_offset as usize)?
        {
            return Some(data);
        }
        None
    }
}

#[cfg(test)]
mod test_slot_map {
    use crate::sparse_set::SlotMap;

    #[derive(PartialEq, Debug)]
    struct Data {
        x: u32,
    }

    #[test]
    fn test_slot_map_insertion_and_removal() {
        let mut slotmap: SlotMap<Data> = SlotMap::new(1);

        slotmap.insert(1, Data { x: 10 });
        slotmap.insert(4, Data { x: 40 });

        assert!(slotmap.get(2).is_none());
        assert!(slotmap.get(3).is_none());

        assert_eq!(*slotmap.get(1).unwrap(), Data { x: 10 });
        assert_eq!(*slotmap.get(4).unwrap(), Data { x: 40 });
    }
}
