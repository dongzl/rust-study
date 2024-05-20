use crypto::digest::Digest;
use crypto::md5::Md5;
use hyper::{body, Body, Client, Method, Request};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time;
use std::time::{SystemTime, UNIX_EPOCH};
use string_builder::Builder;
use tokio::io::AsyncWriteExt;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// async fn hyper_post(domain: &str) -> Result<String, Box<dyn Error>> {
//     let current_time = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//     let formatted_time = chrono::DateTime::<chrono::Local>::from(
//         UNIX_EPOCH + std::time::Duration::from_secs(current_time),
//     )
//     .format("%H%M%Y%m%d")
//     .to_string();
//
//     // println!("current_time========{}", current_time);
//     // println!("formatted_time========{}", formatted_time);
//
//     let mut md5 = Md5::new();
//     let text = format!(
//         "{}{}",
//         "", formatted_time
//     );
//     md5.input_str(&text);
//     // println!("md5==========={}", md5.result_str());
//
//     let mut data = HashMap::new();
//
//     data.insert("domains", vec![domain]);
//
//     let req = Request::builder()
//         .method(Method::POST)
//         .uri("")
//         .header("Content-Type", "application/json; charset=utf-8")
//         .header("appCode", "host-manage")
//         .header("erp", "")
//         .header("timestamp", current_time.to_string())
//         .header("sign", md5.result_str())
//         .body(Body::from(serde_json::to_string(&data).unwrap()))?;
//
//     let client = Client::new();
//     let resp = client.request(req).await?;
//
//     let mut sets: HashSet<String> = HashSet::new();
//     let mut builder = Builder::default();
//     if resp.status().is_success() {
//         let bytes = body::to_bytes(resp.into_body()).await?;
//         // println!("body: {:?}", String::from_utf8(bytes.to_vec()).unwrap());
//         let resp_str = String::from_utf8(bytes.to_vec()).unwrap();
//         let map: Value = serde_json::from_str(resp_str.as_str())
//             .expect(format!("{}{}", "Invalid JSON======", domain).as_str());
//         // let data: &Value = map.get("data").unwrap().get(domain).unwrap().get(0).unwrap();
//         let res_status = map.get("resStatus").unwrap().as_i64().unwrap_or_else(|| 0);
//         if res_status == 200 {
//             let data: &Value = map.get("data").unwrap();
//             if data.is_object() {
//                 let data = data.get(domain).unwrap().as_array().unwrap();
//                 for d in data {
//                     let str = format!(
//                         "{}{}{}",
//                         d.get("domain").unwrap().as_str().unwrap(),
//                         ":",
//                         d.get("port").unwrap().as_str().unwrap()
//                     );
//                     //println!("http data_res format ===========: {}", str);
//                     if !sets.contains(&str) {
//                         sets.insert(str.clone());
//                         builder.append(str.clone());
//                         builder.append("\n");
//                     }
//                 }
//                 Ok(format!("{}{}", "success:", builder.string().unwrap()))
//             } else {
//                 println!("http data is empty ===========: {}", domain);
//                 Ok(format!("{}{}{}", "failure:", domain, "\n"))
//             }
//         } else {
//             println!(
//                 "http res_status error ===========: {}",
//                 format!("{}{}{}", domain, ",", res_status)
//             );
//             Ok(format!("{}{}{}", "failure:", domain, "\n"))
//         }
//     } else {
//         println!("http error ===========: {}", domain);
//         Ok(format!("{}{}{}", "failure:", domain, "\n"))
//     }
// }

async fn hyper_post(domain: &str) -> Result<String, Box<dyn Error>> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let formatted_time = chrono::DateTime::<chrono::Local>::from(
        UNIX_EPOCH + std::time::Duration::from_secs(current_time),
    )
        .format("%H%M%Y%m%d")
        .to_string();

    // println!("current_time========{}", current_time);
    // println!("formatted_time========{}", formatted_time);

    let mut md5 = Md5::new();
    let text = format!(
        "{}{}",
        "", formatted_time
    );
    md5.input_str(&text);
    // println!("md5==========={}", md5.result_str());

    let mut data = HashMap::new();

    data.insert("domains", domain);

    let req = Request::builder()
        .method(Method::POST)
        .uri("")
        .header("Content-Type", "application/json; charset=utf-8")
        .header("appCode", "host-manage")
        .header("erp", "")
        .header("timestamp", current_time.to_string())
        .header("sign", md5.result_str())
        .body(Body::from(serde_json::to_string(&data).unwrap()))?;

    let client = Client::new();
    let resp = client.request(req).await?;

    let mut sets: HashSet<String> = HashSet::new();
    let mut builder = Builder::default();
    if resp.status().is_success() {
        let bytes = body::to_bytes(resp.into_body()).await?;
        println!("body: {:?}", String::from_utf8(bytes.to_vec()).unwrap());
        let resp_str = String::from_utf8(bytes.to_vec()).unwrap();
        let map: Value = serde_json::from_str(resp_str.as_str())
            .expect(format!("{}{}", "Invalid JSON======", domain).as_str());
        // let data: &Value = map.get("data").unwrap().get(domain).unwrap().get(0).unwrap();
        let res_status = map.get("resStatus").unwrap().as_i64().unwrap_or_else(|| 0);
        if res_status == 200 {
            let data: &Value = map.get("data").unwrap();
            if data.is_object() {
                let data = data.get("list").unwrap().as_array().unwrap();
                for d in data {
                    let str = format!(
                        "{}{}{}",
                        d.get("domain").unwrap().as_str().unwrap(),
                        ":",
                        d.get("port").unwrap().as_str().unwrap()
                    );
                    //println!("http data_res format ===========: {}", str);
                    if !sets.contains(&str) {
                        sets.insert(str.clone());
                        builder.append(str.clone());
                        builder.append("\n");
                    }
                }
                Ok(format!("{}{}", "success:", builder.string().unwrap()))
            } else {
                println!("http data is empty ===========: {}", domain);
                Ok(format!("{}{}{}", "failure:", domain, "\n"))
            }
        } else {
            println!(
                "http res_status error ===========: {}",
                format!("{}{}{}", domain, ",", res_status)
            );
            Ok(format!("{}{}{}", "failure:", domain, "\n"))
        }
    } else {
        println!("http error ===========: {}", domain);
        Ok(format!("{}{}{}", "failure:", domain, "\n"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("http pull domain port execute start ===========");
    let ten_millis = time::Duration::from_millis(500);
    let mut success_file = tokio::fs::File::create("success_domains_result.txt").await.unwrap();
    let mut failure_file = tokio::fs::File::create("failure_domains_result.txt").await.unwrap();
    if let Ok(lines) = read_lines("./domains.txt") {
        for line in lines {
            if let Ok(domain) = line {
                thread::sleep(ten_millis);
                let res = hyper_post(domain.as_str())
                    .await
                    .expect(format!("{}{}", "TODO: panic message domain=", domain).as_str());
                // println!("res=========={:?}", res);
                if res.starts_with("success") {
                    success_file.write(res[8..].as_bytes()).await.unwrap();
                } else {
                    failure_file.write(res[8..].as_bytes()).await.unwrap();
                }
            }
        }
    }
    println!("http pull domain port execute end ===========");
    Ok(())
}
