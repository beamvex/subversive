#[derive(Clone)]
pub struct Message {
    pub message: String,
}

#[tokio::test]
async fn test() {
    let (tx, mut rx) = tokio::sync::broadcast::channel::<Message>(100);
    let tx2 = tx.clone();
    let fut1 = async move {
        for _ in 0..10 {
            crate::debug!("test");
            let _result = tx.send(Message {
                message: "test".to_string(),
            });
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    };

    let fut2 = async move {
        for _ in 0..10 {
            crate::debug!("test 2");
            let _result = tx2.send(Message {
                message: "test 2".to_string(),
            });
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    };

    let joinhandle =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut2));

    let joinhandle1 =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut1));

    while let Ok(msg) = rx.recv().await {
        let message = &msg.message;
        crate::debug!("received message: {message}");
    }

    let _ = tokio::join!(joinhandle1, joinhandle);
}
