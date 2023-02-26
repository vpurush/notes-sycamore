export const getAllNotebooks = () => {
    // return [
    //     {
    //         notebook_id: "one",
    //         name: "One",
    //         notes: [{
    //             note_id: "one",
    //             content: "Content",
    //             tags: ["tagone"],
    //         }],
    //     }
    // ];

    return new Promise((resolve, reject) => {
        resolve([
            {
                notebook_id: "one",
                name: "One",
                notes: [{
                    note_id: "one",
                    content: "Content",
                    tags: ["tagone"],
                }],
            }
        ])
    });
}

export const getTemp = () => {
    return "temp";
}