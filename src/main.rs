use std::ops::{Mul, Sub};
// Did this just to try out implementing Mul, Sub for F32_

#[derive(PartialEq, PartialOrd,Debug, Clone, Copy)]
struct F32_(f32);

impl Mul for F32_ {
    type Output = F32_;
    fn mul(self, rhs: Self) -> Self::Output {
        F32_(self.0 * rhs.0)
    }
    
}


impl Sub for F32_ {
    type Output = F32_;

    fn sub(self, rhs: Self) -> Self::Output {
        F32_(self.0 - rhs.0)
    }
    
}


struct Customer {
    name: String,
    surname:String,
    balance: F32_
 
 }
 
 struct Product {
    name:String,
    price: F32_,
    stock: F32_
  
 }


 
 // Market functions
 
 impl Customer {
 
     // Function for purchasing a product
 
     fn buy_product(&mut self, product: &mut Product, quantity: F32_) -> bool {
   

        if product.stock > quantity {
            product.stock = product.stock.clone() - quantity;
            self.balance =  self.balance.clone() - (product.price.clone() * quantity);

            true
        }else {

            false
        }
 
         //If the product is available in sufficient quantity in stock and the customer has enough balance to buy it, purchase the product and update the customer's balance. Return true if the purchase is successful, false otherwise.
 
        
 
     }
 
 }
 
 // Main program
 
 fn main() {
 
    
 
     let mut customer1 = Customer {
        name: "Cust1".to_string(),
        surname: "Sur1".to_string(),
        balance: F32_(100.0),

 
     };
 
    
 
     let mut customer2 = Customer {
 
        name: "Cust2".to_string(),
        surname: "Sur2".to_string(),
        balance: F32_(100.0),
 
     };
 
    
 
     let mut product = Product {
 
        name:"product1".to_string(),
        price: F32_(10.0),
        stock: F32_(10000.0),
     };
 
     // Customers' product purchase operations
 
     println!("Customer 1 is buying a product...");
 
     if customer1.buy_product(&mut product, F32_(10.0)) {
 
         println!("Customer 1 successfully purchased the product.");
 
     } else {
 
         println!("Customer 1 couldn't purchase the product.");
 
     }



 
     println!("Customer 2 is buying a product...");
 
     if customer2.buy_product(&mut product, F32_(12.0)) {
 
         println!("Customer 2 successfully purchased the product.");
 
     } else {
 
         println!("Customer 2 couldn't purchase the product.");
 
     }
 
 }