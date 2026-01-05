use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct ShaMainConfig {
    
}

pub fn validate_main_config(file_path: String) {

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
