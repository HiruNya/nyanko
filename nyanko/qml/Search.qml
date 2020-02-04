import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.10 as Kirigami

Kirigami.Page {
    id: root
    title: "Search"

    header: ToolBar{
        SearchBar{
            id: search
            anchors.fill: parent
        }
    }

    AniGrid {
        anchors.fill: parent
    }
}
