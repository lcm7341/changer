use actix_web::{web::get, *};
use serde_json::Value;
use std::sync::Mutex;
use actix_cors::Cors;


#[derive(serde::Deserialize, serde::Serialize)]
struct Bills {
    hundreds: i32,
    fifties: i32,
    twenties: i32,
    tens: i32,
    fives: i32,
    ones: i32
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Coins {
    quarters: i32,
    dimes: i32,
    nickels: i32,
    pennies: i32
}

struct State {
    cost: Mutex<f64>,
    money_paid: Mutex<f64>
}

#[get("/api/bills")]
async fn get_bills(state: web::Data<State>) -> impl Responder {
    let mut current_count = *state.clone().cost.lock().unwrap() - *state.clone().money_paid.lock().unwrap();
    let mut bills: Bills = Bills { hundreds: 0, fifties: 0, twenties: 0, tens: 0, fives: 0, ones: 0 };

    while current_count.floor() > 0_f64 {
        if current_count > 100_f64 {
            bills.hundreds = current_count as i32 / 100;
            current_count -= (100 * bills.hundreds) as f64;
        }
        if current_count > 50_f64 {
            bills.fifties = current_count as i32 / 50;
            current_count -= (50 * bills.fifties) as f64;
        }
        if current_count > 20_f64 {
            bills.twenties = current_count as i32 / 20;
            current_count -= (20 * bills.twenties) as f64;
        }
        if current_count > 10_f64 {
            bills.tens = current_count as i32 / 10;
            current_count -= (10 * bills.tens) as f64;
        }
        if current_count > 5_f64 {
            bills.fives = current_count as i32 / 5;
            current_count -= (5 * bills.fives) as f64;
        }
        if current_count > 1_f64 {
            bills.ones = current_count as i32 / 1;
            current_count -= bills.ones as f64;
        }
    }

    HttpResponse::Ok().json(bills)
}

#[get("/api/coins")]
async fn get_coins(state: web::Data<State>) -> impl Responder {
    let money_paid = *state.clone().cost.lock().unwrap() - *state.clone().money_paid.lock().unwrap();
    let cents_int_as_f64 = (money_paid - (money_paid.floor() as f64)) * 100_f64;

    let mut current_count = cents_int_as_f64.round() as i32;
    let mut coins: Coins = Coins { quarters: 0, dimes: 0, nickels: 0, pennies: 0 };

    while current_count > 0 {
        if current_count > 25 {
            coins.quarters = current_count as i32 / 25;
            current_count -= 25 * coins.quarters;
        }
        if current_count > 10 {
            coins.dimes = current_count as i32 / 10;
            current_count -= 10 * coins.dimes;
        }
        if current_count > 5 {
            coins.nickels = current_count as i32 / 5;
            current_count -= 5 * coins.nickels;
        }
        if current_count > 1 {
            coins.pennies = current_count as i32 / 1;
            current_count -= coins.pennies;
        }
    }

    HttpResponse::Ok().json(coins)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ValueResponse {
    money_paid: f64
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CostResponse {
    cost: f64
}

#[derive(serde::Deserialize, serde::Serialize)]
struct AllChange {
    bills: Bills,
    coins: Coins
}

#[get("/api/get_cost")]
async fn get_cost(state: web::Data<State>) -> impl Responder {
    HttpResponse::Ok().json(CostResponse{cost: *state.cost.lock().unwrap()})
}

#[post("/api/set_cost")]
async fn set_cost(state: web::Data<State>, data: web::Json<CostResponse>) -> impl Responder {
    *state.cost.lock().unwrap() = data.cost as f64;
    println!("data.0.cost: {}", data.cost);
    HttpResponse::Ok()
}

#[post("/api/change_money")]
async fn change_money(state: web::Data<State>, data: web::Json<ValueResponse>) -> impl Responder {
    *state.money_paid.lock().unwrap() = data.money_paid as f64;
    HttpResponse::Ok()
}

#[get("/api/money_paid")]
async fn get_money(state: web::Data<State>) -> impl Responder {
    let money_paid = *state.money_paid.lock().unwrap();
    HttpResponse::Ok().json(ValueResponse {money_paid})
}

#[post("/api/get_change")]
async fn get_change(state: web::Data<State>, data: web::Json<AllChange>) -> impl Responder {
    
    HttpResponse::Ok().json(AllChange {
        bills,
        coins: Coins { quarters: 0, dimes: 0, nickels: 0, pennies: 0 }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(State {
        cost: Mutex::new(10.0),
        money_paid: Mutex::new(0.0)
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .send_wildcard();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(get_bills)
            .service(get_coins)
            .service(get_money)
            .service(change_money)
            .service(set_cost)
            .service(get_cost)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
