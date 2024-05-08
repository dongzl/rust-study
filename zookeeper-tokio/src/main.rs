use std::rc::Rc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_zookeeper::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let (zk, _) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
        .await
        .unwrap();
    let zk = Rc::new(zk);
    export(
        Rc::clone(&zk),
        "/governance_ds/metadata/sys/schemas/sys/tables",
        "database/sys/",
    )
    .await;
    export(
        Rc::clone(&zk),
        "/governance_ds/metadata/performance_schema/schemas/performance_schema/tables",
        "database/performance_schema/",
    )
    .await;
    export(
        Rc::clone(&zk),
        "/governance_ds/metadata/mysql/schemas/mysql/tables",
        "database/mysql/",
    )
    .await;
    export(
        Rc::clone(&zk),
        "/governance_ds/metadata/information_schema/schemas/information_schema/tables",
        "database/information_schema/",
    )
    .await;
}

async fn export(zk: Rc<ZooKeeper>, path: &str, export: &str) {
    let children = zk.get_children(path).await.unwrap();

    // println!("children {:?}", children);

    for child in children.unwrap().iter() {
        println!("Child {:?}", child);
        let path = format!("{}{}{}{}", path, "/", child, "/versions/0");
        println!("Path {}", path);
        let data = zk.get_data(&path).await.unwrap();
        let data = data.unwrap();
        // println!("Data {:?}", data.0);
        let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
        // println!("{}", s);

        let mut file = File::create(format!("{}{}{}", export, child, ".yaml"))
            .await
            .unwrap();
        file.write(s.as_bytes()).await.unwrap();
    }
}
