#![allow(clippy::needless_lifetimes)]

use async_graphql::http::GQLResponse;
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Object, QueryBuilder, Schema, SimpleObject,
};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

#[SimpleObject]
struct Product {
    upc: String,
    name: String,
    price: i32,
}

struct Query;

#[Object(extends)]
impl Query {
    async fn top_products<'a>(&self, ctx: &'a Context<'_>) -> &'a Vec<Product> {
        ctx.data::<Vec<Product>>()
    }

    #[entity]
    async fn find_product_by_upc<'a>(
        &self,
        ctx: &'a Context<'_>,
        upc: String,
    ) -> Option<&'a Product> {
        let hats = ctx.data::<Vec<Product>>();
        hats.iter().find(|product| product.upc == upc)
    }
}

#[tokio::main]
async fn main() {
    let hats = vec![
        Product {
            upc: "top-1".to_string(),
            name: "Trilby".to_string(),
            price: 11,
        },
        Product {
            upc: "top-2".to_string(),
            name: "Fedora".to_string(),
            price: 22,
        },
        Product {
            upc: "top-3".to_string(),
            name: "Boater".to_string(),
            price: 33,
        },
    ];

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(hats)
        .finish();

    warp::serve(
        graphql(schema).and_then(|(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        }),
    )
    .run(([0, 0, 0, 0], 4002))
    .await;
}
