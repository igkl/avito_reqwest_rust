extern crate reqwest;

//use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
  //  let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
   //     .json()?;
   // println!("{:#?}", resp);

   let client = reqwest::Client::new();
   let mut resp = client.get("http://httpbin.org/").send()?;
   
   if resp.status().is_success() {
    println!("success!");
   } else if resp.status().is_server_error() {
    println!("server error!");
   } else {
    println!("Something else happened. Status: {:?}", resp.status());
   }
   
   let body = resp.text()?;

   println!("{}", body);
   
  Ok(())
}