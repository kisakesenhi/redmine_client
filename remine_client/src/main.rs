//! # Redmine Client
//!
//! Redmine client is a simple client app written for command-line access
//! and modify issues, written in Rust.
//!
//! # Usage
//! Clap has been used to create the main app module.
//! So for usage of the command line -h , --help could be used.

#[macro_use] extern crate magic_crypt;
#[macro_use] extern crate serde_json;
extern crate reqwest;
extern crate serde;
extern crate csv;
#[macro_use] extern crate serde_derive;
#[macro_use]extern crate clap;
extern crate mime_guess;
extern crate quoted_string;
extern crate newline_converter;

use clap::{Arg, App, SubCommand};
mod session;
mod requests;
use requests::update_issue;
use requests::list_issue_statuses;
mod issue;
use issue::get_custom_field_by_name;
use issue::update_custom_field_value;
use issue::create_emty_issue;
use issue::push_custom_field;
use issue::create_issue_template;
mod parsers;
use std::process;
mod file_ops;

fn main() {
    let mut app = App::new("Redmine Client")
        .version("0.0.1")
        .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
        .subcommand(SubCommand::with_name("register")
                    .about("register the app")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>"))
        .subcommand(SubCommand::with_name("get-ticket")
                    .about("will print the ticket in pretty json format! Including the journals, changesets and attachments")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("Ticket ID")
                    ))
        .subcommand(SubCommand::with_name("list-files")
                    .about("Will listt the issue attachments")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("Ticket ID")
                    ))
        .subcommand(SubCommand::with_name("list-issue-statuses")
                    .about("Will list the issue statuses")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>"))
        .subcommand(SubCommand::with_name("get-files")
                    .about("download attachments of a ticket")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id"))
                    .arg(Arg::with_name("files")
                         .short("f")
                         .long("files")
                         .takes_value(true)
                         .default_value("*")
                         .multiple(true)
                         .help("file names ( partial / full )")
                         )
                    .arg(Arg::with_name("output-directory")
                         .short("o")
                         .long("output-dir")
                         .takes_value(true)
                         .default_value("./")
                         .help("output directory to download files.")
                         )
                    )
        .subcommand(SubCommand::with_name("add-note")
                    .about("add note to an issue")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com")
                    .version("0.0.1")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id")
                         )
                    .arg(Arg::with_name("status_id")
                         .short("s")
                         .long("status_id")
                         .required(false)
                         .takes_value(true)
                         .help("status_id, could be printed by list-issue-statuses")
                        )
                    .arg(Arg::with_name("message")
                         .short("m")
                         .long("message")
                         .required(true)
                         .takes_value(true)
                         .default_value("")
                         .help("\"message\" to be added as note")
                         )
                    .arg(Arg::with_name("private_note")
                         .short("p")
                         .long("private")
                         .takes_value(false)
                         .required(false)
                         .help("add message as private!")
                        )
                    //);
            )
        .subcommand(SubCommand::with_name("update_field")
                    .about("update custom field of ticket")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id"))
                    .arg(Arg::with_name("field_name")
                         .short("f")
                         .long("field_name")
                         .required(true)
                         .takes_value(true)
                         .help("field name"))
                    .arg(Arg::with_name("new_value")
                         .short("v")
                         .long("new_value")
                         .takes_value(true)
                         .required(true)
                         .help("new value"))
                    .arg(Arg::with_name("message")
                         .short("m")
                         .long("message")
                         .required(true)
                         .takes_value(true)
                         .default_value("")
                         .help("\"message\" to be added as note")
                         )
                    .arg(Arg::with_name("private_note")
                         .short("p")
                         .long("private")
                         .takes_value(false)
                         .required(false)
                         .help("add message as private!")
                        )
                    )
        .subcommand(SubCommand::with_name("create_template")
                    .about("create a template for parsers, get the custom fieds id's and names")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .help("create a template for parsers, get the custom fieds id's and names")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id"))
                    )
        .subcommand(SubCommand::with_name("update_from_template")
                    .about("read custom fields template from csv parser")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id"))
                    .arg(Arg::with_name("csv_template")
                         .short("c")
                         .long("csv")
                         .required(true)
                         .takes_value(true)
                         .help("csv template"))
                    )

        .subcommand(SubCommand::with_name("upload_file")
                    .about("upload file !!!!")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen <kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("ticketID")
                         .short("t")
                         .long("ticket_id")
                         .required(true)
                         .takes_value(true)
                         .help("ticket id"))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .required(true)
                         .takes_value(true)
                         .help("file to attach"))
                    .arg(Arg::with_name("message")
                         .short("m")
                         .long("message")
                         .required(false)
                         .takes_value(true)
                         .default_value("file attached")
                         .help("message to be added as note")
                        )
                    .arg(Arg::with_name("filename")
                         .short("n")
                         .long("filename")
                         .required(false)
                         .takes_value(true)
                         .help("new file name"))
                    .arg(Arg::with_name("is_text")
                         .long("text")
                         .takes_value(false)
                         .required(false)
                         .help("set the content type to text/plain")
                         )
                    .arg(Arg::with_name("private_note")
                         .short("p")
                         .long("private")
                         .takes_value(false)
                         .required(false)
                         .help("add message as private!")
                        )
                    )
        .subcommand(SubCommand::with_name("getTicketbyStatusID")
                    .about("getTickets by statusid")
                    .version("0.0.1")
                    .author("Ibrahim Kisakesen < kisakesenhi@gmail.com>")
                    .arg(Arg::with_name("status_id") 
                         .short("s")
                         .long("status_id")
                         .required(true)
                         .takes_value(true)
                         .help("status id")
                         )
                    );

    let matches=app.clone().get_matches();
                    //.get_matches();
    

    match matches.subcommand_name(){
        Some("register")=>{session::registerapp()},
        Some("get-ticket")=>{
            if let Some(m)=matches.subcommand_matches("get-ticket"){
                let values=values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                match requests::getissue(values[0]){
                    Ok(issue) => {
                        print!("{}",serde_json::to_string_pretty(&issue).unwrap());
                        },
                    Err(_)=> {
                            println!("Could't get issue");
                            process::exit(1);
                        }
                    };
                }
            },
        Some("list-files") => { 
            if let Some(m)=matches.subcommand_matches("list-files"){
                let values = values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let a=requests::listattachments(values[0]);
                requests::printattachments(a);
            }
        },
        Some("list-issue-statuses")=>{
            if let Some(_)=matches.subcommand_matches("list-issue-statuses"){
            match requests::list_issue_statuses(){
                Ok(statuses) => {
                    print!("{}",serde_json::to_string_pretty(&statuses).unwrap());
                },
                Err(_) => {
                    println!("Couldn't get the issue statuses");
                }
            };
            }
        },
        Some("get-files")=>{
            if let Some(m)=matches.subcommand_matches("get-files"){
                let values = values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let outdir= values_t!(m.values_of("output-directory"),String).unwrap_or_else(|e| e.exit());
                let files = values_t!(m.values_of("files"),String).unwrap_or_else(|e| e.exit());
                let a=requests::listattachments(values[0]);
                let f=requests::filterattachments(a,files);
                for i in f {
                    // Better handle download failures!!
                    file_ops::downloadfile(i,&outdir[0]).expect("Failed to download the file!");
                }

            }
        },
        Some("add-note")=>{
            if let Some(m)=matches.subcommand_matches("add-note"){
                let values = values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let message=values_t!(m.values_of("message"),String ).unwrap_or_else(|e| e.exit());
                let is_note_private:bool =m.is_present("private_note");
                let is_statusID_present:bool = m.is_present("status_id");
                match requests::getissue(values[0]){
                    Ok(_issue) => {
                                let mut new_issue=create_emty_issue();
                                if &message[0] ==""{
                                    println!("Message empty doing nothing!");
                                }else{
                                        // find an elegant way later
                                        // TODO implement is_private
                                        new_issue=issue::add_note_issue(new_issue,message[0].clone(),is_note_private); 
                                        // update status if
                                        if is_statusID_present {
                                            //update the issue_status
                                            let status_id = values_t!(m.values_of("status_id"),u64).unwrap_or_else(|e| e.exit());
                                            new_issue=issue::modify_issue_status_byID(new_issue,status_id[0]);
                                        } 
                                        update_issue(values[0],new_issue);
                                        }
                                        },
                    Err(_) => {
                        println!("Couldn't get issue");
                        process::exit(1);
                        },
                }
            }
        },
        Some("update_field")=>{
            if let Some(m)=matches.subcommand_matches("update_field"){
                let values = values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let value_field = values_t!(m.values_of("field_name"),String).unwrap_or_else(|e| e.exit());
                let value_new = values_t!(m.values_of("new_value"),String).unwrap_or_else(|e| e.exit());
                let message=values_t!(m.values_of("message"),String ).unwrap_or_else(|e| e.exit());
                match requests::getissue(values[0]){
                    Ok(issue) => {
                        let customfields=issue::get_custom_fields(issue);
                        match get_custom_field_by_name( value_field[0].clone() ,customfields){
                            Ok(field)=>{
                                let empty_issue=create_emty_issue();
                                match update_custom_field_value(&value_new[0],field){
                                    Ok(newfield)=>{
                                        let mut added_issue=push_custom_field(empty_issue,newfield);
                                        if &message[0] ==""{}else{
                                            // find an elegant way later
                                            // TODO implement is_private note
                                            added_issue=issue::add_note_issue(added_issue,message[0].clone(),false); 
                                        }
                                        update_issue(values[0],added_issue);
                                        },
                                    Err(e)=>{print!("{}",e)},
                                }
                            },
                            Err(e) => {print!("Error {}",e);}
                        };
                    },
                    Err(_) => {
                        println!("Couldn't get issue");
                        process::exit(1);
                        },
                }
            }
        },
        Some("create_template")=>{
            if let Some(m)=matches.subcommand_matches("create_template"){
                let values=values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                match requests::getissue(values[0]){
                    Ok(issue) => {
                        println!("{}",create_issue_template(issue));
                        },
                    Err(_)=> {
                            println!("Could't get issue");
                            process::exit(1);
                        }
                    };
                }
            },
        Some("update_from_template")=>{
            if let Some(m)=matches.subcommand_matches("update_from_template"){
                let values=values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let template_file=values_t!(m.values_of("csv_template"),String).unwrap_or_else(|e| e.exit());
                match requests::getissue(values[0]){
                    Ok(_issue) => {
                        //println!("{}",create_issue_template(issue));
                        let mut updated_issue=create_emty_issue();
                        match parsers::read_custom_fields_from_template(template_file[0].clone()){
                            Ok(field_vector)=>{
                                for field in field_vector{
                                    updated_issue = push_custom_field(updated_issue,field);
                                }
                                update_issue(values[0],updated_issue);
                            },
                            Err(e)=> {
                                println!("Couldn't read the templates!\n{}",e);
                                process::exit(1);
                            }
                        }
                        },
                    Err(_)=> {
                            println!("Could't get issue");
                            process::exit(1);
                        }
                    };
                }
            },
        Some("upload_file")=>{
            if let Some(m)= matches.subcommand_matches("upload_file"){
                let values=values_t!(m.values_of("ticketID"),usize).unwrap_or_else(|e| e.exit());
                let file=values_t!(m.values_of("file"),String).unwrap_or_else(|e| e.exit());
                let filepath=std::path::Path::new(&file[0]);
                let guess = mime_guess::from_path(filepath);
                let mime: mime_guess::Mime;
                let is_note_private:bool =m.is_present("private_note");
                if m.is_present("is_text"){
                    mime="text/plain".parse().unwrap();
                }else{
                    mime=guess.first_or_octet_stream();
                }
                let message=values_t!(m.values_of("message"),String).unwrap_or_else(|e| e.exit()).join(" ");
                let filename;
                if m.is_present("filename"){
                    filename=values_t!(m.values_of("filename"),String).unwrap_or_else(|e| e.exit());
                }else{
                    filename=file.clone();
                }
                println!("Guessed Mime/guess: {}", mime);
                //
                match requests::getissue(values[0]){
                    Ok(_issue)=>{
                         match file_ops::upload_file(file[0].clone(),filename[0].clone()){
                             Ok(token) => { 
                                 //println!("Returned upload:\n\t{:?}",token);
                                 //println!("Returned toke:\n\t{:?}",token.get("upload").unwrap().get("token").unwrap().as_str().unwrap());
                                 //println!("");
                                 //println!("issue \n{:?}",issue.get("issue").unwrap().get("id").unwrap());
                                 //println!("project id \n{:?}",issue.get("issue").unwrap().get("project").unwrap().get("id").unwrap().as_u64().unwrap());
                                 let token_text=token.get("upload").unwrap().get("token").unwrap().as_str().unwrap();
                                 //let issue_id=issue.get("issue").unwrap().get("id").unwrap().as_i64().unwrap();
                                 //let pid=issue.get("issue").unwrap().get("project").unwrap().get("id").unwrap().as_u64().unwrap();
                                 let upload_json_text=format!("{{\"issue\": {{ \"notes\":\"\",
                                                         \"uploads\":[
                                                         {{\"token\": \"{}\" ,
                                                            \"filename\": \"{}\",
                                                            \"content_type\":\"{}\"
                                                         }}
                                                         ]
                                                         }}}}",
                                                         token_text,filename[0],mime);
                                 let mut upload_json=serde_json::from_str(&upload_json_text).unwrap();
                                 // mut Add the message to the notes
                                 upload_json=issue::add_note_issue(upload_json,message,is_note_private);
                                 //upload_json["issue"]["notes"].push(&message);
                                 println!("Upload_json:\n\t{:?}",&upload_json);
                                 update_issue(values[0],upload_json);
                             }
                             Err(_)=>{
                                 println!("Upload not succesfull!");
                             }
                         }
                    },
                    Err(_)=>{
                        println!("Couldn't get issue");
                        process::exit(1);
                    }
                };
            }
        },

        Some("getTicketbyStatusID") => {
            if let Some(m)=matches.subcommand_matches("getTicketbyStatusID"){ 
                let status_id = values_t!(m.values_of("status_id"),usize).unwrap_or_else(|e| e.exit());

                //println!("Status id entered: {:?}",&status_id);

                match requests::getIssuesbyStatusID(status_id[0]){
                    Ok(issue) => {
                        print!("{}",serde_json::to_string_pretty(&issue).unwrap());
                    },
                    Err(_) => {
                            println!("Could't get issue");
                            process::exit(1);
                    }
                };
            }
        },

        _ => {
                app.print_long_help().expect("Can't print the help!");
            }
    }
}
