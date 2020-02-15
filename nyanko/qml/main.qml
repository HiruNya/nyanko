import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.11 as Kirigami

import core 1.0

Kirigami.ApplicationWindow {
    id: window
    property Kirigami.PagePool pagePool: pagePool_
    property Nyanko core: nyanko
    pageStack.initialPage: pagePool.loadPage("WelcomePage.qml")
    globalDrawer: Kirigami.GlobalDrawer {
        actions: [
            Kirigami.PagePoolAction {
                text: "Welcome"
                page: "WelcomePage.qml"
                pagePool: window.pagePool
            },
            Kirigami.Action {
                text: "Search"
                iconName: "search"
                onTriggered: {
                    let page = window.pagePool.loadPage("Search.qml")
                    window.pageStack.clear()
                    window.pageStack.push(page)
                    if (page.selected) {
                        page.push_anime_page()
                    }
                }
            }
        ]
    }

    Nyanko {
        id: nyanko
    }

    Kirigami.PagePool {
        id: pagePool_
    }
}
