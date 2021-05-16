use crate::db;
use crate::error_handler::CustomError;
use crate::schema::reserve;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "reserve"]
pub struct Reserve{
    pub full_name:String,
    pub phone_number:String,
    pub reserve_date:String
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "reserve"]
pub struct Reserves{
    pub id:i32,
    pub full_name:String,
    pub phone_number:String,
    pub reserve_date:String
}

impl Reserve{
    pub fn from(reserve: Reserve) -> Reserve{
        Reserve{
            full_name:reserve.full_name,
            phone_number:reserve.phone_number,
            reserve_date:reserve.reserve_date,
        }
    }
}

//DATABASE_URL=postgres://root:abolomranihamidradfar@postgresql-adminer-ahmad-test.apps.ir-thr-at1.arvan.run/employeesrest_api

impl Reserves{
    pub fn list() -> Result<Vec<Self>, CustomError>{
        let conn = db::connection()?;
        let reserves = reserve::table.load::<Reserves>(&conn)?;
        Ok(reserves)
    }

    pub fn create(reserve: Reserve) -> Result<Self, CustomError>{
        let conn = db::connection()?;
        let res = Reserve::from(reserve);
        let res = diesel::insert_into(reserve::table)
                        .values(res)
                        .get_result(&conn)?;
        
        Ok(res)
    }
}