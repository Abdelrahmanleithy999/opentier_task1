use embedded_recruitment_task::{
    message::{client_message, server_message, AddRequest, EchoMessage},
    server::Server,
};
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

mod client;

fn create_server(port: u16) -> Arc<Server> {
  Arc::new(Server::new(&format!("localhost:{}", port)).expect("Failed to start server"))
}

fn setup_server_thread(server: Arc<Server>) -> thread::JoinHandle<()> {
  thread::spawn(move || {
      server.run().expect("Server failed to run");
  })
}

#[test]
fn test_client_connection() {
  // Set up the server in a separate thread
  let server = create_server(8081);
  let handle = setup_server_thread(server.clone());

  // Create and connect the client
  let mut client = client::Client::new("localhost", 8081, 1000);
  assert!(client.connect().is_ok(), "Failed to connect to the server");

  // Disconnect the client
  assert!(
      client.disconnect().is_ok(),
      "Failed to disconnect from the server"
  );

  // Stop the server and wait for thread to finish
  server.stop();
  assert!(
      handle.join().is_ok(),
      "Server thread panicked or failed to join"
  );
}

#[test]
fn test_client_echo_message() {
  // Set up the server in a separate thread
  let server = create_server(8082);
  let handle = setup_server_thread(server.clone());

  // Create and connect the client
  let mut client = client::Client::new("localhost", 8082, 1000);
  assert!(client.connect().is_ok(), "Failed to connect to the server");

  // Prepare the message
  let mut echo_message = EchoMessage::default();
  echo_message.content = "Hello, World!".to_string();
  let message = client_message::Message::EchoMessage(echo_message.clone());

  // Send the message to the server
  assert!(client.send(message).is_ok(), "Failed to send message");

  // Receive the echoed message
  let response = client.receive();
  assert!(
      response.is_ok(),
      "Failed to receive response for EchoMessage"
  );

  match response.unwrap().message {
      Some(server_message::Message::EchoMessage(echo)) => {
          assert_eq!(
              echo.content, echo_message.content,
              "Echoed message content does not match"
          );
      }
      _ => panic!("Expected EchoMessage, but received a different message"),
  }

  // Disconnect the client
  assert!(
      client.disconnect().is_ok(),
      "Failed to disconnect from the server"
  );

  // Stop the server and wait for thread to finish
  server.stop();
  assert!(
      handle.join().is_ok(),
      "Server thread panicked or failed to join"
  );
}