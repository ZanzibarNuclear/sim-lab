use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::run(async {

        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("beep"),
                String::from("beep"),
                String::from("beep"),
                String::from("beep"),
            ];
    
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1000)).await;
            }
        });
        
        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        });
        
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("bop"),
                String::from("bop"),
                String::from("bop"),
            ];
    
            for val in vals {
                trpl::sleep(Duration::from_millis(750)).await;
                tx.send(val).unwrap();
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
    })
}