#[derive(Clone)]
pub struct Message {
    pub message: String,
}

#[tokio::test]
async fn test() {
    let (tx, rx) = tokio::sync::broadcast::channel::<Message>(1);

    let fut1 = async move {
        crate::debug!("test");
        let _result = tx.send(Message {
            message: "test".to_string(),
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        crate::debug!("test3");
    };

    let fut2 = async move {
        for _ in 0..10 {
            crate::debug!("test 2");
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    };

    let joinhandle =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut2));

    for i in 0..10 {
        crate::debug!("in main task {i}");
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    let _ = tokio::join!(fut1, joinhandle);
}
