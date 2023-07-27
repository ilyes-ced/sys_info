use std::path::Path;

use super::utils::read_file;

#[derive(Debug)]
pub struct User {
    uid: u32,
    gid: u32,
    name: String,
    groups: Vec<String>,
}




pub fn get_users() -> Result<Vec<User>, ()> {

    let mut users: Vec<User> = Vec::new();
    let mut groups: Vec<u32> = Vec::new();


    let string = match read_file(Path::new("/etc/passwd")){
        Ok(s) => s,
        Err(_) => return Err(())
    };

    let users_lines = string.split("\n");

    // root:x:0:0::/root:/bin/bash
    for user in users_lines {
        if user == "" {break}
        let mut user_data = user.split(":");
        let username = user_data.next().unwrap();
        user_data.next();
        // should exist always = unwrap is safe
        let uid = user_data.next().unwrap();
        let gid = user_data.next().unwrap();

        println!("{} ======> {:?} {:?} {:?}" , user, username, uid, gid);
        users.push(
            User{
                uid: get_id(uid),
                gid: get_id(gid),
                name: String::from(username),
                groups: Vec::new(),
            }
        );

    }

    println!("{:#?}", users);

    Ok(users)
}

fn get_id(id: &str) -> u32 {
    id.parse::<u32>().unwrap()
}