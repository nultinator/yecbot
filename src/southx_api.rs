use tungstenite;
use reqwest::Url;
use reqwest;
use serde_json;
use tokio;
use std::collections::HashMap;

const BASE_URL: &str = "https://market.southxchange.com/api/v4";
const PAIRS: [&str; 2] = ["btc", "usdt"];

pub async fn price_check(currency: &str, ref_currency: &str) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    let url = format!("{}/price/{}/{}", BASE_URL, currency,ref_currency);
    let resp = reqwest::get(url)
        .await
        .expect("Something happened")
        .text()
        .await
        .unwrap();
    let result = format!("{}", resp.trim());//.replace("\"", "");
    let response_json: serde_json::Value = serde_json::from_str(&result).unwrap();
    let bid: f64 = response_json["Bid"].as_f64().expect("error decoding bid");
    let ask: f64 = response_json["Ask"].as_f64().expect("error decoding ask");
    //println!("BID: {:?}", bid);
    map.insert("bid".to_string(), bid);
    map.insert("ask".to_string(), ask);
    //println!("ASK: {:?}", ask);
    return map;
}

pub async fn get_all_prices(coin_ticker: &str) -> HashMap<String, HashMap<String, f64>>{
    let mut map = HashMap::new();    
    for pair in PAIRS {
        let prices = price_check("yec", pair).await;
        map.insert(pair.to_string(), prices);
    }
    return map;
}

pub async fn calculate_average_price(coin: HashMap<String, HashMap<String, f64>>) -> HashMap<String, f64> {
    let mut asks = Vec::new();
    let mut bids = Vec::new();
    let mut map: HashMap<String, f64> = HashMap::new();

    for (k,v) in coin.iter() {
        //println!("yec/{}", k);
        let mut bid_value = 0.0;
        let mut ask_value = 0.0;
        if k == "btc" {
            let pricer = price_check("btc", "usdt").await["bid"];
            bid_value = pricer * v["bid"];
            ask_value = pricer * v["ask"];
            
        } else if k == "usdt" {
            bid_value = v["bid"];
            ask_value = v["ask"];
        }
        asks.push(ask_value);
        bids.push(bid_value);
    }
    let mut asks_sum = 0.0;
    let mut bids_sum = 0.0;
    for i in asks.iter() {
        asks_sum += i;
    }
    for i in bids.iter() {
        bids_sum += i;
    }
    let avg_bid: f64 = bids_sum / (bids.len() as f64);
    let avg_ask: f64 = asks_sum / (asks.len() as f64);
    map.insert("avg_bid".to_string(), avg_bid);
    map.insert("avg_ask".to_string(), avg_ask);
    return map;
}

pub async fn get_orderbook(currency: &str, ref_currency: &str) -> HashMap<String, HashMap<String, f32>> {
    let url: String = format!("{}/book/{}/{}", BASE_URL, currency, ref_currency);
    let resp = reqwest::get(url)
        .await
        .expect("Failed to get the orderbook")
        .text()
        .await
        .unwrap();
    let resp_json: serde_json::Value = serde_json::from_str(&resp).unwrap();
    let current_bid_amount = resp_json["BuyOrders"][0]["Amount"]
        .as_f64().unwrap() as f32;
    let current_bid_price = resp_json["BuyOrders"][0]["Price"]
        .as_f64().unwrap() as f32;
    let current_ask_amount = resp_json["SellOrders"][0]["Amount"]
        .as_f64().unwrap() as f32;
    let current_ask_price = resp_json["SellOrders"][0]["Price"]
        .as_f64().unwrap() as f32;

    let mut map: HashMap<String, f32> = HashMap::new();
    map.insert("BidAmount".to_string(), current_bid_amount);
    map.insert("BidPrice".to_string(), current_bid_price);
    map.insert("AskAmount".to_string(),current_ask_amount);
    map.insert("AskPrice".to_string(),current_ask_price);

    let fill_bid = current_bid_amount * current_bid_price;
    let fill_ask = current_ask_amount * current_ask_price;
    map.insert("FillBid".to_string(), fill_bid);
    map.insert("FillAsk".to_string(), fill_ask);
    let mut return_map: HashMap<String, HashMap<String, f32>> = HashMap::new();
    return_map.insert(ref_currency.to_string(), map);

    return return_map;
}

pub async fn find_arb_opportunity(coin: &str) {
    let all_prices = get_all_prices(coin)
        .await;
    let btc_price = all_prices["btc"].clone();
    all_prices.insert()
    for price in all_prices.iter() {
        println!("{:?}", price);
    }
}
    //println!("ALL PRICES\n {:?}", all_prices);

    //let averages = calculate_average_price(all_prices).await;
    //println!("AVERAGES\n{:?}", averages);



pub async fn coin_to_usdt(coin: &str) -> HashMap<String, f64>{
    let price = price_check(&coin, "usdt").await;
    return price;
}

































pub fn socket_hookup() {
    let url: &str = "wss://market.southxchange.com/api/v4/connect/";
    println!("Connecting to {}", url);

    let (mut socket, mut _response) = 
        tungstenite::connect(Url::parse(&url)
            .unwrap())
            .expect("Failed to connect");

    println!("HTTP Status: {}", _response.status());

    let subscribe_message = tungstenite::Message::Text("('request', 924)".to_string());

    socket.write_message(subscribe_message).expect("Couldn't subscribe");

    println!("HTTP Status: {}", _response.status());

    println!("Subscribed!");

    let mut counter: u32 = 0;

    loop {

        let msg: tungstenite::Message = match socket.read_message().ok() {
            Some(m) => m,
            __ => continue
        };

        println!("Status Code: {}", _response.status());

        //let msg_json = serde_json::Value::from(msg.to_string());

        //let msg_array: [&str; 3] = ["YEC", "BTC", "924"];

        println!("Message #{}", counter);
        counter += 1;


        //println!("{:?}", msg_json.to_string());
    }
}


