# tide-async-graphql-mongodb

Clean boilerplate for graphql services using tide, async-graphql, surf, graphql-client, handlebars-rust, and mongodb. 

## Features

- [x] User register
- [x] Salt and hash a password with PBKDF2 - 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
- [x] Sign in
- [x] JSON web token authentication
- [x] Change password
- [x] Profile Update
- [x] User: query & mutation
- [x] Project: query & mutation
- [x] Use Surf & graphql-client to get & parse graphql data
- [x] Render graphql data to template engine handlebars

## Stacks

- [Rust](https://www.rust-lang.org) - [中文资料集萃](https://budshome.com)
- [Tide](https://github.com/http-rs/tide) - [中文文档](https://tide.budshome.com)
- [async-graphql](https://crates.io/crates/async-graphql) - [中文文档](https://async-graphql.budshome.com)
- [mongodb & mongo-rust-driver](https://crates.io/crates/mongodb)
- [handlebars-rust](https://crates.io/crates/handlebars)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)

## How to run?

``` Bash
git clone https://github.com/zzy/tide-async-graphql-mongodb.git
cd tide-async-graphql-mongodb
```

Put the environment variables into a `.env` file:

```
WEB_ADDRESS=0.0.0.0
WEB_PORT=8080

GRAPHQL_ADDRESS=0.0.0.0
GRAPHQL_PORT=8080
GRAPHQL_PATH=graphql
GRAPHIQL_PATH=graphiql

MONGODB_URI=mongodb://mongo:mongo@localhost:27017
MONGODB_BUDSHOME=budshome

SITE_KEY=0F4EHz+1/hqVvZjuB8EcooQs1K6QKBvLUxqTHt4tpxE=
CLAIM_EXP=10000000000
```

Build & Run:

``` Bash
cargo build
cargo run
```

GraphiQL: connect to http://localhost:8080/graphiql with browser.

## Queries

- getUserByEmail(...): User!
- getUserByUsername(...): User!
- userSignIn(...): SignInfo!
- allUsers(...): [User!]!
- allProjects: [Project!]!
- allProjectsByUser(...): [Project!]!

## MUTATIONS

- userRegister(...): User!
- userChangePassword(...): User!
- userUpdateProfile(...): User!
- addProject(...): Project!

## Sample Usage

Sample mutation for user register:
```
mutation {
  userRegister(
    newUser: { 
      email: "example@budshome.com", 
      username: "我是谁", 
      password: "wo#$shi^$shui" 
    }
  ) {
    id
    email
    username
  }
}
```

Sample query for user sign in:
```
{
  userSignIn(
    userAccount: {
      email: "example@budshome.com"
      username: ""
      password: "wo#$shi^$shui"
    }
  ) {
    email
    username
    token
  }
}
```

When submit method `userSignIn`, a token would be generated, use this token for query all users and every user's projects:
```
{
  allUsers(
    token: "fyJ0eXAiOiJKV1Q..."
  ) {
    id
    email
    username

    projects {
      id
      userId
      subject
      website
    }
  }
}
```

Sample query and mutation for projects was similar to users.
