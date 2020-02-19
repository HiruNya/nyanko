import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.CardsGridView {
    id: grid
    property var selected

    Kirigami.Card {
        id: card
        banner.title: title
        banner.source: image
        banner.cache: false
        banner.sourceSize.height: 260
        banner.sourceSize.width: 400
        /// Original Size
//		banner.sourceSize.width: 400
//		banner.sourceSize.height: 567
        /// Kirigami HIG recommends banner images to have aspect ratio of 16x9. 4x3, or 1x1
        /// This works with content but not without
//        banner.sourceSize.width: 400
//        banner.sourceSize.height: 400*9/16

        actions: [
            Kirigami.Action {
                text: qsTr("Watching")
                iconName: "quickview"
                displayHint: Kirigami.Action.DisplayHint.IconOnly
            },
            Kirigami.Action {
                text: qsTr("Plan To Watch")
                iconName: "clock"
                displayHint: Kirigami.Action.DisplayHint.IconOnly
            },
            Kirigami.Action {
                text: qsTr("Completed")
                iconName: "dialog-ok"
                displayHint: Kirigami.Action.DisplayHint.IconOnly
            }
        ]
        hiddenActions: [
            Kirigami.Action {
                text: qsTr("Dropped")
            }
        ]

        // Make the header, *not* the buttons, clickable to get more information
        MouseArea {
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            height: banner.height
            onClicked: {
                selected = { anime_title: title, image: image, anime_id: anime_id, description: description }
            }
        }
    }
}
