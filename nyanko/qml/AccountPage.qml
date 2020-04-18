import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.ScrollablePage {
    id: accounts
    title: "Accounts"
    property int selected: 0
    property var account_list: ListModel {
        ListElement {
            type: "AniList"
            user: "HiruNya"
        }
        ListElement {
            type: "Local Storage"
            user: "My Local List"
        }
        ListElement {
            type: "AniList"
            user: "OtherAccount"
        }
    }

    actions {
        main: Kirigami.PagePoolAction {
            text: "Add"
            iconName: "list-add-user"
            pagePool: window.pagePool
            page: "login/AniListLogin.qml"
        }
    }

    Kirigami.CardsListView {
        anchors.fill: parent
        model: accounts.account_list
        delegate: Kirigami.AbstractCard {
            height: row.height

            Loader {
                anchors.fill: parent
                active: (index == accounts.selected)
                sourceComponent: Rectangle {
                    id: highlight
                    color: Kirigami.Theme.highlightColor
                    width: parent.width
                    height: row.height
                }
            }

            Row {
                id: row
                spacing: 5
                Rectangle {
                    width: 100
                    height: 100
                }
                Column {
                    spacing: 5
                    padding: 10
                    Label {
                        text: user
                        font.pointSize: 14
                    }
                    Label {
                        text: type
                    }
                }
            }
            MouseArea {
                anchors.fill: parent
                onClicked: { accounts.selected = index }
            }
        }
    }
}
