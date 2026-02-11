use actix_web::cookie::Key;
use actix_web::*;
use std::sync::Mutex;
use std::env;
use actix_cors::Cors;
use actix_session::*;


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Bills {
    hundreds: i32,
    fifties: i32,
    twenties: i32,
    tens: i32,
    fives: i32,
    ones: i32
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Coins {
    quarters: i32,
    dimes: i32,
    nickels: i32,
    pennies: i32
}

fn get_bills(transaction: &Transaction) -> Bills {
    let mut current_count = transaction.given - transaction.cost;
    let mut bills: Bills = Bills { hundreds: 0, fifties: 0, twenties: 0, tens: 0, fives: 0, ones: 0 };

    while current_count.floor() > 0_f64 {
        if current_count >= 100_f64 {
            bills.hundreds = current_count as i32 / 100;
            current_count -= (100 * bills.hundreds) as f64;
        }
        if current_count >= 50_f64 {
            bills.fifties = current_count as i32 / 50;
            current_count -= (50 * bills.fifties) as f64;
        }
        if current_count >= 20_f64 {
            bills.twenties = current_count as i32 / 20;
            current_count -= (20 * bills.twenties) as f64;
        }
        if current_count >= 10_f64 {
            bills.tens = current_count as i32 / 10;
            current_count -= (10 * bills.tens) as f64;
        }
        if current_count >= 5_f64 {
            bills.fives = current_count as i32 / 5;
            current_count -= (5 * bills.fives) as f64;
        }
        if current_count >= 1_f64 {
            bills.ones = current_count as i32 / 1;
            current_count -= bills.ones as f64;
        }
    }

    bills
}

fn get_coins(transaction: &Transaction) -> Coins {
    let money_paid = transaction.given - transaction.cost;
    let cents_int_as_f64 = (money_paid - (money_paid.floor() as f64)) * 100_f64;

    let mut current_count = cents_int_as_f64.round() as i32;
    let mut coins: Coins = Coins { quarters: 0, dimes: 0, nickels: 0, pennies: 0 };

    while current_count > 0 {
        if current_count >= 25 {
            coins.quarters = current_count as i32 / 25;
            current_count -= 25 * coins.quarters;
        }
        if current_count >= 10 {
            coins.dimes = current_count as i32 / 10;
            current_count -= 10 * coins.dimes;
        }
        if current_count >= 5 {
            coins.nickels = current_count as i32 / 5;
            current_count -= 5 * coins.nickels;
        }
        if current_count >= 1 {
            coins.pennies = current_count as i32 / 1;
            current_count -= coins.pennies;
        }
    }

    coins
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Transaction {
    cost: f64,
    given: f64
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Change {
    bills: Bills,
    coins: Coins
}

#[post("/api/calculate_change")]
async fn calculate_change(session: Session, data: web::Json<Transaction>) -> impl Responder {
    let transaction = Transaction {
        cost: data.cost,
        given: data.given
    };

    let bills = get_bills(&transaction);
    let coins = get_coins(&transaction);

    let change = Change {
        bills: bills.clone(),
        coins: coins.clone()
    };

    let session_change: Change = change.clone();

    session.insert("change", &session_change).unwrap();

    HttpResponse::Ok().json(change)
}

#[get("/api/get_change")]
async fn get_change(session: Session) -> impl Responder {
    let change = session.get("change")
        .unwrap()
        .unwrap_or(Change {
            bills: Bills { hundreds: 0, fifties: 0, twenties: 0, tens: 0, fives: 0, ones: 0 },
            coins: Coins { quarters: 0, dimes: 0, nickels: 0, pennies: 0 }
        });
    HttpResponse::Ok().json(change)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let key = Key::generate();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .send_wildcard();

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::new(
                    storage::CookieSessionStore::default(),
                    key.clone(),
                ))
            .service(calculate_change)
            .service(get_change)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
