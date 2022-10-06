use std::{sync::Mutex, fs::File, io::Write, fmt::format};
use rocket::{State, serde::{json::{self, Json}, Serialize, self}, tokio::fs::read_to_string};
use rocket::serde::{Deserialize};
use uuid::Uuid;
use rand::Rng;


#[macro_use] extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message{
    content:  String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Note{
    index: usize,
    content: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Document{
    docId: String,
    content: String
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Coordinate<'a>{
    lattitude: f32,
    longitude: f32,
    ewHemisphere: &'a str,
    nsHemisphere: &'a str
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DocumentInfo{
    docId: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

// #[get("/notes")]
// fn get_all_notes(state: &State<Mutex<Vec<String>>>) ->   String{
//     let messages = state.lock().unwrap();
//     let mut json_array = String::from("");
//     json_array.push_str("[");
//     for c in messages.iter(){
//         let element = format!("\"{}\",",c);
//         json_array.push_str(&element);  
//     }
//     json_array.pop();
//     json_array.push_str("]");
//     json_array
// }

#[get("/notes")]
fn get_all_notes(state: &State<Mutex<Vec<String>>>) ->   Json<Vec<String>>{
    let messages = state.lock().unwrap();
    let notes = messages.to_vec();
    Json(notes)
}

#[get("/notes/<index>")]
fn get_note_by_index(index:&str, state: &State<Mutex<Vec<String>>>) -> Json<Note>{
    let i = index.parse::<usize>().unwrap();
    let messages = state.lock().unwrap();
    let content = &messages[i];
    Json(Note{content:content.to_owned(), index:i})
}


#[post("/notes", data="<message>")]
fn add_note(message: Json<Message>, state: &State<Mutex<Vec<String>>>) -> Json<Note>{
    let mut messages = state.lock().unwrap();
    let rocket::serde::json::Json(Message{content}) = message;
    let mut note = Note{content:content.to_owned(), index:10};
    messages.push(content);
    note.index = messages.len() -1;
    Json(note)
}

#[post("/notes/<index>", data="<message>")]
fn add_note_by_index(index:&str, message: Json<Message>, state: &State<Mutex<Vec<String>>>) -> Json<Note>{
    let i = index.parse::<usize>().unwrap();
    let mut messages = state.lock().unwrap();
    let rocket::serde::json::Json(Message{content}) = message;
    let note = Note{content:content.to_owned(), index:i};
    messages.insert(i, content);
    Json(note)
}

#[put("/notes/<index>", data="<message>")]
fn replace_note_by_index(index:&str, message: Json<Message>, state: &State<Mutex<Vec<String>>>) -> Json<Note>{
    let i = index.parse::<usize>().unwrap();
    let mut messages = state.lock().unwrap();
    let rocket::serde::json::Json(Message{content}) = message;
    let note = Note{content:content.to_owned(), index:i};
    messages[i] = content;
    Json(note)
}


#[delete("/notes/<index>")]
fn delete_note_by_index(index:&str, state: &State<Mutex<Vec<String>>>) -> &'static str{
    let i = index.parse::<usize>().unwrap();
    let mut messages = state.lock().unwrap();
    messages.remove(i);
    "Done"
}

#[post("/documents", data="<message>")]
fn create_document(message: Json<Message>) -> Json<DocumentInfo>{
    let rocket::serde::json::Json(Message{content}) = message;
    let id = Uuid::new_v4();
    let file_name = format!("{}.txt",id.to_string());
    let mut file = File::create(file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    Json(DocumentInfo{docId:id.to_string()})
}

#[get("/documents/<id>")]
async fn get_document(id: &str) -> Json<Document>{
    let file_name = format!("{}.txt",id);
    let content = read_to_string(file_name).await.unwrap();
    Json(Document{ docId:id.to_string(), content: content })
}

#[get("/math/<num1>/<num2>/<amount>")]
fn math(num1: &str, num2: &str, amount: &str) -> &'static str{ 
    let n1 = num1.parse::<f64>().unwrap();
    let n2 = num2.parse::<f64>().unwrap();
    let a = amount.parse::<i128>().unwrap();

    for _x in  0..a {
        let _z = n1*n2;
    }
    "done"
}

#[get("/factorial/<factor>")]
fn factorial(factor: &str) -> String{
    let f = factor.parse::<i128>().unwrap();

    let mut tot: i128 = 1;

    for i in 1..f+1{
        tot *= i;
    }

    tot.to_string()
    
}

#[get("/coordinates/<amount>")]
fn random_coordinates(amount: &str) -> Json<Vec<Coordinate>>{
    let a = amount.parse::<i128>().unwrap();
    let mut rng = rand::thread_rng();
    
    let mut coordinates:Vec<Coordinate> = vec![];

    for _x in 0..a{
        let lat:f32 = rng.gen_range(-90.0..90.0);
        let long:f32 = rng.gen_range(-180.0..180.0);

        let c = Coordinate{
            lattitude:lat,
            longitude:long,
            nsHemisphere: if lat > 0.0 {"North"} else {"South"},
            ewHemisphere: if long > 0.0 {"East"} else {"West"}
        };
        coordinates.push(c);
    }

    Json(coordinates)

}

#[launch]
fn rocket() -> _ {

    let message_mutex = Mutex::new(vec!["m1".to_string(),"m2".to_string(),"m3".to_string()]);

    rocket::build().manage(message_mutex).mount("/", routes![
        index, get_all_notes, 
        add_note, 
        get_note_by_index, 
        add_note_by_index, 
        delete_note_by_index,
        replace_note_by_index,
        create_document,
        get_document,
        math,
        factorial,
        random_coordinates
        ])
}