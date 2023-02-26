use std::future::Future;
use sycamore::reactive::Signal;
use wasm_bindgen_futures::spawn_local;

use crate::js_bindings::notebook_service;
use crate::models::{note_model::NoteModel, notebook_model::NotebookModel};

pub async fn get_all_notebooks(// notebooks_signal: &Signal<Vec<NotebookModel>>,
) -> Vec<NotebookModel> {
    let notebooks = serde_wasm_bindgen::from_value::<Vec<NotebookModel>>(
        notebook_service::getAllNotebooks().await,
    )
    .unwrap();
    notebooks

    // let mut notebooks_outer: Vec<NotebookModel> = vec![];

    // let notebooks_signal_clone = notebooks_signal.clone();
    // spawn_local({
    //     async move {
    //         let notebooks = serde_wasm_bindgen::from_value::<Vec<NotebookModel>>(
    //             notebook_service::getAllNotebooks().await,
    //         )
    //         .unwrap();

    //         // notebooks_outer = notebooks;

    //         // notebook_service::log(&format!("temp {}", notebooks.len()));

    //         notebooks_signal_clone.set(notebooks);
    //     }
    //     // notebooks
    // });

    // notebook_service::log(&format!("temp {}", notebooks_outer.len()));

    // let notebooks = notebooks_future.poll_unpin(cx)
    // vec![NotebookModel {
    //     notebook_id: String::from("one"),
    //     name: String::from("One"),
    //     notes: vec![NoteModel {
    //         note_id: String::from("one"),
    //         content: String::from("Content"),
    //         tags: vec![String::from("tagone")],
    //     }],
    // }]
    // notebook_service::log(&format!("\n\n\n notebooks from js {:?}", notebooks.len()));

    // let temp = serde_wasm_bindgen::from_value::<String>(notebook_service::getTemp()).unwrap();

    // notebook_service::log(&format!("temp{}!", temp));
    // notebooks
}

pub fn get_notebook_by_id(_: String) -> NotebookModel {
    NotebookModel {
        notebook_id: String::from("one"),
        name: String::from("One"),
        notes: vec![NoteModel {
            note_id: String::from("one"),
            content: String::from("Content"),
            tags: vec![String::from("tagone")],
        }],
    }
}
