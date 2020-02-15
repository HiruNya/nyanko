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
        page.anime_title = selected.anime_title
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

    StackLayout {
        id: stack
        // A blank page used at the start
        Item {
        }
        // If we can't find any search results
        Label {
            text: "No results were found"
        }
        // Displays search results
        AniGrid {
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

    Component.onCompleted: {
        core.search_results.modelReset.connect(() => {
            stack.currentIndex = (applicationWindow().core.search_results.rowCount() === 0)? 1: 2
        })
    }
}
