/*
rustup toolchain install nightly
cargo +nightly run --release

Test instances:
dev.lemmy.ml
enterprise.lemmy.ml
voyager.lemmy.ml
ds9.lemmy.ml
*/

use maud::{Markup};
use actix_web::{web, App, HttpServer, Result, error};
use actix_web::client::Client;
use serde::Deserialize;

mod templates;
mod lemmy_api;

use crate::templates::{redirect_page, post_list_page, post_page, comment_page, communities_page, user_page};
use crate::lemmy_api::{PagingParams, get_post_list, get_post, get_community_list, get_community, get_user};

#[derive(Deserialize)]
struct RedirForm {
    i: Option<String>,
}

#[derive(Deserialize)]
struct PathParams2 {
    inst: String,
    command: String
}
#[derive(Deserialize)]
struct PathParams3 {
    inst: String,
    command: String,
    id: String
}
#[derive(Deserialize)]
struct PathParams5 {
    inst: String,
    command: String,
    id: String,
    sub_command: String,
    sub_id: String
}

async fn index(web::Query(query): web::Query<RedirForm>) -> Result<Markup> {
    Ok(redirect_page(
        query.i.ok_or(error::ErrorExpectationFailed("i parameter missing. Is Nginx running?"))?
    ))
}

async fn lvl1(path: web::Path<String>, query: web::Query<PagingParams>) -> Result<Markup> {
    let inst = &path.to_string();
    let client = &Client::default();

    let post_list = get_post_list(client, inst, None, None).await?;
    Ok(post_list_page(inst, post_list))
}

async fn lvl2(p: web::Path<PathParams2>, query: web::Query<PagingParams>) -> Result<Markup> {
    let client = &Client::default();
    if p.command == "communities" {
        let communities = get_community_list(client, &p.inst, None).await?;
        Ok(communities_page(&p.inst, communities))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

async fn lvl3(p: web::Path<PathParams3>, query: web::Query<PagingParams>) -> Result<Markup> {
    let client = &Client::default();
    if p.command == "post" {
        let post_detail = get_post(client, &p.inst, &p.id, None).await?;
        Ok(post_page(&p.inst, post_detail))
    } else if p.command == "c" {
        let communities = get_community(client, &p.inst, &p.id).await?;
        let post_list = get_post_list(client, &p.inst, Some(&communities.community.id), None).await?;
        Ok(post_list_page(&p.inst, post_list))
    } else if p.command == "u" {
        let user = get_user(client, &p.inst, &p.id, None).await?;
        Ok(user_page(&p.inst, user))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

// async fn lvl4(path: web::Path<PathParams4>, query: web::Query<PagingParams>) -> Result<Markup> {
//     Err(error::ErrorExpectationFailed("Invalid path"))
// }

async fn lvl5(p: web::Path<PathParams5>, query: web::Query<PagingParams>) -> Result<Markup> {
    let client = &Client::default();

    if p.command == "post" && p.sub_command == "comment" {
        let post_detail = get_post(client, &p.inst, &p.id, None).await?;
        let comment_id = match p.sub_id.parse::<i32>() {
            Ok(cid) => cid,
            Err(_) => return Err(error::ErrorExpectationFailed("Comment ID is invalid"))
        };
        let comment = match post_detail.comments.iter().find(|c| c.id == comment_id) {
            Some(c) => c.clone(),
            _ => return Err(error::ErrorExpectationFailed("Comment doesn't belong to this post"))
        };
        Ok(comment_page(&p.inst, comment, post_detail))
        
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new()
        .service(
            web::resource("/").route(web::get().to(index))
        ).route(
            "/{inst}", web::get().to(lvl1)
        ).route(
            "/{inst}/{command}", web::get().to(lvl2)
        ).route(
            "/{inst}/{command}/{id}", web::get().to(lvl3)
        // ).route(
        //     "/{inst}/{command}/{id}/{sub_command}", web::get().to(lvl4)
        ).route(
            "/{inst}/{command}/{id}/{sub_command}/{sub_id}", web::get().to(lvl5)
        )
    })
    .bind("127.0.0.1:1131")?
    .run().await
}