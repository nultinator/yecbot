use reqwest::Url;
use reqwest;
use serde_json;
use tokio;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.xeggex.com/api/v2";
const PAIRS: [&str; 2] = ["btc", "usdt"];

pub async fn get_orderbook(currency: &str, ref_currency: &str) -> HashMap<String, HashMap<String, f32>> {
    //create our HashMap to return
    let mut return_map: HashMap<String, HashMap<String, f32>> = HashMap::new();
    //create a string of our url
    let url: String = format!("{}/market/getorderbookbysymbol/{}_{}", BASE_URL, currency, ref_currency);
    //send the GET request and unwrap the result
    let resp = reqwest::get(url)
        .await
        .expect("Failed to get the orderbook")
        .text()
        .await
        .unwrap();
    //create a json object from the result
    let resp_json: serde_json::Value = serde_json::from_str(&resp).unwrap();
    //xeggex returns this as a string, so we parse the string for f32
    let current_bid_amount =  resp_json["bids"][0]["quantity"]
        .as_str().unwrap().parse::<f32>().unwrap();
    //we retrieve this the same way as with Southx        
    let current_bid_price = resp_json["bids"][0]["numberprice"]
        .as_f64().unwrap() as f32;
    //once again, parse the string for f32
    let current_ask_amount = resp_json["asks"][0]["quantity"]
        .as_str().unwrap().parse::<f32>().unwrap();
    //same as Southx
    let current_ask_price = resp_json["asks"][0]["numberprice"]
        .as_f64().unwrap() as f32;
    //insert these pairs into the map
    let mut map = HashMap::<String, f32>::new();
    map.insert("BidAmount".to_string(), current_bid_amount);
    map.insert("BidPrice".to_string(), current_bid_price);
    map.insert("AskAmount".to_string(), current_ask_amount);
    map.insert("AskPrice".to_string(), current_ask_price);
    //calculate the cost to fill ask and bid
    let fill_bid = current_bid_amount * current_bid_price;
    let fill_ask = current_ask_amount * current_ask_price;
    //insert the calculated amounts into the map
    map.insert("FillBid".to_string(), fill_bid);
    map.insert("FillAsk".to_string(), fill_ask);
    //insert the map into a larger map named after our reference currency
    return_map.insert(ref_currency.to_string(), map);

    return return_map;
}