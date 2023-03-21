var db = new PouchDB('default');

// db.post({
//   name: "One",
//   doc_type: "notebook"
// }).then(function (response) {
//   console.log("notebook created", response);
//   db.post({
//     content: "Content",
//     tags: ["tagone"],
//     doc_type: "note",
//     notebook_id: response.id
//   }).then(function (response) {
//     console.log("notebook created")
//   }).catch(function (err) {
//     console.log("error while creating notebook", err);
//   });
// }).catch(function (err) {
//   console.log("error while creating notebook", err);
// });


// db.allDocs({
//   include_docs: true
// }).then((docs) => {
//   console.log("docs", docs);
// });

// db.put({
//   content: "Content",
//   tags: ["tagone"],
//   doc_type: "notebook",
//   notebook_id: ""
// });

var ddoc = {
  _id: '_design/notes_and_notebooks',
  views: {
    all: {
      map: function (doc) {
        doc.doc_type === "notebook" || doc.doc_type === 'note' ? emit(doc._id, doc) : undefined;
      }.toString(),
      reduce: function (keys, values, rereduce) {
        if (rereduce) {
          return values;
        } else {
          const next = {
          };
          values.forEach((item) => {
            if (item.doc_type === "notebook") {
              let notebook = next[item._id] || {};
              notebook = {
                ...notebook,
                ...item,
                notebook_id: item._id,
                notes: []
              };
              next[item._id] = notebook;
            } else if (item.doc_type === "note") {
              const notebook = next[item.notebook_id] || {};
              const notes = notebook.notes || [];
              notes.push({
                ...item,
                note_id: item._id
              });
              notebook.notes = notes;

              next[item.notebook_id] = notebook;
            }
          });
          return next;
        }
      }.toString(),
    }
  }
};

// db.put(ddoc).then(function () {
//   console.log("notebooks index created");
// }).catch(function (err) {
//   console.log("notebooks index could not be created, it may already exist", err);
// });

db.get("_design/notes_and_notebooks").then(res => {
  console.log("res", res)
  db.put({
    ...res,
    ...ddoc
  }).then(function () {
    console.log("notebooks index updated");
  }).catch(function (err) {
    console.log("notebooks index could not be created, it may already exist", err);
  });
});

// db.remove("_design/notebooks");

// db.query('notes_and_notebooks/all', {
//   // include_docs: true
// }).then(function (res) {
//   console.log("notebooks from view", res);
// }).catch(function (err) {
//   console.log("error while fetching notebooks from view", err);
// });

db.on('error', function (err) { console.log("error", err); });

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

  // return new Promise((resolve, reject) => {
  //   // resolve([
  //   //   {
  //   //     notebook_id: "one",
  //   //     name: "One",
  //   //     notes: [{
  //   //       note_id: "one",
  //   //       content: "Content",
  //   //       tags: ["tagone"],
  //   //     }],
  //   //   }
  //   // ])

  // });

  return db.query('notes_and_notebooks/all', {
    // include_docs: true
  }).then(function (res) {
    console.log("notebooks from view", res);
    if (res && res.rows) {
      const reducedValue = res.rows.flatMap(row => {
        return Object.values(row.value);
      });
      console.log("reducedValue", reducedValue);
      return reducedValue;
    }
    return [];
  }).catch(function (err) {
    console.log("error while fetching notebooks from view", err);
  });

}

export const getTemp = () => {
  return "temp";
}