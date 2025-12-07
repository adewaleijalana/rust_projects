use crate::products::product::Product;
pub struct CustomerOrder {
    product: Product,
    quantity: u32,
    is_shipped: bool,
}

impl CustomerOrder {
    pub fn new(product: Product, quantity: u32, is_shipped: bool) -> Self {
        Self {
            product,
            quantity,
            is_shipped,
        }
    }
}
