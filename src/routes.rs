use crate::components::{self, notebook::NotebookProps, notebooks::NotebooksProps};
use sycamore::{
    component,
    reactive::{ReadSignal, Scope},
    view,
    view::View,
    web::Html,
};
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

#[derive(Route)]
enum AppRoutes {
    #[to("/notebooks")]
    Notebooks,
    #[to("/notebook/<notebook_id>")]
    Notebook { notebook_id: String },
    #[to("/notebook/<notebook_id>/note/<note_id>")]
    Note {
        notebook_id: String,
        note_id: String,
    },
    #[not_found]
    NotFound,
}

#[component]
pub fn Routes<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! {cx,
                    div(class="app") {
                        (match route.get().as_ref() {
                            AppRoutes::Notebooks => view! { cx,
                                components::notebooks::Notebooks(NotebooksProps{})
                            },
                            AppRoutes::Notebook{notebook_id} => view! { cx,
                                components::notebook::Notebook(NotebookProps{
                                   notebook_id: String::from(notebook_id)
                                })
                            },
                            AppRoutes::Note{notebook_id, note_id} => view! { cx,
                                "This is a note"
                            },
                            AppRoutes::NotFound => view! { cx,
                                "404 Not Found"
                            },
                        })
                    }
                }
            }
        )
    }
}
