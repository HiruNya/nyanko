import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.11 as Kirigami

import core 1.0

Kirigami.ApplicationWindow {
    id: window
    title: "Nyanko"
    property Kirigami.PagePool pagePool: pagePool_
    property Nyanko core: nyanko
    pageStack.initialPage: pagePool.loadPage("WelcomePage.qml")
    globalDrawer: Kirigami.GlobalDrawer {
        modal: false
        collapsible: true
        collapsed: true
        actions: [
            Kirigami.PagePoolAction {
                text: "Welcome"
                page: "WelcomePage.qml"
                pagePool: window.pagePool
            },
            Kirigami.PagePoolAction {
                text: "Search"
                iconName: "search"
                page: "Search.qml"
                pagePool: window.pagePool
            },
            Kirigami.PagePoolAction {
                text: "Accounts"
                iconName: "user"
                page: "Accounts.qml"
                pagePool: window.pagePool
            }

        ]
    }

    Nyanko {
        id: nyanko
        Component.onCompleted: nyanko.init()
    }

    Kirigami.PagePool {
        id: pagePool_
    }
}
