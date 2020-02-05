import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.10 as Kirigami

import core 1.0

Kirigami.Page {
    id: root
    title: qsTr("Search")

    property var core

    header: ToolBar{
        SearchBar{
            id: search
            anchors.fill: parent
            onAccepted: {
                if (search.text != "") {
                    core.search(search.text)
                }
            }
        }
    }

    AniGrid {
        anchors.fill: parent
        model: core.search_results
    }
}
