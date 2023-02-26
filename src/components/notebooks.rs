use sycamore::{
    component,
    prelude::*,
    reactive::{ReadSignal, Scope},
    view,
    view::View,
    web::Html,
};
use sycamore_futures::spawn_local_scoped;

// use sycamore::

use crate::{models::notebook_model::NotebookModel, services::notebook_service::get_all_notebooks};

#[derive(Prop)]
pub struct NotebooksProps {}

#[component]
pub fn Notebooks<G: Html>(cx: Scope, props: NotebooksProps) -> View<G> {
    let notebooks_signal = create_signal(cx, Vec::<NotebookModel>::from([]));
    spawn_local_scoped(cx, async {
        let notebooks = get_all_notebooks().await;
        notebooks_signal.set(notebooks);
    });

    // notebooks.set(get_all_notebooks(notebooks));
    view! {cx,
        p { ("Notebooks") }
        ul (class="notebooks border rounded-xl border-solid p-8 w-48 mx-auto") {
            Keyed(
                iterable= notebooks_signal,
                view= |cx, x| view! {
                    cx,
                    li {
                        a(href=("notebook/".to_owned() + &x.notebook_id)) {(x.name) }
                    }
                },
                key= |x| String::from(&x.name),
            )
        }
    }
}
