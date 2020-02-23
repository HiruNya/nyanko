import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.ScrollablePage {
    id: page
    property var selected
    title: (selected)? selected.anime_title: "???"

    actions {
        left: Kirigami.Action {
            text: qsTr("Watching")
            iconName: "quickview"
            displayHint: Kirigami.Action.DisplayHint.IconOnly
        }
        main: Kirigami.Action {
            text: qsTr("Completed")
            iconName: "dialog-ok"
            displayHint: Kirigami.Action.DisplayHint.IconOnly
            onTriggered: { selected.active = false }
        }
        right: Kirigami.Action {
            text: qsTr("Plan To Watch")
            iconName: "clock"
            displayHint: Kirigami.Action.DisplayHint.IconOnly
        }
    }

    GridLayout {
        id: layout
        property real max_row_width: Math.max(2*cover_image.width, cover_image.width+title_label.width)
        property bool is_row: (width >= max_row_width)
        rows: (is_row)? 1: 2
        columns: (!is_row)? 1: 2
        rowSpacing: 5

        Image {
            id: cover_image
            source: (page.selected)? page.selected.image: ""
            Layout.alignment: (layout.is_row)? Qt.AlignTop: Qt.AlignHCenter
            Layout.preferredWidth: sourceSize.width
        }

        Column {
            Layout.alignment: Qt.AlignTop
            Layout.fillWidth: true
            width: description_text.width
            spacing: 15
            Label {
                id: title_label
                text: (page.selected)? page.selected.anime_title: "???"
                fontSizeMode: Text.HorizontalFit
                wrapMode: Text.Wrap
                font.underline: true
                font.weight: Font.Bold
            }
            Text {
                id: description_text
                text: (page.selected)? page.selected.description: "???"
                wrapMode: Text.Wrap
                color: Kirigami.Theme.textColor
                width: if (layout.is_row) {layout.parent.width - cover_image.width} else {layout.parent.width}
                textFormat: Text.RichText
            }
        }
    }
}
