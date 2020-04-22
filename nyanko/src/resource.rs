
qrc!(declare_resources_,
	"qml" {
		"./qml/AccountPage.qml" as "AccountPage.qml",
		"./qml/Accounts.qml" as "Accounts.qml",
		"./qml/AniGrid.qml" as "AniGrid.qml",
		"./qml/AniPage.qml" as "AniPage.qml",
		"./qml/login/AniListLogin.qml" as "login/AniListLogin.qml",
		"./qml/main.qml" as "main.qml",
		"./qml/Search.qml" as "Search.qml",
		"./qml/SearchBar.qml" as "SearchBar.qml",
		"./qml/WelcomePage.qml" as "WelcomePage.qml",
	}
);

pub fn declare_resources() {
	declare_resources_()
}
