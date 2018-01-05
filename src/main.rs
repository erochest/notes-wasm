#![recursion_limit="128"]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate yew;

use strum::IntoEnumIterator;
use yew::html::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

struct Model {
    notes: Vec<Note>,
    new_title: String,
    new_text: String,
}

struct Note {
    title: String,
    text: String,
}

enum Msg {
    AddNote,
    CancelAdd,
    NoOp,
}

fn update(context: &mut Context, input: &mut Model, msg: Msg) {
    match msg {
        Msg::AddNote => {
            let console = &context.console;
            console.log(&format!("new_title = {:?}", &input.new_title));
            console.log(&format!("new_text  = {:?}", &input.new_text));
        },
        _ => {
        },
    }
}

trait Viewable {
    fn view(&self) -> Html<Msg>;
}

impl Viewable for Model {
    fn view(&self) -> Html<Msg> {
        html! {
            <div class="container",>
                { for self.notes.iter().map(Note::view) }
                <div class="fixed-action-btn",>
                    <a class=("btn-floating", "btn-large", "waves-effect", "waves-light", "modal-trigger"), href="#add-modal",>
                        <i class=("large", "material-icons"),>{ "add" }</i>
                    </a>
                </div>
            </div>
            <div id="add-modal", class="modal",>
                <div class="modal-content",>
                    <h4>{ "Add Note" }</h4>
                    <div class="row",>
                        <form class=("col", "s12"),>
                            <div class="row",>
                                <div class=("input-field", "col", "s12"),>
                                    <input placeholder="title", id="title_input", type="text", value=&self.new_title, ></input>
                                    <label for="title_input",>{ "Title" }</label>
                                </div>
                            </div>
                            <div class="row",>
                                <div class=("input-field", "col", "s12"),>
                                    <textarea id="body_input", class="materialize-textarea",>{ &self.new_text }</textarea>
                                    <label for="body_input",>{ "Body" }</label>
                                </div>
                            </div>
                        </form>
                    </div>

                </div>
                <div class="modal-footer",>
                    <button class=("modal-action", "modal-close", "waves-effect", "waves-green", "btn-flat"), onclick=|_| Msg::AddNote,>{ "Add" }</button>
                    <button class=("modal-action", "modal-close", "btn-flat"), onclick=|_| Msg::CancelAdd,>{ "Cancel" }</button>
                </div>
            </div>
        }
    }
}

impl Viewable for Note {
    fn view(&self) -> Html<Msg> {
        html! {
            <div class="row",>
                <div class=("col", "s12", "m6"),>
                    <div class=("card", "blue-grey", "darken-1"),>
                        <div class=("card-content", "white-text"),>
                            <span class="card-title",>{ &self.title }</span>
                            <p>{ &self.text }</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    let mut app = App::new();
    let context = Context {
        console: ConsoleService,
    };
    let state = Model {
        notes: vec![
            Note {
                title: String::from("Everything Here is Beautiful"),
                text: String::from("by Mira T. Lee"),
            },
            Note {
                title: String::from("The Friend"),
                text: String::from("by Sigrid Nunez"),
            }
        ],
        new_title: String::from(""),
        new_text: String::from(""),
    };

    app.run(context, state, update, Model::view);
}
