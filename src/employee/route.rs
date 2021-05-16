// use crate::note::Note;
use crate::employee::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use serde_json::json;

#[get("/employees")]
async fn findall() -> Result<HttpResponse, CustomError>{
    println!("****************************************");
    let employees = Employees::find_all()?;
    Ok(HttpResponse::Ok().json(
        employees
    ))
}


#[post("/employee/{id}")]
async fn find(id: web::Path<i32>) ->  Result<HttpResponse, CustomError>{
    
    let Emp = Employee{
        firstname:String::from("mamad"),
        lastnam : String::from("bandari"),
        age:12,
        salary:3000000,
        department:String::from("dcdcd")
    };
    let employee1 = Employees::create(Emp);


    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(
        employee
    ))
}

#[post("/newemp")]
async fn createemp(emp: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    println!("********************************** 8888 ***");
    let employee = Employees::create(emp.into_inner())?;
    Ok(HttpResponse::Ok().json(
        employee
    ))
}

#[post("/employee")]
async fn create(emp: web::Json<Employee>) -> Result<HttpResponse, CustomError>{
    
    let employee = Employees::create(emp.into_inner());
    let employee = match employee{
            Ok(emp) => emp,
            Err(error) =>panic!("every thing is fucked up {:?}", error),  
    };
    let res = HttpResponse::Ok().json(employee);
    Ok(res)
}

pub fn init_routes(config: &mut web::ServiceConfig) 
{
    config.service(create);
    config.service(findall);
    config.service(find);
    config.service(createemp);
}
