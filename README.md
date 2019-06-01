# Simple auth server

Following the tutorial Auth Web Microservice with Rust using actix-web
https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-2/

## Install Postgres

```
brew install postgres
rm -r /usr/local/var/postgres
initdb /usr/local/var/postgres
pg_ctl -D /usr/local/var/postgres -l logfile start
createdb postres_test
```
