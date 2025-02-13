# Rust and DAPR

### Tecnologias
[![My Skills](https://skillicons.dev/icons?i=rust,redis,dapr)](https://skillicons.dev)

## Site Dapr.io
- [https://dapr.io/](https://dapr.io/)


### Before you run the example make sure local redis state store is running by executing:
```
docker ps
```

1. To run the example we need to first build the examples using the following command:


```bash
cargo build 
```

<!-- END_STEP -->

2. Run the example with dapr using the following command:


```bash
dapr run --app-id=dapr_rust_02 --dapr-grpc-port 3500  cargo run
```

<!-- END_STEP -->

If everything went well you should see the following output along with dapr logs:
```
Starting Dapr Rust application...
Waiting for Dapr sidecar to be ready...
Connected to Dapr successfully.
Successfully saved key 'K1' in store 'statestore'.
Retrieved value: "Hello World"
Successfully deleted key 'K1' from store 'statestore'.
Value was successfully deleted.
```

