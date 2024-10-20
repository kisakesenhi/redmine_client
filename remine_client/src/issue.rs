//! # Issues
//!
//! This module contains to functions to modify issues as json objects. 
//! Creates emtpy issues for posting to the API
use serde_json;
use serde_json::Value;
use quoted_string;
use newline_converter;
/// This function gets the custom_fields of an issue as json array.
pub fn get_custom_fields(issue: Value)->Value {
    json!(issue["issue"]["custom_fields"].clone())
}

/// Gets the custom field by name out of custom fields json array.
/// It doens't check if it's unique or not, provides the first instance, case sensitive !!!
/// Need to be improved !!!
/// Be carefull while writing a parser.
pub fn get_custom_field_by_name(name: String, custom_fields: Value )->Result<Value,String>{
    for field in custom_fields.as_array().unwrap(){
        if field["name"]==name{
            return Ok( json!(field.clone()) );
        }
    }
    Err(format!("{} not found in custom fields\n",&name))
} 


/// This function gets the custom field by id out of custom fields json array.
/// A little bit safer than the get_by_name since it's based on custom_field_id. But custom field
/// Id's might differ from different redmine databases !!!!
pub fn get_custom_field_by_id(id: usize, custom_fields: Value ) -> Result<Value, String>{
    for field in custom_fields.as_array().unwrap(){
        if field["id"]==id{
            return Ok(json!(field.clone()));
        }
    }
    Err(format!("Field with id {} not found in custom fields\n",&id))
}

/// This function accepts new value as String and custom_field as json object containing "id","name", "value" as subfields. 
/// For some fields value might be an array or an empty array. That need to be fixed for future
/// releases.
/// It's recoomended to get custom field from get_custom_field_by_id or get_custom_field_by_name
/// functions from issue.jons.
pub fn update_custom_field_value(newvalue: &str , mut custom_field: Value ) -> Result< Value, String>{
    custom_field["value"]=json!(newvalue);
    Ok(custom_field)
}

/// This will create an empty issue
pub fn create_emty_issue()->Value{
    serde_json::from_str(r#"{"issue":{"custom_fields":[]}}"#).unwrap()
}
/// Add note to issue
pub fn add_note_issue(mut issue:Value, message: String, is_private: bool) -> Value {
    if is_private {
        issue["issue"].as_object_mut().unwrap().insert("private_notes".to_string(),json!(true));
    }
    let mut new_message=String::from(message);
    // modify convert the new line characters into dos
    new_message=newline_converter::unix2dos(&new_message).into_owned();
    match serde_json::to_string(&issue["issue"]["notes"]){
        Ok(m)  =>{
                    if m=="null"    {
                        }else{
                            new_message.push_str("\n");
                            // need to find an elegant way to strip the quotes !!!
                            match quoted_string::strip_dquotes(&m){
                                Some(t) =>{ new_message.push_str(t);},
                                None => {new_message.push_str(m.as_str());}
                            }
                    }
                    },
        Err(e) => {
            println!("Error in anote: {}",e);
            println!("Issue in error {:?}",&issue["issue"]);

        }
    };
    issue["issue"]["notes"]=json!(new_message);
    issue
}

/// Modify status by id
pub fn modify_issue_status_byID(mut issue: Value, id: u64 ) -> Value {
    issue["issue"]["status_id"]= json!(id) ;
    issue
}

/// push mdofied custom field into an empty issue
pub fn push_custom_field( mut issue: Value, custom_field: Value)-> Value {
    issue["issue"]["custom_fields"].as_array_mut().unwrap().push(custom_field);
    issue
}
// Here we need a issue parser file. It'll be easier to create templates for issue parsers.
// An issue parser will take the ticket id and a comma seperated csv file that contains the fields.
// It'll create a issue.json file to be updated. But first write a function that updates a single
// field of a ticket. That function will need to use reqwest to upload data as well.

/// This function will create a template out of a given issue 
pub fn create_issue_template(issue: Value)-> String {
    let mut template=String::new();
    template.push_str("id,name,value\n");
    let custom_fields=get_custom_fields(issue);
    for field in custom_fields.as_array().unwrap(){
        template.push_str(format!("{},{},\n",field["id"],field["name"]).as_str())
    }
    template
}
