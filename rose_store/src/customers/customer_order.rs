use crate::products::product::Product;

#[derive(Debug)]
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

    pub fn get_product(&self) -> Product {
        self.product.clone()
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }

    pub fn get_is_shipped(&self) -> bool {
        self.is_shipped
    }
}
