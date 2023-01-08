extern crate shamir;
extern crate base64;

use shamir::SecretData;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![do_split, do_recover])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn do_split(secret: &str) -> Vec<String> {
    let secret_data = SecretData::with_secret(&secret, 2);

    let mut share_1_b64: String = String::from("");
    let mut share_2_b64: String = String::from("");
    let mut share_3_b64: String = String::from("");

    let share1 = secret_data.get_share(1);
    if share1.is_ok() {
        share_1_b64 = base64::encode(share1.unwrap());
    }
    let share2 = secret_data.get_share(2);
    if share2.is_ok() {
        share_2_b64 = base64::encode(share2.unwrap());
    }
    let share3 = secret_data.get_share(3);
    if share3.is_ok() {
        share_3_b64 = base64::encode(share3.unwrap());
    }

    vec![share_1_b64, share_2_b64, share_3_b64]
}

#[tauri::command]
fn do_recover(share1b64: &str, share2b64: &str) -> String {
    // decode base64 shares
    let share1 = base64::decode(share1b64);
    let share2 = base64::decode(share2b64);

    // unwrap shares
    let share1 = share1.unwrap_or(vec![]);
    let share2 = share2.unwrap_or(vec![]);

    if share1.len() == 0 || share2.len() == 0 {
        return String::from("");
    }
    
    let recovered = SecretData::recover_secret(2, vec![share1, share2]);

    // unwrap but fallback to empty string if error
    recovered.unwrap_or(String::from(""))
}
