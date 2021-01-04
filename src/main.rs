mod constant;
mod dbs;
mod gql;

mod users;
mod projects;

use crate::constant::ENV;

//  Tide application scope state.
#[derive(Clone)]
pub struct State(
    pub  async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
);

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::with_state(State(gql::build_schema().await));

    //environment variables defined in .env file
    app.at("/").get(tide::Redirect::new(ENV.get("GRAPHIQL_PATH").unwrap()));
    // app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(async_graphql_tide::endpoint(schema));
    app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(gql::graphql);
    app.at(ENV.get("GRAPHIQL_PATH").unwrap()).get(gql::graphiql);

    app.listen(format!(
        "{}:{}",
        ENV.get("GRAPHQL_ADDRESS").unwrap(),
        ENV.get("GRAPHQL_PORT").unwrap()
    ))
    .await?;

    Ok(())
}
