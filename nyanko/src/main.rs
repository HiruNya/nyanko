#[macro_use]
extern crate qmetaobject;

use qmetaobject::QmlEngine;

qrc!(declare_resources,
	"qml" {
		"./qml/AniGrid.qml" as "AniGrid.qml",
		"./qml/main.qml" as "main.qml",
		"./qml/Search.qml" as "Search.qml",
		"./qml/SearchBar.qml" as "SearchBar.qml",
	}
);

fn main() {
	let mut engine = QmlEngine::new();
	declare_resources();
	engine.load_file("qrc:/qml/main.qml".into());
	engine.exec()
}
