use sycamore::{
    component,
    prelude::*,
    reactive::{ReadSignal, Scope},
    view,
    view::View,
    web::Html,
};

use crate::{
    models::notebook_model::NotebookModel, services::notebook_service::get_notebook_by_id,
};

#[derive(Prop)]
pub struct NotebookProps {
    pub notebook_id: String,
}

#[component]
pub fn Notebook<G: Html>(cx: Scope, props: NotebookProps) -> View<G> {
    let notebook = get_notebook_by_id(props.notebook_id);

    let notesSignal = create_signal(cx, notebook.notes);

    view! {cx,
        p { ("Notebook: ".to_owned() + &(notebook.name)) }
        ul (class="notebooks border rounded-xl border-solid p-8 w-48 mx-auto") {
            Keyed(
                iterable= notesSignal,
                view= |cx, x| view! {
                    cx,
                    li {
                        a(href=("notebook/".to_owned() + &x.note_id)) {(x.content) }
                    }
                },
                key= |x| String::from(&x.note_id),
            )
        }
    }
}
