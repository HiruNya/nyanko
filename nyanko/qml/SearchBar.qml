import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.10 as Kirigami

Kirigami.SearchField {
    id: root
    placeholderText: qsTr("Search here...")

    rightActions: [
        Kirigami.Action {
            text: "Search"
            iconName: "search"
            onTriggered: root.accepted()
        }
    ]
}
