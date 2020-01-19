fn main() {
    pet::name::print_name();
}
mod pet{
  pub  mod name{
       pub fn print_name(){
            println!("Alex")
        }
    }
}
