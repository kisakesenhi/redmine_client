//! # Requests
//!
//! This module is based on reqwest library. Interfaces with the Redmine-API.
//! Uses serde_json module so whenever possible all the interaction with API is via json objects.

use serde_json;
use reqwest::header;
use std::process;
use serde_json::Value;
use crate::session::getsessioninfo;

/// Structure fol holding the session information. Serializable and deserializable to serde_json
/// object.
#[derive(Serialize,Deserialize,Debug)]
pub struct SessionInfo{
    /// Full Base URL for the redmine server
    pub url:String,
    /// Api key could be retrived from Redmine
    pub apikey:String,
}

/// Trait for serialation of  Session Info to serde_json::Value
pub trait SessionSerilaizer{
    fn serializesession(&self)->serde_json::Value;
    fn deserializesession(i: serde_json::Value)->SessionInfo;
}

impl SessionSerilaizer for SessionInfo {
    fn serializesession(&self)->serde_json::Value{
        //json!({"url":&self.url, "apikey":&self.apikey})
        json!(&self)
    }

    fn deserializesession(i:serde_json::Value)->SessionInfo{
        SessionInfo{
            url : i["url"].as_str().unwrap().to_string(),
            apikey : i["apikey"].as_str().unwrap().to_string(),
        }
    }
}
/// reqwest client builder adds the correct header information containing content-type and API-Key
pub fn buildclient(session:&SessionInfo)->reqwest::Client{
    // This will create the client builder using the apikey from the SessionInfo
    let mut headers= header::HeaderMap::new();
    headers.insert("Content-Type",header::HeaderValue::from_static("application/json"));
    headers.insert("X-Redmine-API-Key",header::HeaderValue::from_str(&session.apikey).unwrap());
    reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap()
}
/// reqwest client builder for uploading files
pub fn builduploadclient(session:&SessionInfo)->reqwest::Client{
    // This will create the client builder using the apikey from the SessionInfo
    let mut headers= header::HeaderMap::new();
    headers.insert("Content-Type",header::HeaderValue::from_static("application/octet-stream"));
    headers.insert("X-Redmine-API-Key",header::HeaderValue::from_str(&session.apikey).unwrap());
    reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap()
}

/// Checks the response from the Redmine and prints catched error codes and quits
/// Checked error codes:
/// * NOT_FOUNT
/// * UNAUTHORIZED
/// * REQUEST_TIMEOUT
pub fn checkresponse(response: &reqwest::Response){
    match response.status(){
        reqwest::StatusCode::OK =>{ },
        reqwest::StatusCode::NO_CONTENT =>{ },
        reqwest::StatusCode::NOT_FOUND => {
                eprintln!("Request not found!");
                process::exit(1);
            },
        reqwest::StatusCode::UNAUTHORIZED => {
                eprintln!("You're not Authorized for access!");
                process::exit(1);
            },
        reqwest::StatusCode::REQUEST_TIMEOUT => {
                eprintln!("Request Timed out!!. Make sure the server is accessible!");
                process::exit(1);
            },
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                eprintln!("Unprocessible Entities. Please check custom value specifications");
                process::exit(1);
        },
        _ => {
            eprintln!("Error: {:?}", response.status());
            process::exit(1);
        }
    }
}
///Main function to get the issue in json format including attachments, journals, changesets.
#[tokio::main]
pub async fn getissue(issuenumber: usize)->Result<serde_json::Value, Box<dyn std::error::Error>>{
    // This will get the issue with the correct number
    //First get the build the client from session info
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client =buildclient(&nsession);
    let nurl=format!("{}/issues/{}.json?include=attachments,journals,changesets",nsession.url,issuenumber);
    match client.clone().get(nurl).send().await{
        Ok(issueresponse) => {
                checkresponse(&issueresponse);
                let issuetext = issueresponse.text().await?;
                let s=serde_json::from_str::<Value>(&issuetext).unwrap();
                Ok(s)
            },

        Err(_e) => {
            eprintln!("\nError: connection to server.\n\tPlease make sure you have access to the server\n\t{}\n",_e);
            process::exit(1)
        } 
    }
}

///Function to get issues by status id provided
#[tokio::main]
pub async fn getIssuesbyStatusID(status_id: usize)->Result<serde_json::Value, Box<dyn std::error::Error>>{
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client =buildclient(&nsession);
    let nurl=format!("{}/issues.json?status_id={}",nsession.url, status_id);
    match client.clone().get(nurl).send().await{
        Ok(issueresponse) => {
                checkresponse(&issueresponse);
                let issuetext = issueresponse.text().await?;
                let s=serde_json::from_str::<Value>(&issuetext).unwrap();
                Ok(s)
        },
        Err(_e) => {
            eprintln!("\nError: connection to server.\n\tPlease make sure you have access to the server\n\t{}\n",_e);
            process::exit(1)
        }
    }
}

///Main function to get the issue in json format including attachments, journals, changesets.
#[tokio::main]
pub async fn list_issue_statuses()->Result<serde_json::Value, Box<dyn std::error::Error>>{
    // This will get the issue with the correct number
    //First get the build the client from session info
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client =buildclient(&nsession);
    let nurl=format!("{}/issue_statuses.json",nsession.url);
    match client.clone().get(nurl).send().await{
        Ok(issueresponse) => {
                checkresponse(&issueresponse);
                let issuetext = issueresponse.text().await?;
                //println!("{:?}",&issuetext);
                let s=serde_json::from_str::<Value>(&issuetext).unwrap();
                Ok(s)
            },

        Err(_e) => {
            eprintln!("\nError: connection to server.\n\tPlease make sure you have access to the server\n\t{}\n",_e);
            process::exit(1)
        } 
    }
}


///Main function to get the issue in json format including attachments, journals, changesets.
#[tokio::main]
pub async fn update_issue(issuenumber: usize,issue: Value){
    // This will get the issue with the correct number
    //First get the build the client from session info
    let sessioninfo = getsessioninfo();
    let nsession=SessionInfo::deserializesession(sessioninfo);
    let client =buildclient(&nsession);
    let nurl=format!("{}/issues/{}.json",nsession.url,&issuenumber);
    match client.clone().put(nurl).json(&issue).send().await{
        Ok(issueresponse) => {
                checkresponse(&issueresponse);
                println!("Issue #{} updated successfully!",&issuenumber);
                eprintln!("{:?}",&issueresponse);
            },

        Err(_e) => {
            eprintln!("\nError: connection to server.\n\tPlease make sure you have access to the server\n\t{}\n",_e);
            process::exit(1)
        } 
    }
}
/// List attachment files from using getissue
pub fn listattachments(issuenumber: usize) -> Vec<serde_json::Value>{
    let issue = getissue(issuenumber).unwrap();
    let attachments = issue.get("issue").and_then(|i| i.get("attachments")).unwrap();
    let attachmentarray=attachments.as_array().unwrap();
    /*
    for i in attachmentarray {
        println!("\t{}", i.get("filename").unwrap());
    }
    */
    // return the attachment array
    attachmentarray.to_vec()
}

/// Print attachments summary. 
/// Total number of attachments,
/// filename, filesize, descriptions
pub fn printattachments(attachments: Vec<serde_json::Value>) {
    let total=attachments.len();
    println!("Number of Attachments: {}",total);
    println!("Number\tName\tSize\tDescription");
    for (i,a) in attachments.iter().enumerate(){
        let fname=a.get("filename").unwrap();
        let fsize=a.get("filesize").unwrap();
        let fdescription=a.get("description").unwrap();
        println!("\t{} - {}\t{}\t{}", i+1, fsize, fname, fdescription );
    }
}

/// Filters the attachments by partial filenames.
pub fn filterattachments(attachments: Vec<serde_json::Value>, filenames : Vec<String>) -> Vec<serde_json::Value>{
    let mut filtered : Vec<serde_json::Value>=vec!();
    if filenames==vec!["*"]{
         attachments
    }else{
        for a in attachments{
            let mut isin=false;
            let fname=a.get("filename").unwrap().as_str().unwrap();
            for f in filenames.clone(){
                if fname.contains(&f){isin=true};
                }
            if isin {filtered.push(a)}
            }
    filtered
    }
}

