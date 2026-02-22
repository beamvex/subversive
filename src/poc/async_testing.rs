#[derive(Clone)]
/// A test harness for asynchronous code.
///
/// This struct provides a way to test asynchronous code by managing
/// a broadcast channel for communication between tasks.
#[derive(Debug)]
pub struct AsyncTest {
    /// The sender side of the broadcast channel
    tx: tokio::sync::broadcast::Sender<String>,
}

impl AsyncTest {
    /// Creates a new `AsyncTest` instance.
    #[must_use]
    pub fn new() -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel::<String>(100);
        Self { tx }
    }
}

impl Default for AsyncTest {
    fn default() -> Self {
        Self::new()
    }
}

/// A message that can be sent through the broadcast channel.
///
/// This struct represents a message that can be sent between tasks
/// using the broadcast channel.
#[derive(Clone)]
pub struct Message {
    /// The message content
    pub message: String,
}

/// Tests asynchronous communication between tasks.
///
/// This test demonstrates how to use broadcast channels for communication
/// between multiple asynchronous tasks.
#[tokio::test]
async fn test() {
    let (tx, mut rx) = tokio::sync::broadcast::channel::<Message>(100);
    let tx2 = tx.clone();
    let fut1 = async move {
        for _ in 0..10 {
            slogger::debug!("test");
            let _result = tx.send(Message {
                message: "test".to_string(),
            });
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    };

    let fut2 = async move {
        for _ in 0..10 {
            slogger::debug!("test 2");
            let _result = tx2.send(Message {
                message: "test 2".to_string(),
            });
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    };

    let joinhandle =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut2));

    let joinhandle1 =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut1));

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    while let Ok(msg) = rx.recv().await {
        let message = &msg.message;
        slogger::debug!("received message: {message}");
    }

    let _ = tokio::join!(joinhandle1, joinhandle);
}
