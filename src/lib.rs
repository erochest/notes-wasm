#![recursion_limit="512"]

use log::info;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use wasm_bindgen::prelude::*;
use wasm_logger;

struct Context {
    console: ConsoleService,
}

struct Model {
    link: ComponentLink<Self>,
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

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            notes: Vec::new(),
            new_text: String::new(),
            new_title: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddNote => {
                info!("new-title: {:?}", &self.new_title);
                info!("new-text: {:?}", &self.new_text);
            },
            _ => {},
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        Viewable::view(self)
    }
}

trait Viewable {
    fn view(&self) -> Html;
}

impl Viewable for Model {
    fn view(&self) -> Html {
        html! {
            <>
                <div class="container",>
                    { for self.notes.iter().map(Note::view) }
                    <div class="fixed-action-btn",>
                        <a class=("btn-floating", "btn-large", "waves-effect", "waves-light", "notes-trigger"), href="#add-modal",>
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
                                        <input placeholder="title", id="title_input", type="text", value=&self.new_title, />
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
                        <button class=("modal-action", "modal-close", "waves-effect", "waves-green", "btn-flat"), onclick=self.link.callback(|_| Msg::AddNote)>{ "Add" }</button>
                        <button class=("modal-action", "modal-close", "btn-flat"), onclick=self.link.callback(|_| Msg::CancelAdd)>{ "Cancel" }</button>
                    </div>
                </div>
            </>
        }
    }
}

impl Viewable for Note {
    fn view(&self) -> Html {
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

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_as_body();
}
