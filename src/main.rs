// use std::io::stdin;
use std::env::args;

#[derive(Debug)]
struct Data {
    amount: f64,
    current_stock_price: f64,
    expected_stock_price: f64,
}


#[derive(Debug)]
struct Result {
    amount: f64,
    amount_of_stocks: f64,
    current_stock_price: f64,
    expected_stock_price: f64,
    new_amount: f64,
    clean_profit: f64,
}


fn main() {
    let _args = args();
    let size = _args.size_hint().0;
    let arguments_vec: Vec<String> = _args.collect();
    let mut total: Vec<Result> = Vec::new();
    let mut total_amount: f64 = 0.0;
    let mut total_clean_profit: f64 = 0.0;

    for x in 1..size {
        let stock_result: Result = calc_result(to_data(&arguments_vec[x]));
        total_amount+=stock_result.new_amount;
        total_clean_profit+=stock_result.clean_profit;
        total.push(stock_result);
        println!("\n");
    }

    print_results(total,total_amount, total_clean_profit)
}

fn print_results(results: Vec<Result>, total_amount: f64, clean_profit: f64 )  {
    let results_len = results.len();
    for i in 0..results.len() {
        let result = &results[i];

        if results_len > 1  {
            println!("=============");
            println!("For stock №{}", i);
            println!("=============");
        }

        println!("You had: {}", result.amount);
        println!("You bought {} for {}", result.amount_of_stocks, result.current_stock_price);
        println!("You expect this stock to be {}", result.expected_stock_price);
        println!("If this happens before you had {} and NOW you have {}", result.amount, result.new_amount);
        println!("Clean profit will be {}", result.clean_profit);
        
        if results_len > 1  {
            println!("=============\n");
        }
    }

    if results_len > 1 {
        println!("=============");
        println!("Total new amount is {}", total_amount);
        println!("Total clean profit is {}", clean_profit);
        println!("=============");
    }
}

fn calc_result(data: Data) -> Result {
    let amount = data.amount;
    let current_stock_price =data.current_stock_price;
    let expected_stock_price = data.expected_stock_price;
    
    
    let amount_of_stocks = amount / current_stock_price;
    let new_amount = amount_of_stocks * expected_stock_price;
    let clean_profit = new_amount - amount;

    Result {
        amount,
        amount_of_stocks,
        current_stock_price,
        expected_stock_price,
        new_amount,
        clean_profit,
    }
}

fn to_data(value: &str) -> Data {
    let stonks: Vec<f64> = value.split('-').map(|x| to_f64(x)).collect();

    Data {
            amount: stonks[0],
            current_stock_price: stonks[1],
            expected_stock_price: stonks[2],
        }
}

fn to_f64(arg: &str) -> f64 {
    arg.parse::<f64>().expect("Couldn't parse to f64")
}