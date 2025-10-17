# Rust Reverse Proxy with Backend Servers

This project demonstrates a simple **reverse proxy** in Rust using **Tokio**, along with two backend servers: a **User Server** and an **Order Server**. The reverse proxy forwards requests to the appropriate backend based on the request path.

---

## Project Structure

```
rust-reverse-proxy/
├─ Cargo.toml
└─ src/
   ├─ main.rs         <-- Reverse proxy
   ├─ user_server.rs  <-- Users API
   └─ order_server.rs <-- Orders API
```

---

## Servers

| Server             | Address                  | Notes                                |
|------------------|-------------------------|--------------------------------------|
| **User Server**   | `127.0.0.1:3001/users`  | Returns a JSON list of users         |
| **Order Server**  | `127.0.0.1:3002/orders` | Returns a JSON list of orders        |
| **Reverse Proxy** | `127.0.0.1:3000`        | Forwards requests to backend servers |

---

## Proxy Routing

| Path      | Forwarded To      |
|----------|-----------------|
| `/users` | `127.0.0.1:3001` |
| `/orders`| `127.0.0.1:3002` |

---

## How to Run

1. **Clone the repository**

```bash
git clone <repo_url>
cd rust-reverse-proxy
```

2. **Run the project**

```bash
cargo run
```

> This will start:
> - User Server on port 3001
> - Order Server on port 3002
> - Reverse Proxy on port 3000

3. **Test the APIs**

```bash
# Through reverse proxy
curl http://127.0.0.1:3000/users
curl http://127.0.0.1:3000/orders

# Directly to backend servers
curl http://127.0.0.1:3001/users
curl http://127.0.0.1:3002/orders
```

---

## Example Responses

**Users API**

```json
[
  { "id": 1, "name": "Alice" },
  { "id": 2, "name": "Bob" }
]
```

**Orders API**

```json
[
  { "id": 101, "item": "Book" },
  { "id": 102, "item": "Laptop" }
]
```

---

## Notes

- Implemented using **Tokio TCP listeners** (no Hyper or external HTTP frameworks).  
- Reverse proxy performs **basic routing** and forwards requests based on path.  
- Can be extended to handle **POST requests, headers, streaming**, and other HTTP features.
