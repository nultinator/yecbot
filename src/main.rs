

mod southx_api; 
mod xeggex_api;

#[tokio::main]
async fn main() {

    let mut running: bool = true;

    while running {
        println!("Welcome, please choose one the the options below");
        println!("1: Listen to orders ---NOT WORKING");
        println!("2: Get current pair prices");
        println!("3: Get an orderbook");
        println!("4: Find arbitrage");
        println!("5: Find USD price of a coin");
        println!("q: quit");
        let mut resp: String = String::new();

        std::io::stdin().read_line(&mut resp).unwrap();

        match resp.trim() {
            "1" => southx_api::socket_hookup(),
            "2" => {
                let yec_prices =  southx_api::get_all_prices("yec").await;
                println!("Current prices \n{:?}", yec_prices);
                let yec_average = southx_api::calculate_average_price(yec_prices).await;
                println!("Averages \n{:?}", yec_average);
            },
            "3" => {
                let southx_orderbook = southx_api::get_orderbook("yec", "btc")
                    .await;
                println!("SOUTHX\n{:?}", southx_orderbook);

                let xeggex_orderbook = xeggex_api::get_orderbook("YEC", "BTC")
                    .await;
                println!("XEGGEX\n{:?}", xeggex_orderbook);
            },
            "4" => southx_api::find_arb_opportunity("yec").await,
            "5" => {
                println!("Please choose a ticker name");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let price = southx_api::coin_to_usdt(&input.trim()).await;
                println!("{:?} : {:?}", input.trim(), price["bid"],)
            }
            "q" => {
                println!("Program exiting");
                running = false;
            },
            _ => println!("Unknown command")
        }
        
    }
}
