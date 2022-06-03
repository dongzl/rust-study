use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone)]
struct Tenant {
    name: String,
    users: HashMap<String, User>,
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    dept: String,
}

struct TenantManagerProvider {
    tenants: RwLock<HashMap<String, Tenant>>,
}

trait TenantManager {
    fn remove_user(&mut self, tenant: String, username: String);
}

impl TenantManager for TenantManagerProvider {
    fn remove_user(&mut self, tenant: String, username: String) {
        let mut tenant_map = self.tenants.write().unwrap();
        //let mut tenant_map = self.lock.get_mut().unwrap();
        if !tenant_map.contains_key(&tenant) {
            println!("not contain {}, {}", tenant, username);
            return;
        }
        let tenant = tenant_map.get_mut(&tenant).unwrap();

        //tenant.clone().users compile is OK, but not remove
        println!("before remove {}", tenant.users.len());
        let user_map = &mut tenant.users;
        user_map.remove(&username);
        println!("after remove {}", user_map.len());
        println!("after remove {}", tenant.users.len());

        // tenant.users compile is failure
        // let mut user_map = tenant.users;
        // println!("before remove {}", tenant.users.len());
        // println!("before remove {}", user_map.len());
        // user_map.remove(username.as_str());
        // println!("after remove {}", user_map.len());
        // println!("after remove {}", tenant.users.len());
    }
}

fn main() {
    let u1 = User {
        name: String::from("a"),
        dept: String::from("a"),
    };

    let u2 = User {
        name: String::from("b"),
        dept: String::from("b"),
    };

    let mut user_map: HashMap<String, User> = HashMap::new();
    user_map.insert(u1.name.clone(), u1);
    user_map.insert(u2.name.clone(), u2);

    let tenant = Tenant {
        name: String::from("t"),
        users: user_map,
    };

    let mut tenant_map: HashMap<String, Tenant> = HashMap::new();
    tenant_map.insert(tenant.name.clone(), tenant);

    let mut manager = TenantManagerProvider {
        tenants: RwLock::from(tenant_map),
    };

    println!("{}", manager.tenants.read().unwrap().get("t").unwrap().users.len());
    manager.remove_user(String::from("t"), String::from("a"));
    println!("{}", manager.tenants.read().unwrap().get("t").unwrap().users.len());
}
