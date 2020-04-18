import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.10 as Kirigami

Kirigami.ScrollablePage {
    id: root
    title: qsTr("Search")

    property var selected: QtObject {
        property bool active
        property string anime_title
        property int anime_id
        property string description
        property url image
        onActiveChanged: {
            if (selected.active) {
                push_anime_page()
            } else if (applicationWindow().pageStack.currentItem === page) {
                applicationWindow().pageStack.pop()
            }
        }
    }

    property AniPage page: applicationWindow().pagePool.loadPage("AniPage.qml")

    function push_anime_page() {
        page.selected = root.selected
        applicationWindow().pageStack.push(page)
    }

    header: ToolBar {
        SearchBar {
            id: search
            anchors.fill: parent
            property string old_search
            onAccepted: {
                if ((search.text.trim() !== "") && (old_search !== search.text)) {
                    applicationWindow().core.search(search.text)
                    old_search = search.text
                }
            }
        }
    }

    // Displays search results
    AniGrid {
        id: grid
        model: applicationWindow().core.search_results
        selected: root.selected
    }

    onIsCurrentPageChanged: () => {
        if (isCurrentPage && selected.active) {
            push_anime_page()
        }
    }
}
