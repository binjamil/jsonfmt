# jsonfmt 

jsonfmt is a simple CLI app written in Rust that pretty-prints JSON to stdout.

## Installation

To install jsonfmt, you first need to [install Rust](https://www.rust-lang.org/tools/install).

```sh
git clone https://github.com/binjamil/jsonfmt.git 
cd jsonfmt
cargo install --path .
```

## Usage 

I usually use jsonfmt to pretty print the output from a cURL request. For example

```sh
➜ curl https://dummyjson.com/products/1
{"id":1,"title":"iPhone 9","description":"An apple mobile which is nothing like apple","price":549,"discountPercentage":12.96,"rating":4.69,"stock":94,"brand":"Apple","category":"smartphones","thumbnail":"https://dummyjson.com/image/i/products/1/thumbnail.jpg","images":["https://dummyjson.com/image/i/products/1/1.jpg","https://dummyjson.com/image/i/products/1/2.jpg","https://dummyjson.com/image/i/products/1/3.jpg","https://dummyjson.com/image/i/products/1/4.jpg","https://dummyjson.com/image/i/products/1/thumbnail.jpg"]}
```

Using jsonfmt can format output JSON to a more human-readable form

```sh
➜ curl https://dummyjson.com/products/1 | jsonfmt
{
  "id": 1,
  "title": "iPhone 9",
  "description": "An apple mobile which is nothing like apple",
  "price": 549,
  "discountPercentage": 12.96,
  "rating": 4.69,
  "stock": 94,
  "brand": "Apple",
  "category": "smartphones",
  "thumbnail": "https://dummyjson.com/image/i/products/1/thumbnail.jpg",
  "images": [
    "https://dummyjson.com/image/i/products/1/1.jpg",
    "https://dummyjson.com/image/i/products/1/2.jpg",
    "https://dummyjson.com/image/i/products/1/3.jpg",
    "https://dummyjson.com/image/i/products/1/4.jpg",
    "https://dummyjson.com/image/i/products/1/thumbnail.jpg"
  ]
}
```