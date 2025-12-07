use crate::customers::customer_order::CustomerOrder;

pub struct Customer{
  customer_id: u16,
  orders: Vec<CustomerOrder>
}