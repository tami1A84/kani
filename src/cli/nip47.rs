use clap::Parser;
use nostr_sdk::prelude::*;
use nostr::nips::nip47::{Request, NostrWalletConnectURI, Response, PayInvoiceRequest};
use tokio::time::{timeout, Duration};

#[derive(Parser)]
pub struct Nip47Command {
    /// Nostr Wallet Connect (NIP-47)
    #[command(subcommand)]
    subcommand: Nip47Subcommand,
}

#[derive(Parser)]
pub enum Nip47Subcommand {
    /// Get info from a wallet
    GetInfo {
        /// Wallet Connect URI (nostr+walletconnect://...)
        uri: String,
    },
    /// Get balance from a wallet
    GetBalance {
        /// Wallet Connect URI (nostr+walletconnect://...)
        uri: String,
    },
    /// Pay an invoice
    PayInvoice {
        /// Wallet Connect URI (nostr+walletconnect://...)
        uri: String,
        /// Bolt11 invoice
        invoice: String,
    }
}

pub async fn handle_nip47_command(command: Nip47Command) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        Nip47Subcommand::GetInfo { uri } => {
            let nwc_uri = NostrWalletConnectURI::parse(&uri)?;

            let request = Request::get_info();
            let event = request.to_event(&nwc_uri)?;

            let keys = Keys::new(nwc_uri.secret.clone());
            let client = Client::new(keys);

            for relay in nwc_uri.relays.iter() {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            let event_id = client.send_event(&event).await?;
            println!("GetInfo request sent with id: {}", event_id.to_bech32()?);

            // Handle response
            let filter = Filter::new()
                .kind(Kind::WalletConnectResponse)
                .author(nwc_uri.public_key)
                .event(*event_id)
                .since(Timestamp::now());

            client.subscribe(filter, None).await?;

            println!("Waiting for response...");

            let mut notifications = client.notifications();

            let fut = async {
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        if event.kind == Kind::WalletConnectResponse {
                            if let Ok(response) = Response::from_event(&nwc_uri, &event) {
                                match response.to_get_info() {
                                    Ok(info) => return Some(Ok(info)),
                                    Err(e) => return Some(Err(e.to_string())),
                                }
                            }
                        }
                    }
                }
                None
            };

            match timeout(Duration::from_secs(30), fut).await {
                Ok(Some(Ok(info))) => println!("Received info: {:#?}", info),
                Ok(Some(Err(e))) => println!("Error from wallet: {}", e),
                _ => println!("Timeout or no response."),
            }

            client.shutdown().await;
        }
        Nip47Subcommand::GetBalance { uri } => {
            let nwc_uri = NostrWalletConnectURI::parse(&uri)?;

            let request = Request::get_balance();
            let event = request.to_event(&nwc_uri)?;

            let keys = Keys::new(nwc_uri.secret.clone());
            let client = Client::new(keys);

            for relay in nwc_uri.relays.iter() {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            let event_id = client.send_event(&event).await?;
            println!("GetBalance request sent with id: {}", event_id.to_bech32()?);

            // Handle response
            let filter = Filter::new()
                .kind(Kind::WalletConnectResponse)
                .author(nwc_uri.public_key)
                .event(*event_id)
                .since(Timestamp::now());

            client.subscribe(filter, None).await?;

            println!("Waiting for response...");

            let mut notifications = client.notifications();

            let fut = async {
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        if event.kind == Kind::WalletConnectResponse {
                            if let Ok(response) = Response::from_event(&nwc_uri, &event) {
                                match response.to_get_balance() {
                                    Ok(balance) => return Some(Ok(balance)),
                                    Err(e) => return Some(Err(e.to_string())),
                                }
                            }
                        }
                    }
                }
                None
            };

            match timeout(Duration::from_secs(30), fut).await {
                Ok(Some(Ok(balance))) => println!("Received balance: {} sats", balance.balance / 1000),
                Ok(Some(Err(e))) => println!("Error from wallet: {}", e),
                _ => println!("Timeout or no response."),
            }

            client.shutdown().await;
        }
        Nip47Subcommand::PayInvoice { uri, invoice } => {
            let nwc_uri = NostrWalletConnectURI::parse(&uri)?;

            let params = PayInvoiceRequest {
                id: None,
                invoice,
                amount: None,
            };
            let request = Request::pay_invoice(params);
            let event = request.to_event(&nwc_uri)?;

            let keys = Keys::new(nwc_uri.secret.clone());
            let client = Client::new(keys);

            for relay in nwc_uri.relays.iter() {
                client.add_relay(relay.clone()).await?;
            }
            client.connect().await;

            let event_id = client.send_event(&event).await?;
            println!("PayInvoice request sent with id: {}", event_id.to_bech32()?);

            // Handle response
            let filter = Filter::new()
                .kind(Kind::WalletConnectResponse)
                .author(nwc_uri.public_key)
                .event(*event_id)
                .since(Timestamp::now());

            client.subscribe(filter, None).await?;

            println!("Waiting for response...");

            let mut notifications = client.notifications();

            let fut = async {
                while let Ok(notification) = notifications.recv().await {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        if event.kind == Kind::WalletConnectResponse {
                            if let Ok(response) = Response::from_event(&nwc_uri, &event) {
                                match response.to_pay_invoice() {
                                    Ok(res) => return Some(Ok(res)),
                                    Err(e) => return Some(Err(e.to_string())),
                                }
                            }
                        }
                    }
                }
                None
            };

            match timeout(Duration::from_secs(30), fut).await {
                Ok(Some(Ok(res))) => println!("Invoice paid! Preimage: {}", res.preimage),
                Ok(Some(Err(e))) => println!("Error from wallet: {}", e),
                _ => println!("Timeout or no response."),
            }

            client.shutdown().await;
        }
    }
    Ok(())
}
