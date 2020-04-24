import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtLocation 5.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.ScrollablePage {
    id: accounts
    title: "Accounts"
    property int selected: window.core.current_account
    property var account_list: window.core.accounts
    actions {
        main: Kirigami.PagePoolAction {
            text: "Add"
            iconName: "list-add-user"
            pagePool: window.pagePool
            page: "login/AniListLogin.qml"
        }
    }

    Dialog {
        id: warn_delete
        title: "Warning"
        modal: true
        standardButtons: Dialog.Ok | Dialog.Cancel
        anchors.centerIn: parent

        Label {
            anchors.fill: parent
            text: "Are you sure you want to delete this account?"
        }

        onAccepted: () => console.log('Deleted')
    }

    Kirigami.CardsListView {
        anchors.fill: parent
        model: accounts.account_list
        delegate: Kirigami.AbstractCard {
            height: row.height

            MouseArea {
                anchors.fill: parent
                onClicked: { window.core.current_account = index }
            }

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
                Image {
                    width: 100
                    height: 100
                    source: avatar
                    fillMode: Image.PreserveAspectCrop
                }
                Column {
                    spacing: 5
                    padding: 10
                    Label {
                        text: name
                        font.pointSize: 14
                    }
                    Label {
                        text: account_type
                    }
                }
            }
            Column {
                anchors.top: parent.top
                anchors.right: parent.right
                Button {
                    icon.name: "edit-symbolic"
                    onClicked: () => {
                        window.pageStack.push(window.pagePool.loadPage("AccountPage.qml"), {avatar, name, type: account_type})
                    }
                }
                Button {
                    icon.name: "delete"
                    onClicked: warn_delete.open()
                }
            }
        }
    }
}
