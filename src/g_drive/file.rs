extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_drive3 as drive3;

use self::oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, DiskTokenStorage};
use self::drive3::Drive;
use std::fs::File;

const JSON_SECRET: &'static str = "authfile";
const MIME_TYPE: &'static str = "application/vnd.google-apps.spreadsheet";
const AUTH_URI: &'static str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URI: &'static str = "https://accounts.google.com/o/oauth2/token";

pub struct UploaderConfig {
    pub id: String,
    pub secret: String,
    pub project: String,
    pub email: String,
}

pub struct Uploader {
    config: UploaderConfig,
}

impl Uploader {

    pub fn new(config: UploaderConfig) -> Self {
        Uploader{ config: config }
    }

    pub fn upload(&self, file_name: &str, created: bool) -> Result<(), drive3::Error> {
        let mut file_id: Option<String> = None;

        let secret: ApplicationSecret = ApplicationSecret { 
            client_id: self.config.id.clone(),
            client_secret: self.config.secret.clone(),
            project_id: Some(self.config.project.clone()),
            client_email: Some(self.config.email.clone()),
            auth_uri:AUTH_URI.to_string(),
            token_uri:TOKEN_URI.to_string(),
            ..ApplicationSecret::default()
        };

        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::new(),
                                      DiskTokenStorage::new(&JSON_SECRET.to_string()).unwrap(), 
                                      Some(oauth2::FlowType::InstalledInteractive));

        let hub = Drive::new(hyper::Client::new(), auth);
        let req = drive3::File { 
            mime_type: Some(MIME_TYPE.to_string()),
            is_app_authorized: Some(true),
            name: Some(file_name.to_string()),
            ..drive3::File::default() 
        };

        if created {
            println!("Creating file on Google Drive");
            try!(hub.files()
                    .create(req)
                    .upload_resumable(
                        File::open(file_name).unwrap(),
                        MIME_TYPE.parse().unwrap()
                    ));

            return Ok(());
        }


        if let None = file_id {
            let result = try!(hub.files()
                              .list()
                              .q(&*format!("name='{}'", file_name))
                              .doit());

            file_id = Some(Uploader::extract_id(&result.1));
        }
        println!("Updating file");

        try!(hub.files()
                .update(req, &*file_id.expect("File id is empty"))
                .upload_resumable(
                    File::open(file_name).unwrap(),
                    MIME_TYPE.parse().unwrap()
                ));

        Ok(())
    }

    fn extract_id(file_list: &drive3::FileList) -> String {
        let f = file_list.files.clone();
        let files_unwraped = f.unwrap();
        let first_file = files_unwraped.first();
        let first_file_unwraped = first_file.unwrap();
        let final_file_id = first_file_unwraped.id.clone();
        let id_unwraped = final_file_id.unwrap().clone();
        
        return id_unwraped;
    }
}
