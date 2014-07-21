use std::sync::Arc;
use std::fmt;

#[deriving(Clone,Show)]
pub struct ItemData /*{
}*/;

#[deriving(Clone)]
pub struct Item {
    data: ItemData,
    contents: Arc<Vec<Item>>,
}

impl fmt::Show for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item data={} contents={}", self.data, self.contents.deref())
    }
}

impl Item {
    pub fn make(data: ItemData) -> Item {
        let contents = Arc::new(Vec::new());
        Item { data: data, contents: contents }
    }

    pub fn clone_add<'a>(&'a self, item: Item) -> Item {
        let mut contents = self.contents.clone();
        contents.make_unique().push(item);
        Item { data: self.data, contents: contents }
    }
}
