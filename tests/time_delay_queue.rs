use futures_util::StreamExt;
use tokio::time::Duration;
use tokio_util::time::DelayQueue;

#[tokio::test]
async fn poll_task_delay_time() {
    let task = async {
        println!("Task running at {}", chrono::Local::now());
    };

    let mut queue = DelayQueue::new();

    queue.insert(task, Duration::from_secs(5));

    println!("Task insert at {}", chrono::Local::now());

    tokio::spawn(async move {
        while let Some(task) = queue.next().await {
            task.into_inner().await;
        }
    });

    tokio::time::sleep(Duration::from_secs(10)).await;

    println!("Thread quit at {}", chrono::Local::now());
}
