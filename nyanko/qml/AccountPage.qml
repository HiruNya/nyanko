import QtQuick 2.14
import org.kde.kirigami 2.11 as Kirigami

Kirigami.Page {
    title: name
    property string avatar;
    property string name;
    property string type;

    Image {
        source: avatar
    }
}
