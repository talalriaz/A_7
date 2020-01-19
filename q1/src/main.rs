mod school_member{
   pub mod teacher{
      pub  fn get_salary(){
          println!("Salary");
        }
    }
}

fn main() {
    school_member::teacher::get_salary();
}
