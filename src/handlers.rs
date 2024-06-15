use crate::database::establish_connection;
use crate::models::{Human, NewHuman, UpdateHuman};
use crate::schema::human::dsl::*;
use actix_web::{web, HttpResponse, Result};

use diesel::prelude::*;

pub async fn create_human(new_human: web::Json<NewHuman>) -> Result<HttpResponse> {
    // use crate::schema::human::dsl::*;
    let mut connection = establish_connection();

    diesel::insert_into(human)
        .values(&new_human.into_inner())
        .execute(&mut connection)
        .expect("Error inserting new human");

    Ok(HttpResponse::Ok().json("data inserted into the database"))
}

pub async fn get_humans() -> Result<HttpResponse> {
    // use crate::schema::human::dsl::*;
    let mut connection = establish_connection();

    let humans = human
        .load::<Human>(&mut connection)
        .expect("Error loading humans");

    Ok(HttpResponse::Ok().json(humans))
}

pub async fn get_human(idd: web::Path<i32>) -> Result<HttpResponse> {
    // use crate::schema::human::dsl::*;
    let mut conn = establish_connection();

    let h = human
        .find(idd.abs())
        .load::<Human>(&mut conn)
        .expect("Error");

    Ok(HttpResponse::Ok().json(h))
}

pub async fn update_human(
    idd: web::Path<i32>,
    update_human: web::Json<UpdateHuman>,
) -> Result<HttpResponse> {
    // use crate::schema::human::dsl::*;
    let mut conn = establish_connection();

    let updated_human = diesel::update(human.find(idd.abs()))
        .set(&update_human.into_inner())
        .execute(&mut conn)
        .expect("Failed to update human");

    Ok(HttpResponse::Ok().json(updated_human))
}

pub async fn delete_human(idd: web::Path<i32>) -> Result<HttpResponse> {
    // use crate::schema::human::dsl::*;
    let mut conn = establish_connection();

    diesel::delete(human.find(idd.abs()))
        .execute(&mut conn)
        .expect(&format!("Unable to fin human {:?}", idd));

    Ok(HttpResponse::Ok().json("Deleted successfully"))
}
