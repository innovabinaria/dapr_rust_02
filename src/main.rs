use dapr;
use anyhow::{Context, Result};
use log::{info, error};
use std::time::Duration;
use tokio::time::sleep;

// Constantes para evitar "magic numbers" y strings repetidos
const DAPR_ADDR: &str = "https://127.0.0.1";
const STATE_STORE_NAME: &str = "statestore";
const KEY: &str = "K1";
const INITIAL_DELAY_SECONDS: u64 = 2;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar el logger
    env_logger::init();

    info!("Starting Dapr Rust application...");

    // Esperar a que Dapr esté listo (reemplazo del sleep)
    info!("Waiting for Dapr sidecar to be ready...");
    sleep(Duration::from_secs(INITIAL_DELAY_SECONDS)).await;

    // Conectar al cliente de Dapr
    let mut client = dapr::Client::<dapr::client::TonicClient>::connect(DAPR_ADDR.to_string())
        .await
        .context("Failed to connect to Dapr")?;
    info!("Connected to Dapr successfully.");

    // Guardar un valor en el state store
    let value = "Hello World".as_bytes().to_vec();
    save_state(&mut client, STATE_STORE_NAME, KEY, &value).await?;

    // Recuperar el valor del state store
    let retrieved_value = get_state(&mut client, STATE_STORE_NAME, KEY).await?;
    info!("Retrieved value: {:?}", String::from_utf8_lossy(&retrieved_value));

    // Eliminar el valor del state store
    delete_state(&mut client, STATE_STORE_NAME, KEY).await?;

    // Validar que el valor fue eliminado
    let deleted_value = get_state(&mut client, STATE_STORE_NAME, KEY).await?;
    if deleted_value.is_empty() {
        info!("Value was successfully deleted.");
    } else {
        error!("Failed to delete value: {:?}", deleted_value);
    }

    Ok(())
}

/// Guarda un valor en el state store de Dapr.
async fn save_state(
    client: &mut dapr::Client<dapr::client::TonicClient>,
    store_name: &str,
    key: &str,
    value: &[u8],
) -> Result<()> {
    client
        .save_state(store_name.to_string(), vec![(key.to_string(), value.to_vec())])
        .await
        .context("Failed to save state")?;
    info!("Successfully saved key '{}' in store '{}'.", key, store_name);
    Ok(())
}

/// Recupera un valor del state store de Dapr.
async fn get_state(
    client: &mut dapr::Client<dapr::client::TonicClient>,
    store_name: &str,
    key: &str,
) -> Result<Vec<u8>> {
    let response = client
        .get_state(store_name, key, None)
        .await
        .context("Failed to retrieve state")?;
    Ok(response.data)
}

/// Elimina un valor del state store de Dapr.
async fn delete_state(
    client: &mut dapr::Client<dapr::client::TonicClient>,
    store_name: &str,
    key: &str,
) -> Result<()> {
    client
        .delete_state(store_name, key, None)
        .await
        .context("Failed to delete state")?;
    info!("Successfully deleted key '{}' from store '{}'.", key, store_name);
    Ok(())
}
