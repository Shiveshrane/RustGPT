use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]

struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)] //derive serde traits

struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)] //derive serde traits
struct database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl database {
    fn new() -> Self {
        database {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    //CRUD DATA

    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id) //return task by id
    }
    fn getall(&self) -> Vec<&Task> {
        self.tasks.values().collect() //return all tasks
    }
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id); //remove task from hashmap
    }
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task); //insert task into hashmap
    }

    //USER DATA RELATED FUNCTIONS
    fn create_user(&mut self, user: User) {
        self.users.insert(user.id, user); //insert user into hashmap
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|user| user.username == username) //closure to find user by username
    }

    //DATABASE SAVE
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?; //convert data to string
        let mut file = fs::File::create("database.json")?; //create file
        file.write_all(data.as_bytes())?; //write data to file
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let data = fs::read_to_string("database.json")?; //read data from file
        let db: database = serde_json::from_str(&data)?; //convert data to database
        Ok(db)
    }
}

struct AppState {
    db: Mutex<database>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); //lock database
    db.insert(task.into_inner()); //insert task into database. into_inner() is used to get inner value from Json
    let _ = db.save_to_file(); //save database to file
    HttpResponse::Ok().finish() //return 200 OK
}

async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap(); //lock database
    let task = db.get(&id.into_inner()); //get task by id
    match task {
        Some(task) => HttpResponse::Ok().json(task), //return task if found
        None => HttpResponse::NotFound().finish(), //return 404 if not found
    }
}

async fn read_all_task(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap(); //lock database
    let tasks = db.getall(); //get all tasks
    HttpResponse::Ok().json(tasks) //return all tasks
}

async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); //lock database
    db.delete(&id.into_inner()); //delete task by id
    let _ = db.save_to_file(); //save database to file
    HttpResponse::Ok().finish() //return 200 OK
}

async fn update_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap(); //lock database
    db.update(task.into_inner()); //update task
    let _ = db.save_to_file(); //save database to file
    HttpResponse::Ok().finish() //return 200 OK
}

async fn register(app_state:web::Data<AppState>,user:web::Json<User>)->impl Responder{
    let mut db =app_state.db.lock().unwrap();
    db.create_user(user.into_inner());
    let _=db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state:web::Data<AppState>,user:web::Json<User>)->impl Responder{
    let db=app_state.db.lock().unwrap();
  match db.get_user_by_name(&user.username){
      Some(u) if u.password==user.password=>{
              HttpResponse::Ok().body("logged in Successfully!")
            },
      _=>{
          HttpResponse::BadRequest().body("incorrect username or password!!!")
      }
  }
}


#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let db = match database::load_from_file() {
        //load database from file
        Ok(db) => db,              //if database is loaded from file
        Err(_) => database::new(), //if database is not loaded from file
    };
    let data = web::Data::new(AppState { db: Mutex::new(db) }); //create app state

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) //allow methods like GET, POST, PUT, DELETE. GET is for fetching data, POST is for creating data, PUT is for updating data, DELETE is for deleting data
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT]) //allow headers like AUTHORIZATION, CONTENT_TYPE
                    .allowed_header(header::CONTENT_TYPE) //allow header like CONTENT_TYPE
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/task", web::post().to(create_task)) //create task. This is the rest API endpoint
            .route("/task/{id}", web::get().to(read_task)) //read task by id. This is the rest API endpoint
            .route("/task", web::get().to(read_all_task)) //read all tasks. This is the rest API endpoint
            .route("/task/{id}", web::delete().to(delete_task)) //delete task by id. This is the rest API endpoint
            .route("/task", web::put().to(update_task)) //update task. This is the rest API endpoint
            .route("/register",web::post().to(register))//register user to the system
            .route("/login",web::post().to(login))//login user to the system
    })
    .bind("127.0.0.1:8080")? //bind server
    .run()
    .await
}