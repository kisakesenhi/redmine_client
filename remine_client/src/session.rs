//! Session is the module to keep the session info.
//!
//! # Keeps Redmine URL and API Key
//!
//! Information stored on a local os specific config file.
//! Config file stored encrypted json format.
//! While compiling please set the magic key.
use magic_crypt::MagicCryptTrait;
use std::io::{stdin,stdout,Write};
use std::env::current_exe;
use dirs::config_dir; // this will provide us the config dir
use std::path::{PathBuf};
use serde_json::Value;
//use std::process;
use mac_address;

/// MagicCrypt Object encryption instance. Modify magickey while compiling.
fn mcinstance()-> magic_crypt::MagicCrypt256{
    // Add mac adress in addition to magic word to make is computer specific
    // TODO Receive Magic key from Config file
    let mut magickey="magickey".to_string();
    if let Some(macadress)=mac_address::get_mac_address().unwrap(){
        magickey+= &macadress.to_string();
    }
    new_magic_crypt!(magickey,256)}
/// Encryipt the session, session is serde_json:Value reference.
pub fn encryptsession(session: &Value )->String{
    let mc = mcinstance();
    let session_pretty_string=serde_json::to_string_pretty(&session).unwrap();
    mc.encrypt_str_to_base64(&session_pretty_string)

}
/// Decrypts the session info. Session is read from config file as String.
pub fn decryptsession(session: String )->Value {
    let mc = mcinstance();
    let session=mc.decrypt_base64_to_string(&session).unwrap();
    serde_json::from_str(&session).unwrap()
}

/// Get Os specific config file location.
pub fn getconfig()->PathBuf{
    // This function return the config directory!!!
    // Get file name from current exe stem
    let currentexe=current_exe().unwrap();
    let executable=currentexe.file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Get the system config directry and create a subdirectory
    let confdir=config_dir().unwrap();
    let mut confpath= PathBuf::from(confdir);
    confpath.push(executable);
    confpath.push("config");
    confpath.set_extension("json");
    confpath
}
///Registers the url and api key. FYI Not validates whether URL and API keys are corect!!
///Checks will be implemented later!!!
pub fn registerapp (){
    // ask user the adress
    let mut adress = String::new();
    print!("Enter Adress:");
    let _=stdout().flush();
    stdin().read_line(&mut adress).expect("Did not enter a correct string");
    //check if adress exist

    // ask the API Key
    print!("Enter API Key:");
    let _=stdout().flush();
    let mut apikey = String::new();
    stdin().read_line(&mut apikey).expect("Did not enter a correct API key");
    
    //Create a parent directory for config
    let confpath=getconfig();
    let confdir=confpath.parent().unwrap();
    //create directory
    std::fs::create_dir_all(confdir).expect("Can't create the config directory!");

    //Create a json array with session values and write it to session
    //json{ adress:encrypted , apikey: enctrypted }
    let session = json!({"url":adress.trim(), "apikey":apikey.trim()});
    std::fs::write(getconfig(),encryptsession(&session)).expect("Can't write to the config file !!!");
}
/// Read the session info from file, decrypt and return as serde_json::Value object.
pub fn getsessioninfo()->serde_json::Value{
    // Get config file
   match std::fs::read_to_string(getconfig()){
    Ok(s) =>{
        return json!(decryptsession(s));
        },
    Err(_) => {
        println!("Registration required!");
        registerapp();
        return getsessioninfo()
        }

   }
}
