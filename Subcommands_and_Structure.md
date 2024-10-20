### Subcommand structure

register

get-issue
	--ticket_id	-t

list-files
	--ticket_id	-t

get-files
	--ticket_id	-t
	--files	-f    #filenames partial/full
	--output-dir	-o	#Output Directory

update_field
	--ticket_id	-t
	--field_name	-f
	--new_value	-v #new value

create_template
	--ticket_id	-t

update_from_template
	--ticket_id	-t
	--csv	-c #csv template
	
upload_file
	--ticket_id	-t
	--file	-f
	--filename	-n	#New file name
	--is_test	-	#set content type to text

## register

	Calls the function session::registerapp()
	
	registerapp():
		asks the user the adress of the server and the api key to use. 
		Creates a configs directory
		Creates session infor as json containing adress and apikey
		Encryps the session info and writes to configfile

## get-issue
	Calls the function requests::getissue(ticketid)

	getissue(id):
		Gets session info
		creates a client and gets the response
		Checks response if respone ok
		returns the issue as json

## list-files
	Calls the function requests::listattachements and prints the attachments
	listattachments(issue):
		Gets issue requests::getissue()
		gets attachments arrays
		prints filename with number

## get-files
	Calls the functions request::listaatachments and requests::filterattachments to get list of files and sends dowload request with request::downloadfile function

	filterattachments(a,files):
		a is the list of attachmenst
		files is a string from command line [ * indicates all files ]
		
