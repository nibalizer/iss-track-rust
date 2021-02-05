use std::io::Read;
use std::time::SystemTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Request {
    altitude: u32,
    datetime: u64,
    latitude: f32,
    longitude: f32,
    passes: u32,
}

#[derive(Deserialize, Debug)]
struct Rise {
    duration: u32,
    risetime: u64,
}

#[derive(Deserialize, Debug)]
struct Body {
    message: String,
    request: Request,
    response: Vec<Rise>,
}

fn main() {
    let params = [("lat", "44.986"), ("lon", "-93.258")];
    let client = reqwest::blocking::Client::new();
    let mut res = client.get("http://api.open-notify.org/iss-pass.json").query(&params).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    
    let b: Body = serde_json::from_str(&body).unwrap();
   // let sys_time = SystemTime::now();
    let unix_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();




    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);
    //println!("Body:\n{:#?}", b);
    let time_until_rise = b.response[0].risetime - unix_time;
    let hours = time_until_rise as f64 / 60.0 / 60.0;
    println!("ISS will rise in {0:2} hours", hours);



}
