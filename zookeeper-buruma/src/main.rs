extern crate buruma;
use buruma::ZooKeeper;
use buruma::ZKResult;
use std::time::Duration;
use buruma::CreateMode;
use buruma::ACL;

#[tokio::main]
async fn main() -> ZKResult<()> {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await.unwrap();
    let basic_path = "/buruma";
    // 创建节点
    let path = zk
        .create(basic_path, Some("buruma".as_bytes()), ACL::world_acl(), CreateMode::Persistent)
        .await
        .unwrap();
    // 查询节点
    let result = zk.get(basic_path, None).await.unwrap();
    // 设置节点数据
    let stat = zk.set(basic_path, "kaixinbaba".as_bytes()).await.unwrap();
    // 删除节点
    zk.delete(basic_path);
    Ok(())
}