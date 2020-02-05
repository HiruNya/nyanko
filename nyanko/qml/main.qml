import QtQuick 2.14
import QtQuick.Controls 2.14
import org.kde.kirigami 2.10 as Kirigami

import core 1.0

Kirigami.ApplicationWindow {
    id: root
    pageStack.initialPage: search

    Component {
        id: search
        Search {
            core: nyanko
        }
    }

    Nyanko {
        id: nyanko
        // ToDo: Figure out why this is required for the results to update
        onSearchResultsChanged: console.log("Displaying Search Results")
    }
}
