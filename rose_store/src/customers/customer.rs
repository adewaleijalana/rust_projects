use crate::customers::customer_order::CustomerOrder;

#[derive(Debug)]
pub struct Customer {
    customer_id: u32,
    orders: Vec<CustomerOrder>,
}

impl Customer {
    pub fn new(customer_id: u32, orders: Vec<CustomerOrder>) -> Self {
        Self {
            customer_id,
            orders,
        }
    }

    pub fn get_customer_id(&self) -> u32{
        self.customer_id
    }
}
