#[tokio::test]
async fn test() {
    let fut1 = async {
        crate::debug!("test [thread {:?}]", std::thread::current().id());
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        crate::debug!("test3 [thread {:?}]", std::thread::current().id());
    };

    let fut2 = async {
        for _ in 0..10 {
            crate::debug!("test 2 [thread {:?}]", std::thread::current().id());
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    };

    let joinhandle =
        tokio::task::spawn_blocking(move || tokio::runtime::Handle::current().block_on(fut2));

    for i in 0..10 {
        crate::debug!(
            "in main task {} [thread {:?}]",
            i,
            std::thread::current().id()
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }

    let _ = tokio::join!(fut1, joinhandle);
}
