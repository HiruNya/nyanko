import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.Page {
    id: page
    property string link: window.core.anilist_login_link()
    title: qsTr("Log in to AniList")
    function login () {
        if (window.core.anilist_login(code_field.text)) {
            window.pageStack.replace(window.pagePool.loadPage("AccountPage.qml"))
        } else {
            console.log("Error Logging In")
        }
    }

    Column {
        anchors.fill: parent
        Text {
            width: parent.width
            text: qsTr("Click the button below and a webpage will be opened in your default browser to log into AniList. "
                       + "Once you login, you will be given a code that you should copy and paste into the text box below.")
            color: Kirigami.Theme.textColor
            font.pointSize: 14
            wrapMode: Text.Wrap
        }
        Kirigami.UrlButton {
            width: parent.width
            url: link
            font.pointSize: 12
            topPadding: 10
            bottomPadding: 30
            wrapMode: Text.Wrap
            horizontalAlignment: Text.AlignHCenter
        }
        Button {
            text: qsTr("Log In")
            onClicked: Qt.openUrlExternally(link)
            anchors.horizontalCenter: parent.horizontalCenter
        }
        Label {
            width: parent.width
            text: qsTr("Enter the code below")
            font.pointSize: 14
            topPadding: 30
            bottomPadding: 10
            wrapMode: Text.Wrap
            horizontalAlignment: Text.AlignHCenter
        }
        Kirigami.ActionTextField {
            id: code_field
            width: parent.width * 3 / 4
            anchors.horizontalCenter: parent.horizontalCenter
            rightActions: [
                Kirigami.Action {
                    text: qsTr("Enter Code")
                    iconName: "dialog-ok"
                    onTriggered: page.login()
                },
                Kirigami.Action {
                    text: qsTr("Paste from Clipboard")
                    iconName: "edit-paste"
                    onTriggered: code_field.paste()
                }
            ]
            onAccepted: page.login()
        }
    }
}
