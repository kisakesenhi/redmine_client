//! # File Operations
//!
//! This module is to hold file operations uploads // and downloads
//!

use serde_json;
//use reqwest::header;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
//use crate::session::*;
use crate::session::getsessioninfo;
use crate::requests::*;
use crate::requests::builduploadclient;
use reqwest::header::{HeaderValue, CONTENT_LENGTH};
use std::str::FromStr;

// furures = "0.3"
//use futures::stream::TryStreamExt;
// reqwest ={version = "0.11", features= ["stream"] }

//use tokio_util::codec::{BytesCodec, FramedRead};

//use crate::requests::file_to_body2;

pub fn file_to_body(infile:&String)->Vec<u8>{
    let mut f = std::fs::File::open(&infile).expect("no file found");
    let metadata = std::fs::metadata(&infile).expect("unable to read metadata");
    let mut buffer = vec![0;metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

/// This will upload the files.
#[derive(Debug,Serialize,Deserialize)]
pub struct Upload {
    token: String,
    filename: String,
    description: String,
    content_type: String,
}
#[tokio::main]
pub async fn upload_file(path: String, filename: String)->Result<serde_json::Value, Box<dyn std::error::Error>>{
    // This will get the issue with the correct number
    // First get the build the client from session info
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client =builduploadclient(&nsession);
    let nurl=format!("{}/uploads.json?filename={}",nsession.url,filename.trim());
    println!("uplaod url: {}",nurl);
    let body=file_to_body(&path);
    //let body = reqwest::blocking::Body::new(file);
    match client.clone().post(nurl).body(body).send().await{
        Ok(issueresponse) => {
                println!("Response: {:?}",issueresponse);
                //checkresponse(&issueresponse);
                //will edit later
                let issuetext = issueresponse.text().await?;
                println!("issue_text: {:?}",issuetext);
                let s=serde_json::from_str::<Value>(&issuetext).unwrap();
                Ok(s)
            },

        Err(_e) => {
            println!("\nError: connection to server.\n\tPlease make sure you have access to the server\n\t{}\n",_e);
            process::exit(1)
        } 
    }
}


/// Dowload files
#[tokio::main]
pub async fn downloadfile(attachment:serde_json::Value, outputdirectory: &str )->Result<(),Box<dyn std::error::Error>>{
    // get content url
    let content_url=attachment.get("content_url").unwrap().as_str().unwrap();
    let fname= attachment.get("filename").unwrap().as_str().unwrap();
    // filesize to be used in case not retrieved as part of responsed for older versions of redmine
    let fsize =attachment.get("filesize").unwrap().as_u64().unwrap().to_string();
    let mut outfilepath=std::path::PathBuf::new();
    outfilepath.push(outputdirectory);
    //Check if directory is present
    match outfilepath.is_dir(){
        false =>{
                    println!("Output directory does not creating!");
                    std::fs::create_dir_all(&outfilepath).expect("Output directory can't be created!");
                },
        true =>{}
    }
    outfilepath.push(&fname);
    //println!("content_url:  {:?}\noutfilepath:  {:?}",content_url,outfilepath);
    // below code is not complete first need to get a proper client and get the content!!!
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client=buildclient(&nsession);
    match client.clone().get(content_url).send().await{
        Ok(issueresponse) => {
            println!("Issue Response donwlod:\n{:?}\n\n",&issueresponse);
            // get the response length from attachment information
            let attachments_content_length=HeaderValue::from_str(&fsize).unwrap();
            let length = issueresponse
                .headers()
                .get(CONTENT_LENGTH)
                .unwrap_or({
                        println!("Attachment size not retrieved from response, using from attachment fields");
                        &attachments_content_length
                });
            let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
            println!("Response length: {:?}",length);

            checkresponse(&issueresponse);
            println!("Started dowload: {}",&fname);
            let issuecontent=issueresponse.bytes().await?;
            let mut dest = File::create(&outfilepath)?;
            dest.write_all(&issuecontent)?;
            println!("File {} downloaded successfully!",&fname);
            Ok(())
        },
        Err(_e) =>{
            println!("\nError connecting to server failed to downlaod the file\n");
            process::exit(1)
        }
    }
}
