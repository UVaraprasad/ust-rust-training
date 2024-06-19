use tokio::time::sleep;
use tokio::time::Duration;
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    sleep(Duration::from_secs(3)).await;

    let k=add(10,20).await;
    println!("main finishes its execution");

    let t= tokio::spawn(async{
        let k = add(100,30).await;
        println!("{k}");
    });

    let t1 = tokio::spawn(async{

        for i in 1..10{
            println!("coroutine1-{i}");
            sleep(Duration::from_millis(300)).await;
        }
    });
    let t2 = tokio::spawn(async{
        for i in 1..10{
            println!("coroutine2-{i}");
            sleep(Duration::from_millis(200)).await;
        }
    });

    t.await;
    t1.await;
    t2.await;

}

async fn add(i:i32,j:i32)->i32{
    i+j
}
