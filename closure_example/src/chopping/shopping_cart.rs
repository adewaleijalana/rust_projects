use crate::chopping::super_market_item::SupermarketItem;

#[derive(Debug, Clone)]
pub struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    pub fn new(items: Vec<SupermarketItem>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> Vec<SupermarketItem>{
        self.items.clone()
    }

    pub fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let _ = &self.items.iter_mut().for_each(|item| {
            operation(item);
        });
    }

    pub fn checkout<F>(self, operation: F)
    where
        F: FnOnce(ShoppingCart),
    {
        operation(self);
    }
}
