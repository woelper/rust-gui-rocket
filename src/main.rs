#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::response::content;
use std::sync::Mutex;


//Serialize makes this automatically serializable (into json)
//Deserialize makes this automatically deserializable (from json)
//Debug let's us print the struct (like doing your own repr in python)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    complete: bool
}

// In rust, we can't just write to a global variable, as that would cause
// problems (especially in threads) and be unsafe.
lazy_static! {
    static ref TASKS: Mutex<Vec<Task>> = Mutex::new(vec![]);
}



#[get("/")]
fn index() -> &'static str {
    "Hi"
}

// receive json and make tasks from it (deserialize)
#[post("/save", data = "<tasks>")] //<tasks> means that the function below will have a var called tasks
fn save(tasks: Json<Vec<Task>>) -> &'static str {
    /*
    "task" is deserialized from a POST request that sends us JSON data.
    The browser (or anything else, you could use curl at the command line)
    can send us JSON and we automatically turn it back into a "task" struct.
    super handy.
     */

    let mut old_tasks = TASKS.lock().unwrap();
    * old_tasks = tasks.to_vec();

    // dbg!(&tasks);
    

    "nice..."
}

// turn tasks into json (serialize)
#[get("/load")]
fn load() -> content::Json<String> {
    // get a lock here on TASKS
    let guard = TASKS.lock().unwrap();
    // serialize all tasks
    let json_string = serde_json::to_string(&*guard).unwrap();
    // hand them out to the browser (or anything else) and tell them to expect json
    content::Json(json_string)
}

fn main() {

    // let's make some default tasks for fun
    let get_up = Task {
        description: "Get out of bed.".to_string(),
        complete: false
    };

    let cook = Task {
        description: "Heat up a pizza to an edible temperature".to_string(),
        complete: false
    };

    // get a lock on tasks, this time mutable since we wanna add stuff
    let mut guard = TASKS.lock().unwrap();
    guard.push(get_up);
    guard.push(cook);
    drop(guard);

    // ignite rocket, then mount all endpoints, launch.
    // Now you should be able to point your browser to http://localhost:8000/load and see example tasks.
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![save])
    .mount("/", routes![load])
    .mount("/static", StaticFiles::from("web"))
    .launch();
}