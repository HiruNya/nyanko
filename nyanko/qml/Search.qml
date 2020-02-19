import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.10 as Kirigami

Kirigami.ScrollablePage {
    id: root
    title: qsTr("Search")
    property var selected

    function push_anime_page() {
        let page = applicationWindow().pagePool.loadPage("AniPage.qml")
        applicationWindow().pageStack.push(page)
        page.anime_id = selected.anime_id
        page.anime_title = selected.anime_title
        page.description = selected.description
        page.image = selected.image
    }

    header: ToolBar {
        SearchBar {
            id: search
            anchors.fill: parent
            onAccepted: {
                if (search.text !== "") {
                    applicationWindow().core.search(search.text)
                }
            }
        }
    }

    // Displays search results
    AniGrid {
        id: grid
        model: applicationWindow().core.search_results
        selected: root.selected
        onSelectedChanged: {
            if (selected) {
                push_anime_page()
            } else {
                applicationWindow().pageStack.pop()
            }
        }
    }
}
