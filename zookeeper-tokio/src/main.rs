use std::io::Write;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_zookeeper::*;
//use futures::prelude::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
    //     .await
    //     .unwrap();
    // let stat = zk.watch().exists("/example").await.unwrap();
    // assert_eq!(stat, None);
    // let path = zk
    //     .create(
    //         "/example",
    //         &b"Hello world"[..],
    //         Acl::open_unsafe(),
    //         CreateMode::Persistent,
    //     )
    //     .await
    //     .unwrap();
    // assert_eq!(path.as_deref(), Ok("/example"));

    // let children = zk.get_children("/governance_ds/metadata/sys/schemas/sys/tables").await.unwrap();
    //
    // //println!("children {:?}", children);
    //
    // for child in children.unwrap().iter() {
    //     println!("Child {:?}", child);
    //     let path = format!("{}{}{}", "/governance_ds/metadata/sys/schemas/sys/tables/", child, "/versions/0");
    //     println!("Path {}", path);
    //     let data = zk.get_data(&path).await.unwrap();
    //     let data = data.unwrap();
    //     //println!("Data {:?}", data.0);
    //     let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
    //     //println!("{}", s);
    //
    //
    //     let mut file = File::create(format!("{}{}{}", "sys/", child, ".yaml")).await.unwrap();
    //     file.write(s.as_bytes()).await.unwrap();
    // }

    export_sys().await;
    export_performance_schema().await;
    export_mysql().await;
    export_information_schema().await;
}

async fn export_sys() {
    let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
        .await
        .unwrap();
    let children = zk
        .get_children("/governance_ds/metadata/sys/schemas/sys/tables")
        .await
        .unwrap();

    //println!("children {:?}", children);

    for child in children.unwrap().iter() {
        println!("Child {:?}", child);
        let path = format!(
            "{}{}{}",
            "/governance_ds/metadata/sys/schemas/sys/tables/", child, "/versions/0"
        );
        println!("Path {}", path);
        let data = zk.get_data(&path).await.unwrap();
        let data = data.unwrap();
        //println!("Data {:?}", data.0);
        let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
        //println!("{}", s);

        let mut file = File::create(format!("{}{}{}", "database/sys/", child, ".yaml"))
            .await
            .unwrap();
        file.write(s.as_bytes()).await.unwrap();
    }
}

async fn export_performance_schema() {
    let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
        .await
        .unwrap();
    let children = zk
        .get_children(
            "/governance_ds/metadata/performance_schema/schemas/performance_schema/tables",
        )
        .await
        .unwrap();

    //println!("children {:?}", children);

    for child in children.unwrap().iter() {
        println!("Child {:?}", child);
        let path = format!(
            "{}{}{}",
            "/governance_ds/metadata/performance_schema/schemas/performance_schema/tables/",
            child,
            "/versions/0"
        );
        println!("Path {}", path);
        let data = zk.get_data(&path).await.unwrap();
        let data = data.unwrap();
        //println!("Data {:?}", data.0);
        let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
        //println!("{}", s);

        let mut file = File::create(format!(
            "{}{}{}",
            "database/performance_schema/", child, ".yaml"
        ))
        .await
        .unwrap();
        file.write(s.as_bytes()).await.unwrap();
    }
}

async fn export_mysql() {
    let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
        .await
        .unwrap();
    let children = zk
        .get_children("/governance_ds/metadata/mysql/schemas/mysql/tables")
        .await
        .unwrap();

    //println!("children {:?}", children);

    for child in children.unwrap().iter() {
        println!("Child {:?}", child);
        let path = format!(
            "{}{}{}",
            "/governance_ds/metadata/mysql/schemas/mysql/tables/", child, "/versions/0"
        );
        println!("Path {}", path);
        let data = zk.get_data(&path).await.unwrap();
        let data = data.unwrap();
        //println!("Data {:?}", data.0);
        let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
        //println!("{}", s);

        let mut file = File::create(format!("{}{}{}", "database/mysql/", child, ".yaml"))
            .await
            .unwrap();
        file.write(s.as_bytes()).await.unwrap();
    }
}

async fn export_information_schema() {
    let (zk, default_watcher) = ZooKeeper::connect(&"127.0.0.1:2181".parse().unwrap())
        .await
        .unwrap();
    let children = zk
        .get_children(
            "/governance_ds/metadata/information_schema/schemas/information_schema/tables",
        )
        .await
        .unwrap();

    //println!("children {:?}", children);

    for child in children.unwrap().iter() {
        println!("Child {:?}", child);
        let path = format!(
            "{}{}{}",
            "/governance_ds/metadata/information_schema/schemas/information_schema/tables/",
            child,
            "/versions/0"
        );
        println!("Path {}", path);
        let data = zk.get_data(&path).await.unwrap();
        let data = data.unwrap();
        //println!("Data {:?}", data.0);
        let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
        //println!("{}", s);

        let mut file = File::create(format!(
            "{}{}{}",
            "database/information_schema/", child, ".yaml"
        ))
        .await
        .unwrap();
        file.write(s.as_bytes()).await.unwrap();
    }
}

// async fn export(conn: ZooKeeper, path: String, dir: String) {
//     let children = conn.get_children("/governance_ds/metadata/sys/schemas/sys/tables").await.unwrap();
//
//     //println!("children {:?}", children);
//
//     for child in children.unwrap().iter() {
//         // println!("Child {:?}", child);
//         // let path = format!("{}{}{}", "/governance_ds/metadata/sys/schemas/sys/tables/", child, "/versions/0");
//         // println!("Path {}", path);
//         // let data = zk.get_data(&path).await.unwrap();
//         // let data = data.unwrap();
//         // //println!("Data {:?}", data.0);
//         // let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
//         // println!("{}", s);
//         write(conn, path, child, dir);
//     }
// }
//
// async fn write(conn: ZooKeeper, path: String, child: &String, dir: String) {
//     println!("Child {:?}", child);
//     let path = format!("{}{}{}", path, child, "/versions/0");
//     println!("Path {}", path);
//     let data = conn.get_data(&path).await.unwrap();
//     let data = data.unwrap();
//     //println!("Data {:?}", data.0);
//     let s = String::from_utf8(data.0).expect("Found invalid UTF-8");
//     println!("{}", s);
// }
