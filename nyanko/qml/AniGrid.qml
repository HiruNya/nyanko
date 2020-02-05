import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.7 as Kirigami

Kirigami.CardsGridView {
    delegate: Kirigami.Card {
        banner.title: title
        banner.source: image

//        contentItem: Label {
//            elide: Text.ElideRight
//            maximumLineCount: 3
//            text: "Taking off right after the last episode of the first season. The school is temporarily closed due to security. When U.A. restarts, it is announced that the highly anticipated School Sports Festival will soon be taking place. All classes: Hero, Support, General and Business will be participating. Tournaments all round will decide who is the top Hero in training."
//        }

        /// Original Size
//		banner.sourceSize.width: 400
//		banner.sourceSize.height: 567
        /// Kirigami HIG recommends banner images to have aspect ratio of 16x9. 4x3, or 1x1
        /// This works with content but not without
//        banner.sourceSize.width: 400
//        banner.sourceSize.height: 400*9/16
        banner.sourceSize.height: 260
        banner.sourceSize.width: 400
        banner.fillMode: Image.PreserveAspectCrop

        actions: [
            Kirigami.Action {
                text: qsTr("Watching")
            },
            Kirigami.Action {
                text: qsTr("Plan To Watch")
                iconName: "clock"
            }
        ]
        hiddenActions: [
            Kirigami.Action {
                text: qsTr("Dropped")
            }
        ]
    }
}
