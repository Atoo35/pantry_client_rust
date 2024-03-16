# Pantry Client in Rust

This is a Rust client for interacting with the Pantry API.

## Installation

```bash
cargo add pantry_client
```

or add the following to your `Cargo.toml` file:

```toml
pantry_client = "0.1.0"
```

## Usage

Create a new client by bringing in the package in scope

```rust
 let client = pantry_client::new_client("<PANTRY ID HERE>");
```

## Get Pantry

To get a pantry, use the GetPantry method.

```rust
    let res = client.get_pantry().await?;
    println!("Pantry: {:#?}", res);
```

## Update Pantry Details

To update pantry details, use the UpdatePantryDetails method.

```rust
    let req = pantry_client::UpdatePantryRequest {
        name: "My Pantry".to_string(),
        description: "My Pantry Description".to_string(),
    };
    let res = p.update_pantry_details(req).await.unwrap();
    println!("Pantry: {:#?}", res);
```

## Upset Basket

To update basket, use the UpdateBasket method.

```rust
    let data = serde_json::json!({
        "name": "My Basket",
        "description": "My Basket Description",
    });
    let res = p.upsert_basket("my_basket", data).await?;
    println!("Upsert Basket: {:#?}", res);
```

## Get Basket Contents

To get contents of the basket, use the GetContents method.

```rust
    let res = p.get_basket_content("my_basket").await?;
    println!("Basket Content: {:#?}", res);
```

## Delete Basket

To delete a basket, use the DeleteBasket method.

```rust
    let del = p.delete_basket("my_basket_2").await?;
    println!("Delete Basket 2: {:#?}", del);
```

## Update Basket Contents

To update basket contents, use the UpdateBasketContents method.

```rust
    let update = serde_json::json!({
        "name": "My Basket",
        "description": "My Basket Description",
        "items": [
            {
                "name": "Item 1",
                "description": "Item 1 Description",
                "quantity": 1,
            },
            {
                "name": "Item 2",
                "description": "Item 2 Description",
                "quantity": 2,
            },
        ],
    });
    let res = p.update_basket_content("my_basket", update).await?;
    println!("Update Basket: {:#?}", res);
```
