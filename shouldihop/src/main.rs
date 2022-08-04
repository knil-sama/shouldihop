#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;

#[derive(Debug, FromForm)]
struct MyForm {
    current_company_salary: String,
    new_company_salary: String,
    stay_window: String, 
}

#[post("/", data = "<form>")]
fn submit(form: Form<MyForm>) -> Template {
    let current_company_salary = form.current_company_salary.parse::<i32>().unwrap();
    let new_company_salary = form.new_company_salary.parse::<i32>().unwrap();
    let stay_window = form.stay_window.parse::<i32>().unwrap();
    let _total_current_company_salary = current_company_salary * stay_window;
    let _total_new_company_salary = new_company_salary * stay_window;
    let _gain = _total_new_company_salary - _total_current_company_salary;
    let total_current_company_salary = _total_current_company_salary.to_string();
    let total_new_company_salary = _total_new_company_salary.to_string();
    let gain = _gain.to_string();
    let context: HashMap<&str, &str> = [("current_company_salary", form.current_company_salary.as_str()),("new_company_salary", form.new_company_salary.as_str()), ("stay_window",form.stay_window.as_str()),("has_result","true"),("total_current_company_salary",&total_current_company_salary),("total_new_company_salary", &total_new_company_salary) ,("gain", &gain)]
        .iter().cloned().collect();
    Template::render("index", &context)
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("current_company_salary", "30000"),("new_company_salary","31000"),("stay_window","5")]
        .iter().cloned().collect();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, submit])
    .attach(Template::fairing())
    .launch();
}