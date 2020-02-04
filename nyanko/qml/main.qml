import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.10 as Kirigami

Kirigami.ApplicationWindow {
    id: root
    pageStack.initialPage: search

    Component {
        id: search
        Search {}
    }
}
