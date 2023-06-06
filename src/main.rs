// <------------------------ PACKAGES -------------------------->
// rocket framework
#[macro_use] extern crate rocket;
// database 
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};
// json
use rocket::serde::json::Json;
use serde_json::{Value, from_str};
// file
use std::fs;

// <------------------------ STRUCT/DATA STRUCTURE -------------------------->

// define database
#[derive(Database)]
#[database("test")]
struct Product(sqlx::MySqlPool); 

// <------------------------ GET PRODUCT BY ID ------------------------------>
// add no product id found
// no need to print all data
#[get("/<id>")]
async fn read(mut db: Connection<Product>, id: i64) -> Option<String> {
    match sqlx::query("SELECT PRODUCT_CODE, NAME, PRICE FROM product WHERE PRODUCT_ID = ?")
        .bind(id)
        .fetch_one(&mut *db)
        .await
    {
        Ok(row) => {
            let product_code: String = row.try_get(0).ok()?;
            let name: String = row.try_get(1).ok()?;
            let price: f64 = row.try_get(2).ok()?;
            let product_data = format!("Product Code: {}\nName: {}\nPrice: {}", product_code, name, price);
            println!("{}", product_data);
            Some(product_data)
        }
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("Product with ID {} not found", id);
            Some(format!("Product with ID {} not found", id))
        }
        Err(e) => {
            eprintln!("Error fetching product: {}", e);
            None
        }
    }
}

// <------------------------ GET PRODUCT BY ALL ------------------------------>

#[get("/all")]
async fn read_all(mut db: Connection<Product>) -> Option<String> {
    match sqlx::query("SELECT PRODUCT_CODE, NAME, PRICE FROM product")
        .fetch_all(&mut *db)
        .await
    {
        Ok(rows) => {
            let mut product_data = String::new();
            for row in rows {
                let product_code: String = row.try_get(0).ok()?;
                let name: String = row.try_get(1).ok()?;
                let price: f64 = row.try_get(2).ok()?;
                // Append the product data to the existing string, with a newline after each product
                product_data.push_str(&format!("Product Code: {}\nName: {}\nPrice: {}\n\n", product_code, name, price));
            }
            println!("{}", product_data);
            Some(product_data)
        }
        Err(e) => {
            eprintln!("Error fetching products: {}", e);
            None
        }
    }
}

// <------------------------ INSERT NEW PRODUCT ------------------------------>

#[post("/insert", format = "json", data = "<payload>")]
async fn insert(mut db: Connection<Product>, payload: String) -> Json<String> {
    // define variable 
    let json_value: Value;
    
    // if payload from URL
    if payload.starts_with("http://") || payload.starts_with("https://") {
        let response = reqwest::get(&payload).await.expect("Failed to fetch URL");
        let file_contents = response.text().await.expect("Failed to read response");
        json_value = from_str(&file_contents).expect("Failed to parse JSON");
    }
    // if raw json data is in one json object only (one set of data)
    else if payload.starts_with("{") && payload.ends_with("}") {
        json_value = match from_str(&payload) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
    }
	// if raw json data is in mutiple json objects (multiple set of data)
	else if payload.starts_with("[") && payload.ends_with("]") {
        json_value = match from_str(&payload) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
    } 
    // if data from local file
    else {
        let file_contents = match fs::read_to_string(&payload) {
            Ok(contents) => contents,
            Err(e) => {
                return Json(format!("Failed to read file: {}", e));
            }
        };
        json_value = match from_str(&file_contents) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
    }

    // check & get JSON data inside file/URL
    if let Value::Array(items) = json_value {
        for item in items {
            if let Value::Object(product) = item {
                let code = match product.get("code") {
                    Some(value) => match value.as_str() {
                        Some(str_value) => str_value,
                        None => return Json(format!("Failed to extract product code as string")),
                    },
                    None => return Json(format!("Failed to find product code")),
                };
                let price = match product.get("price") {
                    Some(value) => match value.as_f64() {
                        Some(float_value) => float_value,
                        None => return Json(format!("Failed to extract product price as float")),
                    },
                    None => return Json(format!("Failed to find product price")),
                };
                let name = match product.get("name") {
                    Some(value) => match value.as_str() {
                        Some(str_value) => str_value,
                        None => return Json(format!("Failed to extract product name as string")),
                    },
                    None => return Json(format!("Failed to find product name")),
                };

                // check if the product already exists
                let count_result = sqlx::query_scalar("SELECT COUNT(*) FROM product WHERE PRODUCT_CODE = ?")
                .bind(code)
                .fetch_one(&mut *db)
                .await;

                match count_result {
                    Ok(count) => {
                        let count: i64 = count;
                        if count > 0 {
                            return Json(format!("Product with code {} already exists", code));
                        }
                        else{
                            // database query
                            let result = sqlx::query(
                                "INSERT INTO product (PRODUCT_CODE, PRICE, NAME) VALUES (?, ?, ?)",  
                            )
                            .bind(code)
                            .bind(price)
                            .bind(name)
                            .execute(&mut *db)
                            .await;

                            match result {
                                // Might need to hide or not doing this
                                Ok(_) => println!("Product inserted successfully"),
                                Err(e) => println!("Failed to insert product: {}", e),
                            }
                        }
                    }
                    
                    Err(e) => println!("Failed to check for duplicate entry: {}", e),
                }
            }
        }
        Json(String::from("Products inserted successfully"))
    }
    else if let Value::Object(product) = json_value{
        let code = match product.get("code") {
            Some(value) => match value.as_str() {
                Some(str_value) => str_value,
                None => return Json(format!("Failed to extract product code as string")),
            },
            None => return Json(format!("Failed to find product code")),
        };
        let price = match product.get("price") {
            Some(value) => match value.as_f64() {
                Some(float_value) => float_value,
                None => return Json(format!("Failed to extract product price as float")),
            },
            None => return Json(format!("Failed to find product price")),
        };
        let name = match product.get("name") {
            Some(value) => match value.as_str() {
                Some(str_value) => str_value,
                None => return Json(format!("Failed to extract product name as string")),
            },
            None => return Json(format!("Failed to find product name")),
        };

        // check if the product already exists
        let count_result = sqlx::query_scalar("SELECT COUNT(*) FROM product WHERE PRODUCT_CODE = ?")
            .bind(code)
            .fetch_one(&mut *db)
            .await;

        match count_result {
            Ok(count) => {
                let count: i64 = count;
                if count > 0 {
                    return Json(format!("Product with code {} already exists", code));
                }
                else{
                    // database query
                    let result = sqlx::query(
                        "INSERT INTO product (PRODUCT_CODE, PRICE, NAME) VALUES (?, ?, ?)",  
                    )
                    .bind(code)
                    .bind(price)
                    .bind(name)
                    .execute(&mut *db)
                    .await;

                    match result {
                        Ok(_) => println!("Product inserted successfully"),
                        Err(e) => println!("Failed to insert product: {}", e),
                    }
                }
            }
            Err(e) => println!("Failed to check for duplicate entry: {}", e),
        }

        Json(String::from("Products inserted successfully"))
    }
	else{
		Json(String::from("Invalid JSON data provided"))
	}
}

// <------------------------ UPDATE PRODUCT ------------------------------>

// to read data inside file/URL
#[patch("/update/<id>", format = "json", data = "<payload>")]
async fn update(id: i32, db: Connection<Product>, payload: String) -> Json<String> {

    // define variable 
    let json_value: Value;
    // check if filepath has http/https
    if payload.starts_with("http://") || payload.starts_with("https://") {
        // Use reqwest library to sends an HTTP GET request to URL in variable 'payload'
        let response = reqwest::get(&payload).await.expect("Failed to fetch URL");
        // read the response from the HTTP request made to the URL specified in the payload variable
        let file_contents = response.text().await.expect("Failed to read response");
        // parse JSON data from response
        json_value = from_str(&file_contents).expect("Failed to parse JSON");
        // update product in database
        update_product(id, json_value, db).await
    }
    // if raw json data is in one json object only (one set of data)
    else if payload.starts_with("{") && payload.ends_with("}") {
        json_value = match from_str(&payload) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
        update_product(id, json_value, db).await
    }
	// if raw json data is in mutiple json objects (multiple set of data)
	else if payload.starts_with("[") && payload.ends_with("]") {
        json_value = match from_str(&payload) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
        update_product(id, json_value, db).await
    } 
    // if data from local file
    else {
        let file_contents = match fs::read_to_string(&payload) {
            Ok(contents) => contents,
            Err(e) => {
                return Json(format!("Failed to read file: {}", e));
            }
        };
        json_value = match from_str(&file_contents) {
            Ok(value) => value,
            Err(e) => {
                return Json(format!("Failed to parse JSON: {}", e));
            }
        };
        update_product(id, json_value, db).await
    }
}

// get JSON data & query database
async fn update_product(id: i32, json_value: Value, mut db: Connection<Product>) -> Json<String> {
    // check if JSON data is an object
    if let Value::Object(product) = json_value {
        // extract product information from JSON data
        let code = match product.get("code") {
            Some(value) => match value.as_str() {
                Some(str_value) => str_value,
                None => return Json(format!("Failed to extract product code as string")),
            },
            None => return Json(format!("Failed to find product code")),
        };
        let price = match product.get("price") {
            Some(value) => match value.as_f64() {
                Some(float_value) => float_value,
                None => return Json(format!("Failed to extract product price as float")),
            },
            None => return Json(format!("Failed to find product price")),
        };
        let name = match product.get("name") {
            Some(value) => match value.as_str() {
                Some(str_value) => str_value,
                None => return Json(format!("Failed to extract product name as string")),
            },
            None => return Json(format!("Failed to find product name")),
        };

        // update product in database
        let result = sqlx::query(
            "UPDATE product SET PRODUCT_CODE = ?, PRICE = ?, NAME = ? WHERE PRODUCT_ID = ? AND isDELETED = 'N'",
        )
        .bind(code)
        .bind(price)
        .bind(name)
        .bind(id)
        .execute(&mut *db)
        .await;

        match result {
            Ok(ref res) if res.rows_affected() == 1 => Json(format!("Product with ID {} updated successfully", id)),
            Ok(_) => Json(format!("No product found with ID: {}", id)),
            Err(e) => Json(format!("Failed to update product with ID {}: {}", id, e)), // this supposed to be not displayed to user
        }

    }else {
        Json(format!("Invalid JSON data provided"))
    }
}

// <------------------------ DELETE PRODUCT BY ID ------------------------------>

#[delete("/delete/<id>")]
async fn delete(mut db: Connection<Product>, id: i64) -> Json<String> {
    let result = sqlx::query("UPDATE product SET isDELETED = 'Y' WHERE PRODUCT_ID = ? AND isDELETED = 'N'")
        .bind(id)
        .execute(&mut *db)
        .await;

    match result {
        Ok(ref res) if res.rows_affected() == 1 => Json(String::from("Product deleted successfully")),
        Ok(_) => {
            let message = format!("No product found with ID: {}", id);
            eprintln!("{}", message);
            Json(message)
        },
        Err(e) => {
            let message = format!("Failed to delete product: {}", e);
            eprintln!("{}", message);
            Json(message)
        },
    }
}


// <------------------- ROCKET LAUNCH ------------------------------>

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Product::init()).mount("/", routes![read, read_all, insert, delete, update])
}
