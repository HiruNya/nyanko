import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.Page {
    id: page
    property string anime_title
    property url image
    title: anime_title

    Image {
        anchors.fill: parent
        source: image
    }
}
