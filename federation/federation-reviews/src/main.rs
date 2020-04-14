use async_graphql::http::GQLResponse;
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Object, QueryBuilder, Schema, SimpleObject, ID,
};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

struct User {
    id: ID,
}

#[Object(extends)]
impl User {
    #[field(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    #[field]
    async fn reviews<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Review> {
        let reviews = ctx.data::<Vec<Review>>();
        reviews
            .iter()
            .filter(|review| review.author.id == self.id)
            .collect()
    }
}

struct Product {
    upc: String,
}

#[Object(extends)]
impl Product {
    #[field(external)]
    async fn upc(&self) -> &String {
        &self.upc
    }

    #[field]
    async fn reviews<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Review> {
        let reviews = ctx.data::<Vec<Review>>();
        reviews
            .iter()
            .filter(|review| review.product.upc == self.upc)
            .collect()
    }
}

#[SimpleObject]
struct Review {
    #[field]
    body: String,

    #[field(provides = "username")]
    author: User,

    #[field]
    product: Product,
}

struct Query;

#[Object]
impl Query {
    #[entity]
    async fn find_user_by_id<'a>(&self, id: ID) -> User {
        User { id }
    }

    #[entity]
    async fn find_product_by_upc<'a>(&self, upc: String) -> Product {
        Product { upc }
    }
}

#[tokio::main]
async fn main() {
    let reviews = vec![
        Review {
            body: "A highly effective form of birth control.".into(),
            author: User { id: "1234".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
        Review {
            body: "Fedoras are one of the most fashionable hats around and can look great with a variety of outfits.".into(),
            author: User { id: "1234".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
        Review {
            body: "This is the last straw. Hat you will wear. 11/10".into(),
            author: User { id: "7777".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
    ];

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(reviews)
        .finish();

    warp::serve(
        graphql(schema).and_then(|(schema, builder): (_, QueryBuilder)| async move {
            let resp = builder.execute(&schema).await;
            Ok::<_, Infallible>(warp::reply::json(&GQLResponse(resp)).into_response())
        }),
    )
    .run(([0, 0, 0, 0], 4003))
    .await;
}